//! editor
mod editor;
mod text_buffer;

use editor::*;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue, style,
    terminal::{self, size, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
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
    close: bool,
    editor: Editor,
    mode: Mode,
    context: Context,
    out: &'w mut W,
}

impl<'w, W> App<'w, W>
where
    W: std::io::Write,
{
    /// Generates a new `App` Struct from an
    /// object that implements `Write`
    pub fn new(out: &'w mut W) -> Result<App<'w, W>> {
        let size = size()?;

        Ok(App {
            close: false,
            editor: Editor::from_path(size.1 - 1, "src/lib.rs")?,
            mode: Mode::Normal,
            context: Context {
                size_x: size.0,
                size_y: size.1,
            },
            out,
        })
    }

    pub fn run(&mut self) -> Result<()> {
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

        Ok(())
    }

    fn early_update(&mut self) -> Result<()> {
        let cursor_pos = self.editor.get_cursor_pos();
        queue!(self.out, cursor::MoveTo(cursor_pos.0, cursor_pos.1))
    }

    fn update(&mut self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            match &mut self.mode {
                Mode::Normal => {
                    if let KeyCode::Char(c) = key.code {
                        match c {
                            ':' => self.mode = Mode::EnteringCommand(String::new()),
                            'h' => self.editor.move_cursor(-1, 0)?,
                            'l' => self.editor.move_cursor(1, 0)?,
                            'j' => self.editor.move_cursor(0, 1)?,
                            'k' => self.editor.move_cursor(0, -1)?,
                            'q' => self.close = true,
                            'i' => self.mode = Mode::Editing,
                            _ => {}
                        }
                    }
                }
                Mode::EnteringCommand(command) => match key.code {
                    KeyCode::Esc => self.mode = Mode::Normal,
                    KeyCode::Char(c) => command.push(c),
                    _ => {}
                },
                Mode::Editing => match key.code {
                    KeyCode::Esc => self.mode = Mode::Normal,
                    KeyCode::Char(c) => self.editor.insert_char(c)?,
                    _ => {}
                },
            }
        } else if let Event::Resize(x, y) = event {
            self.context.set_size((x, y));
            self.editor.set_size(y - 1);
        }
        self.editor.update(&self.context)?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        if self.editor.is_dirty() {
            queue!(self.out, cursor::MoveTo(0, 0))?;

            for i in 0..self.context.size_y - 1 {
                if let Some(line) = self.editor.get_line(i) {
                    queue!(self.out, style::Print(line), cursor::MoveToColumn(0))?;
                }
            }
            self.editor.clean();
        }

        let cursor_pos = self.editor.get_cursor_pos();
        queue!(
            self.out,
            cursor::MoveTo(0, self.context.size_x),
            terminal::Clear(ClearType::CurrentLine),
            style::Print(format!(
                " {} index: {}, x: {}, y: {}, line: {}",
                self.mode.into_string(),
                self.editor.cursor_idx(),
                cursor_pos.0,
                cursor_pos.1,
                self.editor.get_cursor_line_idx(),
            )),
        )?;

        Ok(())
    }

    fn get_bar(&self) -> Result<()> {
        Ok(())
    }

    fn run_command(&mut self, command: String) -> Result<()> {
        Ok(())
    }
}

struct Context {
    size_x: u16,
    size_y: u16,
}

impl Context {
    fn get_size(&self) -> (u16, u16) {
        (self.size_x, self.size_y)
    }

    fn set_size(&mut self, size: (u16, u16)) {
        self.size_x = size.0;
        self.size_y = size.1;
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
            Mode::EnteringCommand(s) => format!("NOR :{}", s.clone()),
            Mode::Editing => "INS".to_string(),
        }
    }
}
