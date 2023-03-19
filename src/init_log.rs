use pretty_env_logger;
pub use log::LevelFilter;
use std::io::{Write};
use colored::*;
use std::thread;

pub fn init_log(filter_level: LevelFilter) {
    pretty_env_logger::formatted_builder()
       .filter_level(filter_level)
       .format(move | buf, record| {
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Cyan,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Magenta,
            };
            let level_symbol = match record.level() {
                log::Level::Error => "✖".bold(),
                log::Level::Warn => "⚠".bold(),
                log::Level::Info => "ℹ".bold(),
                log::Level::Debug => "⚙".bold(),
                log::Level::Trace => "➤".bold(),
            };
            let module_path = record.module_path().unwrap_or("<unknown>");
            let module_name = module_path.split("::").last().unwrap_or_default();
            let truncated_module_name = truncate_module_name(module_name, 8);
            let current_thread = thread::current();
            let thread_id = if current_thread.name().unwrap_or("") != "main" {
                format!(" {{tid:{}}}", filter_numeric_chars(format!("{:?}", current_thread.id())))
            } else {
                String::new()
            };
            let log_line = format!(
                "{} [{:<8}]{} {}",
                level_symbol,
                truncated_module_name,
                thread_id,
                record.args()
            );
            writeln!(buf, "{}", log_line.color(level_color))
        }).init();
    debug!("Initialized logger");
}


fn truncate_module_name(module_name: &str, max_length: usize) -> String {
    if module_name.len() > max_length {
        let truncated_length = max_length - 3; // Subtract 3 for "..."
        let start = &module_name[..truncated_length / 2];
        let end = &module_name[module_name.len() - truncated_length / 2..];
        format!("{}...{}", start, end)
    } else {
        module_name.to_owned()
    }
}

fn filter_numeric_chars(input: String) -> String {
    let filtered: String = input.chars()
        .filter(|c| c.is_numeric())
        .collect();
    if filtered.is_empty() {
        input
    } else {
        filtered
    }
}
