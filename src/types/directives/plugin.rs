use std::borrow::Cow;
use std::convert::TryFrom;

use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::currency::Currency;
use crate::types::date::Date;
/// # Plugin Directive
///
/// The Plugin directive allows loading and configuring Python modules as plugins in Beancount.
///
/// ## Syntax
/// ```
/// plugin "beancount.plugins.module_name" "optional configuration data"
/// ```
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
/// ```
/// plugin "beancount.plugins.module_name" "configuration data"
/// ```
///
/// ## References
/// - Refer to "Scripting & Plugins" documentation for detailed information.
/// - Check individual plugin documentation for specific configuration options.

/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.lxgs9ewvbt8k>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Plugin<'a> {
    /// Full module name of the plugin.
    pub module: Cow<'a, str>,

    /// Configuration data to be passed to the plugin.
    #[builder(default)]
    pub config: Option<Cow<'a, str>>,

    /// Source string from the parsed input
    #[builder(default)]
    pub source: Option<&'a str>,
}
