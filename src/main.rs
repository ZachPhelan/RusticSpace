use std::time::Duration;
use std::thread;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Switch, Box, Orientation};
use gtk::Orientation::*;
use glib;
use glib::{MainContext, PRIORITY_DEFAULT, clone};

fn main() {
    let app = Application::builder()
        .application_id("com.github.ZachPhelan.RusticSpace")
        .build();

    app.connect_activate(build_ui);
    
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rustic Space")
        .build();

    let button = Button::builder()
        .label("test")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    window.set_child(Some(&button));

    window.present();
    
}
