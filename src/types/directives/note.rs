use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::types::date::Date;

/// # Note Directive
///
/// The Note directive is used to attach dated comments to specific accounts in the journal.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD note Account Description
/// ```ignore
///
/// ## Key Points
/// 1. Attaches a dated comment to a particular account.
/// 2. Useful for recording additional information not captured in transactions.
/// 3. The description can be a multi-line string.
/// 4. Notes are rendered in context when the journal is displayed.
///
/// ## Example
/// ```ignore
/// 2013-11-03 note Liabilities:CreditCard "Called about fraudulent card."
/// ```ignore
///
/// ## Usage Notes
/// - Helps record important facts or events related to an account.
/// - Can capture information that doesn't fit into standard transaction formats.
/// - Provides additional context when reviewing account history.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.c4cyaa6o6rqm>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Note {
    /// Date of the note.
    pub date: Date,

    /// Account being noted.
    pub account: Account,

    /// Note description.
    pub comment: String,
}
