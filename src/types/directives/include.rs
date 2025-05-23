use typed_builder::TypedBuilder;

/// # Include Directive
///
/// The Include directive allows splitting large Beancount input files into multiple files.
///
/// ## Syntax
/// ```ignore
/// include "path/to/include/file.beancount"
/// ```ignore
///
/// ## Key Points
/// 1. Supports both absolute and relative file paths.
/// 2. Relative paths are resolved from the including file's directory.
/// 3. Include directives are processed separately by the loader, not strictly in order.
/// 4. Order of declarations in Beancount files is not relevant.
///
/// ## Usage Notes
/// - Useful for organizing large financial datasets.
/// - Facilitates source control and flexible file organization.
/// - Options are currently parsed per-file, with top-level file options taking precedence.
///
/// ## Current Limitations
/// - Options handling may be subject to change in future versions.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.86lelow4097r>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Include {
    /// Fully qualified filename, including any necessary path segments.
    pub filename: String,

    /// Source string from the parsed input
    #[builder(default)]
    pub source: Option<String>,
}


impl std::fmt::Display for Include {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "include {}", self.filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let include = Include::builder().filename("path/to/include/file.beancount".to_string()).build();
        assert_eq!(include.to_string(), "include path/to/include/file.beancount");
    }
}