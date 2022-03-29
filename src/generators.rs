use gtk::{
    gio::ListStore,
    pango::EllipsizeMode,
    prelude::{Cast, ListModelExt, ObjectExt, StaticType},
    traits::WidgetExt,
    Align, ColumnView, ColumnViewColumn, CustomSorter,
};

use crate::{item_object::ItemObject, Item};

pub fn generate_list(view: &ColumnView, model: &[Item]) {
    let mut items = ListStore::new(ItemObject::static_type());
    generate_items(view, Some(model), &mut items, true, |data, list_store| {
        for row in data {
            let new_item = ItemObject::new(row.id, &row.name);
            list_store.append(&new_item);
        }
    });

    if view.columns().n_items() == 0 {
        let id = "ID".to_string();
        let name = "Name".to_string();
        let cols = vec![(id.as_str(), "id"), (name.as_str(), "name")];
        generate_columns::<ItemObject>(view, &cols, false);
    }
    if let Some(sorter) = view.sorter() {
        let model = view.model().unwrap();
        view.set_model(Some(&gtk::NoSelection::new(Some(
            &gtk::SortListModel::new(Some(&model), Some(&sorter)),
        ))));
    }
}

fn generate_items<T>(
    view: &ColumnView,
    data: Option<T>,
    store: &mut ListStore,
    readonly: bool,
    cb: fn(T, &mut ListStore) -> (),
) {
    if let Some(d) = data {
        cb(d, store);
        if readonly {
            let selection_model = gtk::NoSelection::new(Some(store));
            view.set_model(Some(&selection_model));
        } else {
            let selection_model = gtk::SingleSelection::new(Some(store));
            view.set_model(Some(&selection_model));
        }
    } else {
        let selection_model = gtk::NoSelection::new(Some(store));
        view.set_model(Some(&selection_model));
        for c in view.columns() {
            view.remove_column(c.downcast_ref::<ColumnViewColumn>().unwrap());
        }
    }
}

fn generate_columns<T>(view: &ColumnView, cols: &[(&str, &str)], sortable: bool)
where
    T: StaticType,
{
    view.set_reorderable(false);
    for col in view.columns() {
        view.remove_column(&col.downcast::<ColumnViewColumn>().unwrap());
    }
    for c in cols.iter().enumerate() {
        let column = ColumnViewColumn::new(Some(c.1 .0), Some(&generate_factory::<T>(c.1 .1)));
        if sortable {
            let property_name = (c.1).1.to_string();
            let sorter = CustomSorter::new(move |a, b| {
                let a_s: String = a.property(&property_name);
                let b_s: String = b.property(&property_name);
                a_s.cmp(&b_s).into()
            });
            column.set_sorter(Some(&sorter));
        }
        column.set_resizable(true);
        if c.0 == cols.len() - 1 {
            column.set_expand(true);
        }
        view.insert_column(c.0 as u32, &column);
    }
}

fn generate_factory<T: StaticType>(property_name: &str) -> gtk::SignalListItemFactory {
    let property_name = property_name.to_string();
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let item = gtk::Label::new(None);
        item.set_use_markup(true);
        item.set_ellipsize(EllipsizeMode::Middle);
        item.set_halign(Align::Start);
        list_item.set_child(Some(&item));
        let list_item_expression = gtk::ConstantExpression::new(list_item);
        let integer_object_expression = gtk::PropertyExpression::new(
            gtk::ListItem::static_type(),
            Some(&list_item_expression),
            "item",
        );
        let number_expression = gtk::PropertyExpression::new(
            <T>::static_type(),
            Some(&integer_object_expression),
            &property_name,
        );
        number_expression.bind(&item, "label", Some(&item));
    });
    factory
}
