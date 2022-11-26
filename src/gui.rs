use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub struct Gui<'a> {
    app: &'a Application,
    filename: Option<String>,
    file_data: Option<String>,
}

impl<'a> Gui<'a> {
    pub fn new(app: &'a Application) -> Self {
        Self {
            app,
            filename: None,
            file_data: None,
        }
    }

    pub fn present(&mut self) {
        let title = match &self.filename {
            Some(filename) => format!("Markdown Editor {filename}"),
            None => "Markdown Editor".to_string(),
        };

        let window = ApplicationWindow::builder()
            .application(self.app)
            .title(title.as_ref())
            .build();

        window.present();
    }
}
