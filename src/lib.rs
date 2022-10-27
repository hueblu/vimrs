mod text_buffer;

use text_buffer::*;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    queue, style,
    terminal::{self, size, ClearType},
    Result,
};

pub struct App<'w, W> {
    cursor: Cursor,
    buffer: TextBuffer,
    close: bool,
    mode: Mode,
    size: (u16, u16),
    w: &'w mut W,
}

impl<'w, W> App<'w, W>
where
    W: std::io::Write,
{
    pub fn new(w: &'w mut W) -> Result<App<'w, W>> {
        Ok(App {
            cursor: Cursor::new(0, 0),
            buffer: TextBuffer::from_path("./src/main.rs")?,
            close: false,
            mode: Mode::Normal,
            size: size()?,
            w,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            queue!(
                self.w,
                terminal::Clear(ClearType::All),
                style::ResetColor,
                cursor::MoveTo(self.size.0, self.size.1),
            )?;

            self.draw()?;

            queue!(
                self.w,
                cursor::MoveTo(self.cursor.x, self.cursor.y),
                style::Print(format!("{} : {}\n", self.cursor.x, self.cursor.y).to_string()),
            )?;

            self.w.flush()?;
            self.update(event::read()?)?;

            if self.close {
                break;
            }
        }
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
                    Mode::Editing => {}
                }
            }
        } else if let Event::Resize(x, y) = event {
            self.size = (x, y);
        }
        self.cursor.update()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        queue!(
            self.w,
            style::Print(self.buffer.get_lines_at(0, (self.size.1 - 1).into()))
        )?;

        Ok(())
    }
}

#[derive(Debug)]
enum Mode {
    Normal,
    EnteringCommand(String),
    Editing,
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
