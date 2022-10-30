#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, Error};
use std::path::PathBuf;

use ropey::{
    iter::{Bytes, Chars, Chunks, Lines},
    Rope, RopeSlice,
};

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

    pub fn get_line(&self, idx: usize) -> RopeSlice {
        self.text.line(idx)
    }

    pub fn bytes(&self) -> Bytes {
        self.text.bytes()
    }

    pub fn chars(&self) -> Chars {
        self.text.chars()
    }

    pub fn lines(&self) -> Lines {
        self.text.lines()
    }

    pub fn chunks(&self) -> Chunks {
        self.text.chunks()
    }
}
