use std::convert::TryFrom;

use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::currency::Currency;
use crate::types::date::Date;

/// # Open Directive
///
/// All accounts need to be declared "open" before they can accept postings.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD open Account [ConstraintCurrency,...]  ["BookingMethod"]
/// ```ignore
///
/// Example:
/// ```ignore
/// 2014-05-01 open Liabilities:CreditCard:CapitalOne     USD
/// ```ignore
///
/// ## Key Points
/// 1. Optional list of constraint currencies enforces that all postings to this account
///    are in the specified currencies.
///
/// 2. The date of the Open directive must precede or be the same as the date of the first
///    transaction posting to that account.
///
/// 3. The order of declarations in the file is not important; only the dates matter.
///
/// ## Booking Method
/// An optional declaration for opening accounts, specifying how to handle ambiguous
/// lot matching:
///
/// - STRICT: Lot specification must match exactly one lot (default).
/// - NONE: No lot matching is performed, accepts lots of any price.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.omdgvaikswd0>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Open {
    /// Date the account was opened.
    pub date: Date,

    /// Account being opened.
    pub account: Account,

    /// Commodities allowed for the opened account. An empty list means no restrictions on the
    /// allowed commodities.
    #[builder(default)]
    pub currencies: Vec<Currency>,

    /// Booking method. The default booking method for accounts is
    /// [`Booking::Strict`](enum.Booking.html).
    #[builder(default)]
    pub booking: Option<Booking>,
}


impl std::fmt::Display for Open {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} open {} {}", self.date, self.account, self.currencies.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "))
    }
}



/// The set of booking methods for positions on accounts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Booking {
    /// Reject ambiguous matches with an error.
    Strict,

    /// Strict booking method, but disambiguate further with sizes. Reject ambiguous matches with
    /// an error but if a lot matches the size exactly, accept it the oldest.
    StrictWithSize,

    /// Disable matching and accept the creation of mixed inventories.
    None,

    /// Average cost booking: merge all matching lots before and after.
    Average,

    /// First-in first-out in the case of ambiguity.
    Fifo,

    /// Last-in first-out in the case of ambiguity.
    Lifo,
}

impl TryFrom<&str> for Booking {
    type Error = ();

    fn try_from(val: &str) -> Result<Self, Self::Error> {
        match val {
            "STRICT" => Ok(Booking::Strict),
            "STRICT_WITH_SIZE" => Ok(Booking::StrictWithSize),
            "NONE" => Ok(Booking::None),
            "AVERAGE" => Ok(Booking::Average),
            "FIFO" => Ok(Booking::Fifo),
            "LIFO" => Ok(Booking::Lifo),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let open = Open::builder().date(Date::from_str_unchecked("2014-05-01")).account(Account::from("Liabilities:CreditCard:CapitalOne")).currencies(vec![Currency::from("USD")]).build();
        assert_eq!(open.to_string(), "2014-05-01 open Liabilities:CreditCard:CapitalOne USD");
    }
}

