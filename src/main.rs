use anyhow::Result;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use vimrs::{util::setup_logger, App};

fn main() -> Result<()> {
    setup_logger(2)?;
    enable_raw_mode()?;

    App::new(&mut std::io::stdout())?.run()?;

    disable_raw_mode()?;

    Ok(())
}
