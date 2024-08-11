use std::fmt;

use rust_decimal::Decimal;
use typed_builder::TypedBuilder;

use crate::{amount::Amount, currency::Currency, types::date::Date};

/// # Reducing Positions
///
/// When you post a reduction to a position in an account, the reduction must always match an existing lot.
/// For example, if an account holds 3200 USD and a transaction posts a -1200 USD change to that account,
/// the 1200 USD match against the existing 3200 USD, and the result is a single position of 2000 USD.
/// This also works for negative values. For example, if an account has a -1300 USD balance and you post
/// a +2000 USD change to it, you obtain a 700 USD balance.
///
/// A change posted to an account, regardless of the account type, can result in a positive or negative balance;
/// there are no limitations on the balances of simple commodity amounts (that is, those with no cost associated
/// to them). For example, while Assets accounts normally have a positive balance and Liabilities accounts
/// usually a negative one, you can legally credit an Assets account to a negative balance, or debit a Liabilities
/// account to a positive balance. This is because in the real world these things do happen: you might write a
/// check too many and obtain temporary credit from your bank's checking account (usually along with an outrageous
/// "overdraft" fee), or pay that credit card balance twice by mistake.
///
/// For commodities held at cost, the cost specification of the posting must match one of the lots held in the
/// inventory before the transaction is applied. The list of lots is gathered, and matched against the specification
/// in the {...} part of the posting. For example, if you provide a cost, only those lots whose cost match that
/// will remain. If you provide a date, only those lots which match that date will remain. And you can use a label
/// as well. If you provide a cost and a date, both of these are matched against the list of candidate lots to reduce.
/// This is essentially a filter on the list of lots.
///
/// If the filtered list results in a single lot, that lot is chosen to be reduced. If the list results in multiple
/// lots, but the total amount being reduced equals the total amount in the lots, all those lots are reduced by that posting.
///
/// For example, if in the past you had the following transactions:
/// ```ignore
/// 2014-02-11 * "Bought shares of S&P 500"
///   Assets:ETrade:IVV                20 IVV {183.07 USD, "ref-001"}
///   …
/// 2014-03-22 * "Bought shares of S&P 500"
///   Assets:ETrade:IVV                15 IVV {187.12 USD}
///   …
/// ```ignore
/// Each of the following reductions would be unambiguous:
/// ```ignore
/// 2014-05-01 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -20 IVV {183.07 USD}
///   …
/// 2014-05-01 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -20 IVV {2014-02-11}
///   …
/// 2014-05-01 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -20 IVV {"ref-001"}
///   …
/// 2014-05-01 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -35 IVV {}
///   …
/// ```ignore
/// However, the following would be ambiguous:
/// ```ignore
/// 2014-05-01 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -20 IVV {}
///   …
/// ```ignore
/// If multiple lots match against the reducing posting and their number is not the total number, we are in a
/// situation of ambiguous matches. What happens then, is that the account's booking method is invoked. There are
/// multiple booking methods, but by default, all accounts are set to use the "STRICT" booking method. This method
/// simply issues an error in an ambiguous situation.
///
/// You may set the account's booking method to "FIFO" to instruct Beancount to select the oldest of the lots.
/// Or "LIFO" for the latest (youngest) of the lots. This will automatically select all the necessary matching
/// lots to fulfill the reduction.
///
/// # Note
/// Requiring the dates to match will be dealt with more sensibly in the near future. See A Proposal for an
/// Improvement on Inventory Booking for details of this upcoming change.
///
/// For such postings, a change that results in a negative number of units is usually impossible. Beancount does
/// not currently allow holding a negative number of a commodity held at cost. For example, an input with just
/// this transaction will fail:
/// ```ignore
/// 2014-05-23 *
///   Assets:Investments:MSFT        -10 MSFT {43.40 USD}
///   Assets:Investments:Cash     434.00 USD
/// ```ignore
/// If it did not, this would result in a balance of -10 units of MSFT. On the other hand, if the account had a
/// balance of 12 units of MSFT held at 43.40 USD on 5/23, the transaction would book just fine, reducing the
/// existing 12 units to 2. Most often, the error that will occur is that the account will be holding a balance
/// of 10 or more units of MSFT at a different cost, and the user will specify an incorrect value for the cost.
/// For instance, if the account had a positive balance of 20 MSFT {42.10 USD}, the transaction above would still
/// fail, because there aren't 10 or more units of MSFT at 43.40 USD to remove from.
///
/// This constraint is enforced for a few reasons:
/// 1. Mistakes in data entry for stock units are not uncommon, they have an important negative impact on the
///    correctness of your Ledger—the amounts are usually large—and they will typically trigger an error from
///    this constraint. Therefore, the error check is a useful way to detect these types of errors.
/// 2. Negative numbers of units held at cost are fairly rare. Chances are you don't need them at all. Exceptions
///    include: short sales of stock, holding spreads on futures contracts, and depending on how you account for
///    them, short positions in currency trading.
///
/// This is why this check is enabled by default.
///
/// # Note
/// In a future version of Beancount, we will relax this constraint somewhat. We will allow an account to hold a
/// negative number of units of a commodity if and only if there are no other units of that commodity held in the
/// account. Either that, or we will allow you to mark an account as having no such constraints at all. The purpose
/// is to allow the account of short positions in commodities. The only blocking factor is this constraint.
///
/// For more details of the inventory booking algorithm, see the How Inventories Work document.
#[derive(Clone, Debug, Eq, PartialEq, Hash, TypedBuilder)]
pub struct Cost {
    pub number: Decimal,
    pub currency: Currency,
    pub date: Date,
    pub label: Option<String>,
}

// TODO: Important Note. Amounts specified as either per-share or total prices or costs are always
// unsigned. It is an error to use a negative sign or a negative cost and Beancount will raise an
// error if you attempt to do so.

/// Represents a "cost", which typically belongs to a [Posting](struct.Posting.html).
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.mtqrwt24wnzs>
#[derive(Clone, Debug, Eq, PartialEq, Hash, TypedBuilder)]
pub struct CostSpec {
    #[builder(default)]
    pub number_per: Option<Decimal>,
    #[builder(default)]
    pub number_total: Option<Decimal>,
    /// The type of commodity for this cost.
    #[builder(default)]
    pub currency: Option<Currency>,
    /// The date of the at-cost.
    #[builder(default)]
    pub date: Option<Date>,
    /// The label of the cost.
    #[builder(default)]
    pub label: Option<String>,
    /// Flag to indicate that all lots should be merged and average cost to be used
    #[builder(default)]
    pub merge_cost: bool,
}

impl fmt::Display for CostSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = format!("{}", self.number_per.unwrap_or(Decimal::default()));
        if let Some(currency) = &self.currency {
            res.push_str(&format!(" {}", currency));
        }
        write!(f, "{}", res)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, TypedBuilder)]
pub struct Position {
    pub units: Amount,
    pub cost: Option<Cost>,
}
