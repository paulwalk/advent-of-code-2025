use flexi_logger::{colored_detailed_format};

pub fn configure_flexi_logger(log_level: String) -> Result<(), Box<dyn std::error::Error>> {
    flexi_logger::Logger::try_with_str(log_level)?.log_to_stdout().set_palette("1;5;32;3;-".parse()?).format(colored_detailed_format).start()?;
    log::debug!("Debug level logging is enabled");
    Ok(())
}

