use std::fs::File;
use std::io::{BufReader, Error};
use std::path::PathBuf;

use ropey::{Rope, RopeSlice};

pub struct TextBuffer {
    pub text: Rope,
    path: Option<PathBuf>,
    dirty: bool,
}

impl TextBuffer {
    pub fn new() -> TextBuffer {
        TextBuffer {
            text: Rope::new(),
            path: None,
            dirty: false,
        }
    }

    pub fn from_path(path: &str) -> Result<TextBuffer, Error> {
        Ok(TextBuffer {
            text: Rope::from_reader(&mut BufReader::new(File::open(&path)?))?,
            path: Some(PathBuf::from(path.to_string())),
            dirty: false,
        })
    }

    pub fn get_lines_at(&self, start_idx: usize, stop_idx: usize) -> RopeSlice {
        let start = self.text.line_to_char(start_idx);
        let stop = self.text.line_to_char(stop_idx);

        if self.text.len_lines() < start + stop {
            self.text.slice(0..self.text.len_lines())
        } else {
            self.text.slice(start..stop)
        }
    }
}
