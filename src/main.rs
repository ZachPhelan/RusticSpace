use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use pango;

fn main() {
    let app = Application::builder()
        .application_id("org.exmaple.RusticSpace")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(50)
            .default_height(400)
            .title("Rustic Space")
            .build();

        let button = Button::with_label("Click");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });

        win.add(&button);
        win.show_all();
    });
    
    app.run();
}

fn configure_app(app: &gtk::Application) {
    
}
