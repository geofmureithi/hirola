mod html_macro;
use std::path::Path;

use source_file::{format_file_source, FormatError};

mod collect;
mod format;
mod source_file;

#[cfg(test)]
mod test_helpers;

pub use self::format::*;
pub use collect::collect_macros_in_file;

pub fn format_file(path: &Path, settings: FormatterSettings) -> Result<String, FormatError> {
    let file = std::fs::read_to_string(path)?;
    format_file_source(&file, settings)
}
