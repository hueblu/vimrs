use anyhow::Result;
use ropey::RopeSlice;

use crate::text_buffer::*;
use crate::*;

pub struct Editor {
    dirty: bool,
    current_buffer: u16,
    offset: u16,
    buffers: Vec<TextBuffer>,
    cursor: Cursor,
    window_size: u16,
}

impl Editor {
    pub fn new(size: u16) -> Result<Editor> {
        let buffers = vec![TextBuffer::new()?];

        log::info!("editor created");
        Ok(Editor {
            dirty: true,
            current_buffer: 0,
            offset: 0,
            buffers,
            cursor: Cursor::new()?,
            window_size: size,
        })
    }

    pub fn from_path(size: u16, path: &str) -> Result<Editor> {
        let buffers = vec![TextBuffer::from_path(path)?];

        log::info!("new editor created from path {}", path);
        Ok(Editor {
            dirty: true,
            current_buffer: 0,
            offset: 0,
            buffers,
            cursor: Cursor::new()?,
            window_size: size,
        })
    }

    pub fn current_buffer(&self) -> Result<&TextBuffer> {
        Ok(&self.buffers[self.current_buffer as usize])
    }

    pub fn update(&mut self, context: &AppContext) -> Result<()> {
        Ok(())
    }

    pub fn get_line(&self, line_idx: u16) -> Result<RopeSlice> {
        Ok(self
            .current_buffer()?
            .get_line((line_idx + self.offset) as usize)
            .unwrap())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clean(&mut self) -> Result<()> {
        self.dirty = false;
        Ok(())
    }

    pub fn set_size(&mut self, new_size: u16) {
        self.window_size = new_size;
    }

    pub fn get_cursor_line_idx(&self) -> Result<u16> {
        Ok(self.current_buffer()?.char_to_line(self.cursor.index))
    }

    pub fn cursor_idx(&self) -> u16 {
        self.cursor.index
    }

    pub fn get_cursor_pos(&self) -> Result<(u16, u16)> {
        let line = self.current_buffer()?.char_to_line(self.cursor.index);

        Ok((
            self.cursor.index - self.current_buffer()?.line_to_char(line),
            line.saturating_sub(self.offset),
        ))
    }

    pub fn move_cursor(&mut self, x: i16, y: i16) -> Result<()> {
        let line = self.get_cursor_line_idx()?;

        self.cursor.index = self.cursor.index.saturating_add_signed(x);

        let mut char_offset = self
            .cursor
            .index
            .saturating_sub(self.current_buffer()?.line_to_char(line));

        if x != 0 {
            self.cursor.previous_offset = char_offset;
        }

        if y != 0 {
            let new_line_length = self
                .current_buffer()?
                .get_line((line as i16 + y) as usize)
                .unwrap_or_else(|| self.current_buffer().unwrap().get_line(0).unwrap())
                .len_chars();
            char_offset = self.cursor.previous_offset + 1;
            if new_line_length < char_offset as usize {
                char_offset = new_line_length as u16;
            }
            self.cursor.index = self
                .current_buffer()?
                .line_to_char(line.saturating_add_signed(y))
                + char_offset.saturating_sub(1);
        }

        log::debug!(
            "cursor moved x: {}, y: {}, to index: {}",
            x,
            y,
            self.cursor.index
        );

        Ok(())
    }

    pub fn insert_char(&mut self, c: char) -> Result<()> {
        log::debug!("inserted char '{}' at index '{}'", c, self.cursor.index);
        Ok(())
    }
}

struct Cursor {
    index: u16,
    previous_offset: u16,
}

impl Cursor {
    fn new() -> Result<Cursor> {
        log::info!("new cursor created");
        Ok(Cursor {
            index: 0,
            previous_offset: 0,
        })
    }
}
