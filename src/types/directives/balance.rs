use rust_decimal::Decimal;
use typed_builder::TypedBuilder;

use crate::{account::Account, amount::Amount, types::date::Date};
/// # Balance Assertion Directive
///
/// Balance assertions are used to verify account balances at specific points in time.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD balance Account Amount
/// ```ignore
///
/// ## Key Points
/// 1. Verifies that the account balance matches the specified amount on the given date.
/// 2. Applies at the beginning of the specified date (midnight).
/// 3. Useful for catching errors and ensuring transaction accuracy.
/// 4. Most relevant for balance sheet accounts (Assets and Liabilities).
/// 5. Accounts have an implicit zero balance assertion at their opening date.
///
/// ## Example
/// ```ignore
/// 2014-12-26 balance Liabilities:US:CreditCard   -3492.02 USD
/// ```ignore
///
/// ## Usage Notes
/// - Helps catch issues like duplicate transactions.
/// - Not typically used for income statement accounts (Income and Expenses).
/// - Beancount reports an error if the actual balance doesn't match the assertion.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.l0pvgeniwvq8>
#[derive(Clone, Debug, PartialEq, TypedBuilder)]
pub struct Balance {
    /// Date of the balance.
    pub date: Date,

    /// Account to check the balance of.
    pub account: Account,

    /// Amount to balance.
    pub amount: Amount,

    #[builder(default)]
    pub tolerance: Option<Decimal>,
}


impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} balance {} {}", self.date, self.account, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::currency::Currency;

    use super::*;


    #[test]
    fn test_display() {
        let balance = Balance::builder().account(Account::from("Liabilities:CreditCard:CapitalOne"))
            .amount(Amount::builder().num(Decimal::from(100)).currency(Currency::from("USD")).build())
            .date(Date::from_str_unchecked("2016-11-28"))
            .build();
        
        assert_eq!(balance.to_string(), "2016-11-28 balance Liabilities:CreditCard:CapitalOne 100 USD");
    }

    #[test]
    fn test_display_with_tolerance() {
        let balance = Balance::builder().account(Account::from("Liabilities:US:CreditCard"))
            .amount(Amount::builder().num(Decimal::from_str("-3492.02").unwrap()).currency(Currency::from("USD")).build())
            .date(Date::from_str_unchecked("2014-12-26"))
            .build();
        
        assert_eq!(balance.to_string(), "2014-12-26 balance Liabilities:US:CreditCard -3492.02 USD");
    }
}