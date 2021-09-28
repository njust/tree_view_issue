use gtk4::{
    self,
    prelude::*,
    glib,
    gio,
    Orientation,
    TreeListRow,
    TreeExpanderBuilder,
};

use crate::model::{
    PersonObject
};


pub fn get_persons(n: i32) -> Vec<glib::Object>  {
    (0..n).map(|i| {
        PersonObject::new(format!("test: {}", i))
    }).collect()
}

pub fn tree() -> gtk4::Box {
    let item_factory = gtk4::SignalListItemFactory::new();
    item_factory.connect_bind(move |_, list_item| {
        if let Some((row, child_obj)) = list_item.item()
            .and_then(|obj| obj.downcast::<TreeListRow>().ok())
            .and_then(|row| row.item().map(|child_obj| (row, child_obj)))
        {
            let lbl = gtk4::Label::new(None);
            child_obj.bind_property("name", &lbl, "label")
                .flags(glib::BindingFlags::DEFAULT |glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL).build();

            let expander = TreeExpanderBuilder::new()
                .child(&lbl)
                .list_row(&row)
                .build();

            list_item.set_child(Some(&expander));
        }
    });

    let list_store = gio::ListStore::new(PersonObject::static_type());
    let persons = get_persons(10);
    for person in persons {
        list_store.append(&person);
    }

    let model = list_store.upcast_ref::<gio::ListModel>();
    let tree_list_model = gtk4::TreeListModel::new(model, false, false, |obj| {
        if let Some(name) = obj.property("name").ok().and_then(|p|p.get::<String>().ok()) {
            println!("Expand: {}", name);
        }

        let list_store = gio::ListStore::new(PersonObject::static_type());
        let persons = get_persons(5);
        for person in persons {
            list_store.append(&person);
        }
        Some(list_store.upcast::<gio::ListModel>())
    });

    let selection_model = gtk4::SingleSelection::new(Some(&tree_list_model));
    let list_view = gtk4::ListViewBuilder::new()
        .factory(&item_factory)
        .model(&selection_model)
        .build();

    let container = gtk4::Box::new(Orientation::Vertical, 0);
    let sw = gtk4::ScrolledWindow::new();

    sw.set_vexpand(true);
    sw.set_child(Some(&list_view));
    container.append(&sw);
    container
}

