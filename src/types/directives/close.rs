use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::types::date::Date;
/// # Close Directive
///
/// The Close directive is used to indicate that an account has become inactive.
///
/// ## Syntax
/// ```
/// YYYY-MM-DD close Account
/// ```
///
/// Example:
/// ```
/// 2016-11-28 close Liabilities:CreditCard:CapitalOne
/// ```
///
/// ## Key Functions
/// 1. Error Prevention: Raises an error if postings are made to the account after its closing date.
/// 2. Reporting Aid: Helps filtering out closed accounts in reports for periods after closure.
///
/// ## Important Notes
/// - Does not currently generate an implicit zero balance check.
/// - Once closed, an account cannot be reopened (except by removing the Close directive).
/// - Utility functions exist to determine which accounts are open on a particular date.
///
/// ## Best Practice
/// It's recommended to close accounts in your ledger when they close in reality to keep your records tidy.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.wf248e8stnac>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Close<'a> {
    /// Date the account was closed.
    pub date: Date<'a>,

    /// Account being closed.
    pub account: Account,
}
