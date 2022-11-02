use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::stdout;

use rusty_editor::App;

fn main() -> Result<()> {
    enable_raw_mode()?;
    println!("got here");
    let mut stdout = stdout();
    let mut app = App::new(&mut stdout)?;
    app.run()?;
    disable_raw_mode()?;
    Ok(())
}
