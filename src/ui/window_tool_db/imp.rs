#![allow(unreachable_code)]

use std::ops::Deref;

use gtk::{
    gio,
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    custom_object::tree_tool_object::{TreeToolObject, TreeToolType},
    ui::custom_object::tool_setting_object::ToolSettingObject,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window_tool_db.ui")]
#[properties(wrapper_type=super::WindowToolDB)]
pub struct WindowToolDB {
    #[template_child]
    pub tool_list: TemplateChild<gtk::ListView>,
}

impl WindowToolDB {
    fn create_tree_model(obj: &glib::Object) -> Option<gio::ListModel> {
        let tree_tool: &TreeToolObject = obj
            .downcast_ref()
            .expect("[create_tree_model] tool need to be TreeToolObject");

        if tree_tool.is_tool() {
            return None;
        }

        let child_model = gio::ListStore::new::<TreeToolObject>();

        for i in 1..10 {
            let name = format!("{} #{}", tree_tool.get_name(), i + 1);
            match tree_tool.get_tool_type() {
                TreeToolType::Drill => child_model.append(&TreeToolObject::new_drill_tool(name, i)),
                TreeToolType::Endmill => {
                    child_model.append(&TreeToolObject::new_endmill_tool(name, i))
                }
                TreeToolType::VBit => child_model.append(&TreeToolObject::new_vbit_tool(name, i)),
            };
        }

        Some(child_model.into())
    }

    fn factory_setup(_factory: &gtk::SignalListItemFactory, list_item: &glib::Object) {
        let widget = gtk::Label::new(None);
        let tree_expander = gtk::TreeExpander::new();

        tree_expander.set_child(Some(&widget));

        tree_expander.connect_notify(Some("has-focus"), |tree, _b| {
            if tree.has_focus() && tree.is_focusable() {
                println!(
                    "[{}] HAS FOCUS ! ({:?})",
                    tree.child().and_downcast::<gtk::Label>().unwrap().label(),
                    tree
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
            .and_downcast::<gtk::Label>()
            .expect("Tree expander's child need to be Label");

        let tree_tool_item: TreeToolObject = list_row
            .item()
            .and_downcast()
            .expect("[factory_bind] tree_tool_item need to be TreeToolObject");

        tree_expander.set_list_row(Some(&list_row));
        tree_expander_child.set_label(&tree_tool_item.name());

        if tree_tool_item.is_category() {
            tree_expander_child.add_css_class("label_tool_category");
            tree_expander.set_focusable(false);
            list_item.set_selectable(false);
            list_item.set_activatable(false);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowToolDB {
    const NAME: &'static str = "WindowToolDB";
    type Type = super::WindowToolDB;
    type ParentType = gtk::Window;

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

        let vector: Vec<TreeToolObject> = vec![
            TreeToolObject::new_category("Drill".to_string(), TreeToolType::Drill),
            TreeToolObject::new_category("V bit".to_string(), TreeToolType::VBit),
            TreeToolObject::new_category("Endmill".to_string(), TreeToolType::Endmill),
        ];

        let model = gio::ListStore::new::<TreeToolObject>();
        model.extend_from_slice(&vector);

        let tree_model =
            gtk::TreeListModel::new(model, false, true, WindowToolDB::create_tree_model);

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(WindowToolDB::factory_setup);
        factory.connect_bind(WindowToolDB::factory_bind);

        let tool_list: &gtk::ListView = self
            .tool_list
            .downcast_ref()
            .expect("Expect to be a ListView");

        tool_list.set_model(Some(&gtk::SingleSelection::new(Some(tree_model))));
        tool_list.set_factory(Some(&factory));
    }
}
impl WidgetImpl for WindowToolDB {}
impl WindowImpl for WindowToolDB {}
