pub fn setup_logger(level: u8) -> anyhow::Result<()> {
    use log::LevelFilter::*;

    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open("output.log")?;
    file.set_len(0)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(match level {
            0 => Error,
            1 => Warn,
            2 => Info,
            3 => Debug,
            _ => Trace,
        })
        .chain(file)
        .apply()?;

    Ok(())
}
