#![allow(unreachable_code)]
use std::cell::RefCell;
use std::sync::{Arc, Mutex, OnceLock};

use gtk::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};

use crate::database::database::Database;
use crate::tools::ToolType;
use crate::ui::object::db_label_object::DBLabelObject;
use crate::ui::object::tree_tool_row::TreeToolRow;

// Object holding the state
#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_tree_tools.ui")]
#[properties(wrapper_type = super::FrameTreeTools)]
pub struct FrameTreeTools {
    #[template_child]
    pub tool_list: TemplateChild<gtk::ListView>,

    root_rows: RefCell<Vec<TreeToolRow>>,
    model_selection: gtk::SingleSelection,
    pub selected_tree: RefCell<Option<gtk::TreeExpander>>,
}

impl FrameTreeTools {
    fn tree_model_callback(
        obj: &glib::Object,
        mutex_db: &Mutex<Database>,
    ) -> Option<gio::ListModel> {
        let tree_tool: &TreeToolRow = obj
            .downcast_ref()
            .expect("[create_tree_model] tool need to be TreeToolRow");

        if tree_tool.is_tool() {
            return None;
        }

        let child_model = gio::ListStore::new::<TreeToolRow>();
        let db = mutex_db.lock().unwrap();

        match tree_tool.get_tool_type() {
            ToolType::Drill => {
                // TODO Avoid this unwrap
                for drill in db.get_all_drills().unwrap() {
                    child_model.append(&TreeToolRow::new_drill_tool(
                        drill.base_tool.name,
                        drill.base_tool.id,
                    ));
                }
            }
            ToolType::Endmill => {
                // TODO Avoid this unwrap
                for endmill in db.get_all_endmills().unwrap() {
                    child_model.append(&TreeToolRow::new_endmill_tool(
                        endmill.base_tool.name,
                        endmill.base_tool.id,
                    ));
                }
            }
            ToolType::VBit => {
                // TODO Avoid this unwrap
                for vbit in db.get_all_vbits().unwrap() {
                    child_model.append(&TreeToolRow::new_vbit_tool(
                        vbit.base_tool.name,
                        vbit.base_tool.id,
                    ));
                }
            }
        };

        Some(child_model.into())
    }

    fn factory_setup(_factory: &gtk::SignalListItemFactory, list_item: &glib::Object) {
        let widget = DBLabelObject::new();
        let tree_expander = gtk::TreeExpander::new();

        tree_expander.set_child(Some(&widget));

        tree_expander.connect_notify(Some("has-focus"), |tree, _b| {
            if tree.has_focus() && tree.is_focusable() {
                let label: DBLabelObject = tree
                    .child()
                    .and_downcast()
                    .expect("[connect_notify] child need to be DBLabelObject");

                match tree.ancestor(FrameTreeTools::type_()) {
                    Some(widget) => {
                        widget.emit_by_name::<()>(
                            "row_selected",
                            &[&label.db_id(), &label.tool_type().unwrap()],
                        );
                        widget
                            .downcast_ref::<super::FrameTreeTools>()
                            .unwrap()
                            .imp()
                            .selected_tree
                            .set(Some(tree.clone()));
                    }
                    None => (),
                };
            }
        });

        list_item
            .downcast_ref::<gtk::ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&tree_expander));
    }

    fn factory_bind(_factory: &gtk::SignalListItemFactory, list_item: &glib::Object) {
        let list_item: &gtk::ListItem = list_item.downcast_ref().expect("Need to be ListItem");

        let tree_expander: gtk::TreeExpander = list_item
            .child()
            .and_downcast()
            .expect("The child need to be a TreeExpander");

        let list_row: gtk::TreeListRow = list_item
            .item()
            .and_downcast()
            .expect("The item need to be a TreeListRow");

        let tree_expander_child = tree_expander
            .child()
            .and_downcast::<DBLabelObject>()
            .expect("Tree expander's child need to be DBLabel");

        let tree_tool_item: TreeToolRow = list_row
            .item()
            .and_downcast()
            .expect("[factory_bind] tree_tool_item need to be TreeToolRow");

        tree_expander.set_list_row(Some(&list_row));
        tree_expander_child.set_label(&tree_tool_item.name());
        tree_expander_child.set_tool_type(Some(tree_tool_item.get_tool_type()));
        match tree_tool_item.get_tool_id() {
            Ok(id) => tree_expander_child.set_db_id(id),
            Err(_) => {}
        };

        if tree_tool_item.is_category() {
            tree_expander_child.add_css_class("label_tool_category");
            tree_expander.set_focusable(false);
            list_item.set_selectable(false);
            list_item.set_activatable(false);
        }
    }

    fn generate_tree_model(&self, database: Arc<Mutex<Database>>) -> gtk::TreeListModel {
        let model = gio::ListStore::new::<TreeToolRow>();
        model.extend_from_slice(self.root_rows.borrow().as_slice());

        gtk::TreeListModel::new(model, false, true, move |obj| {
            Self::tree_model_callback(obj, &database)
        })
    }

    pub fn set_root_elements(&self, elems: Vec<TreeToolRow>, database: Arc<Mutex<Database>>) {
        self.root_rows.set(elems);
        self.refresh_tree(database);
    }

    pub fn refresh_tree(&self, database: Arc<Mutex<Database>>) {
        self.model_selection
            .set_model(Some(&self.generate_tree_model(database)));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameTreeTools {
    const NAME: &'static str = "FrameTreeTools";
    type Type = super::FrameTreeTools;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameTreeTools {
    fn constructed(&self) {
        self.parent_constructed();

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(FrameTreeTools::factory_setup);
        factory.connect_bind(FrameTreeTools::factory_bind);

        let tool_list: &gtk::ListView = self
            .tool_list
            .downcast_ref()
            .expect("Expect to be a ListView");

        tool_list.set_model(Some(&self.model_selection));
        tool_list.set_factory(Some(&factory));
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("row-selected")
                .param_types([u32::static_type(), ToolType::static_type()])
                .build()]
        })
    }
}

impl WidgetImpl for FrameTreeTools {}
impl BoxImpl for FrameTreeTools {}
