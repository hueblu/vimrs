//!
mod text_buffer;

use text_buffer::*;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
    Result,
};

/// Main struct that represents the editor.
/// Contains all of the editor's logic for taking input,
/// drawing to the terminal, and loading and saving files.
///
/// After creating an `App` with [`new`], you can run
/// it with [`run`].
///
/// [`new`]: App::new
/// [`run`]: App::run
pub struct App<'w, W> {
    cursor: Cursor,
    buffer: TextBuffer,
    is_body_dirty: bool,
    close: bool,
    mode: Mode,
    size: (u16, u16),
    out: &'w mut W,
}

impl<'w, W> App<'w, W>
where
    W: std::io::Write,
{
    /// Generates a new App Struct
    ///
    ///
    pub fn new(out: &'w mut W) -> Result<App<'w, W>> {
        Ok(App {
            cursor: Cursor::new(0, 0),
            buffer: TextBuffer::from_path("./src/lib.rs")?,
            is_body_dirty: true,
            close: false,
            mode: Mode::Normal,
            size: size()?,
            out,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(self.out, EnterAlternateScreen)?;

        loop {
            self.draw()?;
            self.early_update()?;
            self.out.flush()?;
            self.update(event::read()?)?;

            if self.close {
                break;
            }
        }

        execute!(self.out, LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn early_update(&mut self) -> Result<()> {
        queue!(self.out, cursor::MoveTo(self.cursor.x, self.cursor.y))?;
        Ok(())
    }

    fn update(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if let KeyCode::Char(c) = key.code {
                match &self.mode {
                    Mode::Normal => match c {
                        ':' => self.mode = Mode::EnteringCommand(String::new()),
                        'h' => self.cursor.move_position(-1, 0)?,
                        'l' => self.cursor.move_position(1, 0)?,
                        'j' => self.cursor.move_position(0, 1)?,
                        'k' => self.cursor.move_position(0, -1)?,
                        'q' => self.close = true,
                        _ => {}
                    },
                    Mode::EnteringCommand(_command) => {}
                    Mode::Editing => {
                        self.is_body_dirty = true;
                    }
                }
            }
        } else if let Event::Resize(x, y) = event {
            self.size = (x, y);
        }
        self.cursor.update()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        queue!(self.out, cursor::MoveTo(0, 0))?;

        if self.is_body_dirty {
            for i in 0..self.size.1 - 1 {
                queue!(
                    self.out,
                    style::Print(self.buffer.get_line(i as usize)),
                    cursor::MoveToColumn(0)
                )?;
            }
            self.is_body_dirty = false;
        }

        // queue!(
        // self.out,
        // cursor::MoveTo(208, 58),
        // style::Print(format!(
        // "mode:  {}, x: {}, y: {}",
        // self.mode.into_string(),
        // self.size.0,
        // self.size.1
        // ))
        // )?;

        Ok(())
    }

    fn get_bar(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
enum Mode {
    Normal,
    EnteringCommand(String),
    Editing,
}

impl Mode {
    fn into_string(&self) -> String {
        match self {
            Mode::Normal => "NOR".to_string(),
            Mode::EnteringCommand(s) => s.clone(),
            Mode::Editing => "INS".to_string(),
        }
    }
}

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn new(x: u16, y: u16) -> Cursor {
        Cursor { x, y }
    }

    fn update(&mut self) -> Result<()> {
        self.justify()?;
        Ok(())
    }

    fn justify(&mut self) -> Result<()> {
        Ok(())
    }

    fn move_position(&mut self, move_x: i16, move_y: i16) -> Result<()> {
        self.x = self.x.saturating_add_signed(move_x);
        self.y = self.y.saturating_add_signed(move_y);
        self.update()?;
        Ok(())
    }
}
