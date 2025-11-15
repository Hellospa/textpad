slint::include_modules!();

mod muti_lang;
mod text_editor;

use slint;
use crate::muti_lang::MutiLang;

fn main() -> Result<(), slint::PlatformError> {
    let muti_lang = MutiLang::load("zh_simple", "resource/lang");

    let app = TextPad::new()?;
    app.run()
}
