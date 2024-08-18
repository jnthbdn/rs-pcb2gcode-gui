#![allow(unreachable_code)]

use std::sync::Arc;

use gtk::{
    gio,
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    custom_object::tree_tool_object::TreeToolObject,
    database::database::Database,
    tools::ToolType,
    ui::custom_object::{db_label_object::DBLabelObject, tool_setting_object::ToolSettingObject},
};

#[derive(gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window_tool_db.ui")]
#[properties(wrapper_type=super::WindowToolDB)]
pub struct WindowToolDB {
    #[template_child]
    pub tool_list: TemplateChild<gtk::ListView>,

    model_selection: gtk::SingleSelection,

    pub database: Arc<Database>,
}

impl WindowToolDB {
    fn tree_model_callback(obj: &glib::Object, db: &Database) -> Option<gio::ListModel> {
        let tree_tool: &TreeToolObject = obj
            .downcast_ref()
            .expect("[create_tree_model] tool need to be TreeToolObject");

        if tree_tool.is_tool() {
            return None;
        }

        let child_model = gio::ListStore::new::<TreeToolObject>();

        match tree_tool.get_tool_type() {
            ToolType::Drill => {
                // TODO Avoid this unwrap
                for drill in db.get_all_drills().unwrap() {
                    child_model.append(&TreeToolObject::new_drill_tool(
                        drill.base_tool.name,
                        drill.base_tool.id,
                    ));
                }
            }
            ToolType::Endmill => {
                // TODO Avoid this unwrap
                for endmill in db.get_all_endmills().unwrap() {
                    child_model.append(&TreeToolObject::new_endmill_tool(
                        endmill.base_tool.name,
                        endmill.base_tool.id,
                    ));
                }
            }
            ToolType::VBit => {
                // TODO Avoid this unwrap
                for vbit in db.get_all_vbits().unwrap() {
                    child_model.append(&TreeToolObject::new_vbit_tool(
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
                println!(
                    "[{} - #{} - {:?}] HAS FOCUS",
                    tree.child()
                        .and_downcast::<DBLabelObject>()
                        .unwrap()
                        .label(),
                    tree.child()
                        .and_downcast::<DBLabelObject>()
                        .unwrap()
                        .db_id(),
                    tree.child()
                        .and_downcast::<DBLabelObject>()
                        .unwrap()
                        .tool_type()
                );
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

        let tree_tool_item: TreeToolObject = list_row
            .item()
            .and_downcast()
            .expect("[factory_bind] tree_tool_item need to be TreeToolObject");

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

    fn generate_tree_model(&self) -> gtk::TreeListModel {
        let vector: Vec<TreeToolObject> = vec![
            TreeToolObject::new_category("Drill".to_string(), ToolType::Drill),
            TreeToolObject::new_category("Endmill".to_string(), ToolType::Endmill),
            TreeToolObject::new_category("V bit".to_string(), ToolType::VBit),
        ];

        let model = gio::ListStore::new::<TreeToolObject>();
        model.extend_from_slice(&vector);

        let cb_db = self.database.clone();
        gtk::TreeListModel::new(model, false, true, move |obj| {
            Self::tree_model_callback(obj, &cb_db)
        })
    }

    pub fn refresh_model(&self) {
        self.model_selection
            .set_model(Some(&self.generate_tree_model()));
    }
}

impl Default for WindowToolDB {
    fn default() -> Self {
        Self {
            tool_list: Default::default(),
            model_selection: Default::default(),
            database: Arc::new(Database::new().expect("[default] Unable to create database")),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowToolDB {
    const NAME: &'static str = "WindowToolDB";
    type Type = super::WindowToolDB;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        ToolSettingObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowToolDB {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup_actions();

        self.model_selection
            .set_model(Some(&self.generate_tree_model()));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(WindowToolDB::factory_setup);
        factory.connect_bind(WindowToolDB::factory_bind);

        let tool_list: &gtk::ListView = self
            .tool_list
            .downcast_ref()
            .expect("Expect to be a ListView");

        tool_list.set_model(Some(&self.model_selection));
        tool_list.set_factory(Some(&factory));
    }
}
impl WidgetImpl for WindowToolDB {}
impl WindowImpl for WindowToolDB {}
impl ApplicationWindowImpl for WindowToolDB {}
