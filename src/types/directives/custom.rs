use typed_builder::TypedBuilder;

use crate::types::date::Date;

/// # Custom Directive
///
/// The Custom directive allows users to define their own directive types in Beancount.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD custom TypeName Value1 Value2 ...
/// ```ignore
///
/// ## Key Points
/// 1. Designed as a flexible, generic directive for prototyping new features.
/// 2. Allows external plugins and clients to define custom directive types.
/// 3. First argument (TypeName) is a string that acts as the directive type.
/// 4. Subsequent arguments can be strings, dates, booleans, amounts, or numbers.
///
/// ## Example
/// ```ignore
/// 2014-07-09 custom "budget" "..." TRUE 45.30 USD
/// ```ignore
///
/// ## Usage Notes
/// - Intended as an interim solution before full custom directive support.
/// - No built-in validation for argument consistency across instances of the same type.
/// - Useful for prototyping features like budgeting.
///
/// ## Future Plans
/// - Long-term goal is to allow full custom directive definition and validation.
///
/// Represents a `custom` directive, which is a generic directive provided to allow clients to
/// experiment with new features, e.g., budgeting.
///
/// The long-term plan for Beancount is to allow plugins and
/// external clients to define their own directive types, to be declared and validated by the
/// Beancount input language parser. In the meantime, a generic directive is provided for clients
/// to prototype new features, e.g., budgeting.
///
/// The grammar for this directive is flexible:
///
/// ```ignoretext
/// YYYY-MM-DD custom TypeName Value1 ...
/// ```ignore
///
/// The first argument is a string and is intended to be unique to your directive. Think of this as
/// the type of your directive. Following it, you can put an arbitrary list of strings, dates,
/// booleans, amounts, and numbers.
///
/// Example custom directive:
///
/// ```ignoretext
/// 2014-07-09 custom "budget" "..." TRUE 45.30 USD
/// ```ignore
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.20klpeqb6ajy>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Custom {
    /// Date associated with the custom directive.
    pub date: Date,

    /// Custom directive name.
    pub name: String,

    /// Arbitrary number of custom directive arguments.
    pub args: Vec<String>,
}
