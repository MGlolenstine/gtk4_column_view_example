use generators::generate_list;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual, Cast, ListModelExt, ObjectExt},
    traits::GtkWindowExt,
    Application, ApplicationWindow,
};
use item_object::ItemObject;

pub mod generators;
pub mod item_object;

pub struct Item {
    id: u32,
    name: String,
}

impl Item {
    pub fn new(id: u32, name: String) -> Self {
        Item { id, name }
    }
}

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.column_view_example")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let mut items = vec![];
    items.push(Item::new(0, "First".to_string()));
    items.push(Item::new(1, "Second".to_string()));
    items.push(Item::new(2, "Third".to_string()));
    items.push(Item::new(3, "Fourth".to_string()));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK ColumnView example")
        .build();

    let column_view = gtk::ColumnView::default();

    column_view.set_single_click_activate(true);
    column_view.connect_activate(move |column_view, position| {
        let model = column_view.model().expect("The model has to exist!");
        let auto_row_object = model
            .item(position)
            .expect("The item has to exist.")
            .downcast::<ItemObject>()
            .expect("The item has to be an `ItemObject`.");

        let id: u32 = auto_row_object.property("id");
        let name: String = auto_row_object.property("name");
        println!("The entry with ID: {id} and Name: {name} has been activated!");
    });

    
    generate_list(&column_view, &items);
    println!("ColumnView: {}", column_view);
    window.set_child(Some(&column_view));

    window.present();
}
