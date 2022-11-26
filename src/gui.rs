use gtk::{prelude::*, ScrolledWindow};
use gtk::{Application, ApplicationWindow};
use gtk::{TextBuffer, TextView};

use crate::markdown::{read, write};

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
        // Temporary read
        let data = read("example.md".to_string()).unwrap();

        let title = match &self.filename {
            Some(filename) => format!("Markdown Editor {filename}"),
            None => "Markdown Editor".to_string(),
        };

        let window = ApplicationWindow::builder()
            .application(self.app)
            .title(title.as_ref())
            .build();

        let buffer = TextBuffer::builder()
            .enable_undo(true)
            .text(data.as_ref())
            .build();

        buffer.connect_insert_text(|itself, iter, stri| {
            println!(
                "{}",
                itself.text(&itself.start_iter(), &itself.end_iter(), false)
            );
        });

        let view = TextView::builder()
            .accepts_tab(true)
            .buffer(&buffer)
            .build();

        let win = ScrolledWindow::builder().child(&view).build();

        window.set_child(Some(&win));
        window.present();
    }
}
