//! ## Logging
//!
//! `logging` is the module which initializes the logging system for termscp

/**
 * MIT License
 *
 * termscp - Copyright (c) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
// locals
use crate::system::environment::{get_log_paths, init_config_dir};
use crate::utils::file::open_file;
// ext
use simplelog::{Color, ConfigBuilder, Level, LevelFilter, WriteLogger};
use std::fs::File;
use std::path::PathBuf;

/// ### init
///
/// Initialize logger
pub fn init() -> Result<(), String> {
    // Init config dir
    let config_dir: PathBuf = match init_config_dir() {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Err(String::from(
                "This system doesn't seem to support CONFIG_DIR",
            ))
        }
        Err(err) => return Err(err),
    };
    let log_file_path: PathBuf = get_log_paths(config_dir.as_path());
    // Open log file
    let file: File = open_file(log_file_path.as_path(), true, true, false)
        .map_err(|e| format!("Failed to open file {}: {}", log_file_path.display(), e))?;
    // Prepare log config
    let config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%dT%H:%M:%S%z")
        .set_level_color(Level::Trace, None)
        .set_level_color(Level::Debug, Some(Color::Cyan))
        .set_level_color(Level::Info, Some(Color::Yellow))
        .set_level_color(Level::Warn, Some(Color::Magenta))
        .set_level_color(Level::Error, Some(Color::Red))
        .build();
    // Make logger
    WriteLogger::init(LevelFilter::Debug, config, file)
        .map_err(|e| format!("Failed to initialize logger: {}", e))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_system_logging_setup() {
        assert!(init().is_ok());
    }
}
