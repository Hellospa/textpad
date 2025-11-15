slint::include_modules!();

mod muti_lang;

use slint;
use slint::ToSharedString;
use crate::muti_lang::MutiLang;

fn main() -> Result<(), slint::PlatformError> {
    let muti_lang = MutiLang::load("zh_simple", "resource/lang");

    let app = TextPad::new()?;

    app.set_save_button_text(muti_lang.get_text("save_button_text").unwrap_or("Save").to_shared_string());
    app.set_save_as_button_text(muti_lang.get_text("save_as_button_text").unwrap_or("Save As").to_shared_string());
    app.set_open_button_text(muti_lang.get_text("open_button_text").unwrap_or("Open").to_shared_string());
    app.set_new_button_text(muti_lang.get_text("new_button_text").unwrap_or("New").to_shared_string());

    app.run()
}
