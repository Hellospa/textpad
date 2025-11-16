use std::io::Lines;

pub struct TextEditor {
    pub content: Vec<String>,
    pub file_path: Option<String>,

    pub font_size: u32,
    pub font_family: String,

    pub cursor_position: usize,
    pub selection_start: Option<(usize, usize)>,
    pub selection_end: Option<(usize, usize)>,

    pub scroll_position: (usize, usize),
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