use typed_builder::TypedBuilder;
/// # Plugin Directive
///
/// The Plugin directive allows loading and configuring Python modules as plugins in Beancount.
///
/// ## Syntax
/// ```ignore
/// plugin "beancount.plugins.module_name" "optional configuration data"
/// ```ignore
///
/// ## Key Points
/// 1. Plugins are Python modules loaded from the PYTHONPATH.
/// 2. Plugins can transform entries or output errors.
/// 3. Optional configuration data can be provided as a string.
/// 4. Plugin processing order is determined by the order of plugin directives.
///
/// ## Usage Notes
/// - Allows integration of custom code within Beancount.
/// - Configuration format is plugin-specific.
/// - The "plugin processing mode" option affects built-in plugin execution.
///
/// ## Example
/// ```ignore
/// plugin "beancount.plugins.module_name" "configuration data"
/// ```ignore
///
/// ## References
/// - Refer to "Scripting & Plugins" documentation for detailed information.
/// - Check individual plugin documentation for specific configuration options.

/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.lxgs9ewvbt8k>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Plugin {
    /// Full module name of the plugin.
    pub module: String,

    /// Configuration data to be passed to the plugin.
    #[builder(default)]
    pub config: Option<String>,

    /// Source string from the parsed input
    #[builder(default)]
    pub source: Option<String>,
}

impl std::fmt::Display for Plugin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "plugin {} {}", self.module, self.config.as_deref().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let plugin = Plugin::builder().module("beancount.plugins.module_name".to_string()).config(Some("configuration data".to_string())).build();
        assert_eq!(plugin.to_string(), "plugin beancount.plugins.module_name configuration data");
    }
}