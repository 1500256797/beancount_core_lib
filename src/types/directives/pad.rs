use std::convert::TryFrom;

use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::currency::Currency;
use crate::types::date::Date;

/// # Pad Directive
///
/// The Pad directive automatically inserts a transaction to make a subsequent balance assertion succeed.
///
/// ## Syntax
/// ```
/// YYYY-MM-DD pad Account AccountPad
/// ```
///
/// ## Key Points
/// 1. Automatically inserts a transaction to balance accounts.
/// 2. Used primarily for initializing new account balances.
/// 3. Works in conjunction with balance assertions.
/// 4. The inserted transaction uses a special 'P' flag.
/// 5. Typically used for balance sheet accounts (Assets and Liabilities).
///
/// ## Example
/// ```
/// 2002-01-17 open Assets:US:BofA:Checking
/// 2002-01-17 pad Assets:US:BofA:Checking Equity:Opening-Balances
/// 2014-07-09 balance Assets:US:BofA:Checking  987.34 USD
/// ```
///
/// ## Resulting Inserted Transaction
/// ```
/// 2002-01-17 P "(Padding inserted for balance of 987.34 USD)"
///   Assets:US:BofA:Checking        987.34 USD
///   Equity:Opening-Balances       -987.34 USD
/// ```
///
/// ## Usage Notes
/// - Can be used between balance assertions to adjust for differences.
/// - The amount is automatically calculated to satisfy the balance assertion.
/// - Without balance assertions, Pad directives have no effect.
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.aw8ic3d8k8rq>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Pad<'a> {
    /// Date of the pad.
    pub date: Date<'a>,

    /// Account to pad into.
    pub pad_to_account: Account,

    /// Account to pad from.
    pub pad_from_account: Account,
}
