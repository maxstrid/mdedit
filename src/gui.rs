use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, ScrolledWindow, TextBuffer, TextView, FileChooserNative, Button};

use crate::markdown::{read, write};

const DEFAULT_WINDOW_WIDTH: i32 = 800;
const DEFAULT_WINDOW_HEIGHT: i32 = 600;

#[derive(Clone)]
pub struct Gui<'a> {
    app: &'a Application,
    window: ApplicationWindow,
    text_buffer: TextBuffer,
    files: Option<Vec<(String, bool)>> // (Filename, is the file open)
}

impl<'a> Gui<'a> {
    pub fn new(app: &'a Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Markdown Editor")
            .width_request(DEFAULT_WINDOW_WIDTH)
            .height_request(DEFAULT_WINDOW_HEIGHT)
            .build();
        
        let text_buffer = TextBuffer::builder()
            .enable_undo(true)
            .build();

        Self {
            app,
            window,
            text_buffer,
            files: None,
        }
    }

    pub fn present(&mut self) {
        // Temporary read
        let data = match read("example.md".to_string()) {
            Ok(data) => {
                if self.files.is_none() {
                    let _files: Vec<(String, bool)> = vec![("example.md".to_string(), true)];
                    self.files = Some(_files);
                }
                data
            },
            Err(err) => {
                self.error_window(err.as_str());
                "".to_string()
            }
        };

        self.text_buffer.set_text(data.as_str());

        self.build_ui();

        self.window.present();
    }

    fn build_ui(&self) {
        // Top bar
        let open_file_preview = FileChooserNative::builder()
            .title("Choose A File")
            .action(gtk::FileChooserAction::Open)
            .transient_for(&self.window)
            .accept_label("Open")
            .cancel_label("Cancel")
            .build();
        
        let open_file_button = Button::with_label("Open File");

        open_file_button.connect_clicked(|_| {
            todo!() // Do something with open_file_preview
        });

        // Text view
        let text_view = TextView::builder()
            .buffer(&self.text_buffer)
            .editable(true)
            .accepts_tab(true)
            .cursor_visible(true)
            .monospace(true)
            .input_purpose(gtk::InputPurpose::FreeForm)
            .wrap_mode(gtk::WrapMode::WordChar)
            .build();
        
        let scrollable_view = ScrolledWindow::builder()
            .child(&text_view)
            .overlay_scrolling(true)
            .build();

        self.window.set_child(Some(&scrollable_view));
    }

    fn error_window(&self, error: &str) {
        let error_window = ApplicationWindow::builder()
            .application(self.app)
            .title("Error!")
            .width_request(600)
            .height_request(200)
            .build();

        error_window.set_child(Some(&Label::new(Some(error))));

        error_window.present();
    }
}
