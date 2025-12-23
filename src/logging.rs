use flexi_logger::{style, DeferredNow, LogSpecification};
use log::Record;

pub fn configure_flexi_logger(log_spec: String) -> Result<(), Box<dyn std::error::Error>> {
    let spec = LogSpecification::parse(log_spec).unwrap();
    flexi_logger::Logger::with(spec).log_to_stdout().set_palette("1;5;32;3;-".parse()?).format(custom_logging_format).start()?;
    log::debug!("Debug level logging is enabled");
    Ok(())
}

pub fn custom_logging_format(w: &mut dyn std::io::Write, _now: &mut DeferredNow, record: &Record) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{}: {}:{} ",
        style(level).paint(level.to_string()),
        style(level).paint(record.file().unwrap_or("<unnamed>")),
        style(level).paint(record.line().unwrap_or(0).to_string()),
    )?;
    write!(w, "{}", record.args())
}
