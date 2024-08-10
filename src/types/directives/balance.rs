use rust_decimal::Decimal;
use typed_builder::TypedBuilder;

use crate::{account::Account, amount::Amount, types::date::Date};
/// # Balance Assertion Directive
///
/// Balance assertions are used to verify account balances at specific points in time.
///
/// ## Syntax
/// ```
/// YYYY-MM-DD balance Account Amount
/// ```
///
/// ## Key Points
/// 1. Verifies that the account balance matches the specified amount on the given date.
/// 2. Applies at the beginning of the specified date (midnight).
/// 3. Useful for catching errors and ensuring transaction accuracy.
/// 4. Most relevant for balance sheet accounts (Assets and Liabilities).
/// 5. Accounts have an implicit zero balance assertion at their opening date.
///
/// ## Example
/// ```
/// 2014-12-26 balance Liabilities:US:CreditCard   -3492.02 USD
/// ```
///
/// ## Usage Notes
/// - Helps catch issues like duplicate transactions.
/// - Not typically used for income statement accounts (Income and Expenses).
/// - Beancount reports an error if the actual balance doesn't match the assertion.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.l0pvgeniwvq8>
#[derive(Clone, Debug, PartialEq, TypedBuilder)]
pub struct Balance<'a> {
    /// Date of the balance.
    pub date: Date<'a>,

    /// Account to check the balance of.
    pub account: Account,

    /// Amount to balance.
    pub amount: Amount<'a>,

    #[builder(default)]
    pub tolerance: Option<Decimal>,
}
