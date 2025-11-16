slint::include_modules!();
mod muti_lang;
mod text_editor;

use std::cell::RefCell;
use rfd::{FileDialog, MessageButtons, MessageDialog};
use slint::{self};
use crate::muti_lang::MutiLang;

thread_local! {
    static TEXT_PAD : RefCell<TextPadApp> = RefCell::new(TextPadApp {
        muti_lang: MutiLang::load("en_us", "resource/lang"),
        editors: vec![text_editor::TextEditor::new()],
    })
}

struct TextPadApp {
    pub muti_lang: MutiLang,
    pub editors: Vec<text_editor::TextEditor>,
}

fn main() -> Result<(), slint::PlatformError> {
    let app = std::rc::Rc::new(TextPad::new()?);

    let app_ = app.clone();
    app.on_save_file(move || {
        let current_index = app_.get_current_file_index();
        TEXT_PAD.with(|text_pad| {
            let textpad = text_pad.borrow();
            if current_index >= 0 && current_index < textpad.editors.len() as i32 {
                std::fs::write(app_.get_current_file(), textpad.editors[current_index as usize].content.join("\n"))
                   .expect("Failed to save file");
            }
            else {
                let file_path = FileDialog::new()
                    .set_title(textpad.muti_lang.get_text("save_file_dialog_title").unwrap_or("Save File"))
                    .add_filter("Any File", &["*"])
                    .save_file();
                if file_path.is_none() {
                    return;
                }
                std::fs::write(file_path.unwrap(), textpad.editors[current_index as usize].content.join("\n"))
                   .expect("Failed to save file");
            }
        })
    });
    let app_ = app.clone();
    app.on_save_as_file(move || {
        let current_index = app_.get_current_file_index();
        TEXT_PAD.with(|text_pad| {
            let textpad = text_pad.borrow();
            let file_path = FileDialog::new()
                .set_title(textpad.muti_lang.get_text("save_as_file_dialog_title").unwrap_or("Save As File"))
                .add_filter("Any File", &["*"])
                .save_file();
            if file_path.is_none() {
                return;
            }
            std::fs::write(file_path.unwrap(), textpad.editors[current_index as usize].content.join("\n"))
               .expect("Failed to save file");
        })
    });
    let app_ = app.clone();
    app.on_open_file(move || {
        TEXT_PAD.with(|text_pad| {
            let textpad = text_pad.borrow();
            let text = textpad.muti_lang.get_text("open_file_dialog_title").unwrap_or("Open File");
            let file_path = FileDialog::new()
                .set_title(text)
                .add_filter("Any File", &["*"])
                .pick_file();
            if file_path.is_none() {
                return;
            }
            MessageDialog::new()
                .set_title(format!("{}: {}",text, file_path.unwrap().file_name().unwrap().to_string_lossy()))
                .set_buttons(MessageButtons::OkCancelCustom(
                    textpad.muti_lang.get_text("this_window").unwrap_or("This Window").to_string(),
                    textpad.muti_lang.get_text("new_window").unwrap_or("New Window").to_string(),
                ))
                .show();
        })
    });

    app.run()
}