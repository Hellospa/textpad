use std::io::Lines;

pub struct TextEditor {
    content: Vec<String>,
    file_path: Option<String>,

    font_size: u32,
    font_family: String,

    cursor_position: usize,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,

    scroll_position: (usize, usize),
}
impl TextEditor {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
            file_path: None,
            font_size: 14,
            font_family: "Arial".to_string(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            scroll_position: (0, 0),
        }
    }

}