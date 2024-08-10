use typed_builder::TypedBuilder;

use crate::currency::Currency;
use crate::types::date::Date;
/// # Commodity Directive
///
/// The Commodity directive is used to declare currencies, financial instruments, or commodities.
///
/// ## Syntax
/// ```
/// YYYY-MM-DD commodity Currency
/// ```
///
/// Example:
/// ```
/// 1867-07-01 commodity CAD
/// ```
///
/// ## Key Points
/// 1. Optional: Commodities can be used without explicit declaration.
/// 2. Main purpose: Attach commodity-specific metadata for later use by plugins.
/// 3. Can include additional information like full name and asset class.
/// 4. Date can be any relevant date, often the introduction date of the currency or financial instrument.
/// 5. It's an error to declare the same commodity twice.
///
/// ## Example with Metadata
/// ```
/// 1867-07-01 commodity CAD
///   name: "Canadian Dollar"
///   asset-class: "cash"
///
/// 2012-01-01 commodity HOOL
///   name: "Hooli Corporation Class C Shares"
///   asset-class: "stock"
/// ```
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.a3si01ejc035>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Commodity<'a> {
    /// Date the commodity was declared.
    pub date: Date<'a>,

    /// Commodity name.
    pub name: Currency<'a>,
}
