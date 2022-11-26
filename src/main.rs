use gtk::prelude::*;
use gtk::Application;

mod gui;
mod markdown;

const APP_ID: &str = "io.github.maxstrid.MarkdownEditor";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(gui_main);

    app.run();
}

fn gui_main(app: &Application) {
    let mut gui = gui::Gui::new(app);

    gui.present();
}
