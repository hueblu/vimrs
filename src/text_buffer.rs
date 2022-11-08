#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
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
    pub fn new() -> Result<TextBuffer> {
        Ok(TextBuffer {
            text: Rope::new(),
            path: None,
            dirty: false,
        })
    }

    pub fn from_path(path: &str) -> Result<TextBuffer> {
        Ok(TextBuffer {
            text: Rope::from_reader(&mut BufReader::new(File::open(&path)?))?,
            path: Some(PathBuf::from(path.to_string())),
            dirty: false,
        })
    }

    pub fn get_line(&self, idx: usize) -> Option<RopeSlice> {
        self.text.get_line(idx)
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

    pub fn char_to_line(&self, char_idx: u16) -> u16 {
        self.text.char_to_line(char_idx as usize) as u16
    }

    pub fn line_to_char(&self, line_idx: u16) -> u16 {
        self.text.line_to_char(line_idx as usize) as u16
    }

    pub fn line(&self, line_idx: usize) -> RopeSlice<'_> {
        self.text.line(line_idx)
    }

    pub fn edit(&mut self, index: u16) -> Result<()> {
        Ok(())
    }
}
