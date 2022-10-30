use crossterm::Result;
use std::io::stdout;

use rusty_editor::App;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut app = App::new(&mut stdout)?;
    app.run()
}
