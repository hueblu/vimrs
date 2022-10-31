use crossterm::Result;

use crate::text_buffer::*;
use crate::*;

pub struct Editor<'a> {
    dirty: bool,
    current_buffer: &'a TextBuffer,
    offset: u16,
    buffers: Vec<TextBuffer>,
    cursor: Cursor,
    window_size: u16,
}

impl<'a> Editor<'a> {
    pub fn new(size: u16) -> Result<Editor<'a>> {
        let buffers = vec![TextBuffer::new()];

        Ok(Editor {
            dirty: true,
            current_buffer: &buffers[0],
            offset: 0,
            buffers,
            cursor: Cursor::new(),
            window_size: size,
        })
    }

    pub fn update(&mut self, context: &Context) -> Result<()> {
        Ok(())
    }

    pub fn get_line(&self, line_idx: u16) -> Option<&str> {
        self.current_buffer
            .get_line((line_idx + self.offset) as usize)
            .as_str()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clean(&mut self) {
        self.dirty = false;
    }

    pub fn set_size(&mut self, new_size: u16) {
        self.window_size = new_size;
    }

    pub fn get_cursor_pos(&self) -> (u16, u16) {
        let line = self.current_buffer.char_to_line(self.cursor.index);

        (
            self.cursor.index - self.current_buffer.line_to_char(line),
            line - self.offset,
        )
    }
}

struct Cursor {
    index: u16,
}

impl Cursor {
    fn new() -> Cursor {
        Cursor { index: 0 }
    }
}
