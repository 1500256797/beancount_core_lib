use crate::types::date::Date;
use typed_builder::TypedBuilder;

/// # Event Directive
///
/// The Event directive tracks the value of user-defined variables over time.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD event Name Value
/// ```ignore
///
/// ## Key Points
/// 1. Used to record changes in user-defined variables.
/// 2. Each event type has only one value per day.
/// 3. Event names don't need pre-declaration.
/// 4. Values can be any string without prescribed structure.
///
/// ## Examples
/// ```ignore
/// 2014-07-09 event "location" "Paris, France"
/// ```ignore
///
/// ## Common Use Cases
/// - Tracking location
/// - Recording address changes
/// - Logging employment history
/// - Marking trading windows for stocks
///
/// ## Usage Notes
/// - Useful for reporting on time-based conditions (e.g., residency for tax purposes).
/// - Can be used in future filtering language for complex queries.
/// - Not associated with any specific account.
///
/// ## Current Limitations
/// - Filters and event reports are not yet implemented in Beancount 2.0.
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.tm5fxddlik5x>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Event {
    /// Date the event occurred.
    pub date: Date,

    /// Name of the event.
    pub name: String,

    /// New value of the event.
    pub description: String,
}
