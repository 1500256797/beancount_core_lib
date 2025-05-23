use std::fmt;

use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::amount::IncompleteAmount;
use crate::flags::Flag;
use crate::metadata::Meta;

use super::position::CostSpec;
/// # Costs and Prices in Beancount
///
/// Beancount provides various ways to represent costs and prices in transactions.
///
/// ## Simple Postings
/// ```ignore
/// 2012-11-03 * "Transfer to pay credit card"
///   Assets:MyBank:Checking            -400.00 USD
///   Liabilities:CreditCard             400.00 USD
/// ```ignore
///
/// ## Postings with Price
/// Using '@' for per-unit price:
/// ```ignore
/// 2012-11-03 * "Transfer to account in Canada"
///   Assets:MyBank:Checking            -400.00 USD @ 1.09 CAD
///   Assets:FR:SocGen:Checking          436.01 CAD
/// ```ignore
///
/// Using '@@' for total price:
/// ```ignore
/// 2012-11-03 * "Transfer to account in Canada"
///   Assets:MyBank:Checking            -400.00 USD @@ 436.01 CAD
///   Assets:FR:SocGen:Checking          436.01 CAD
/// ```ignore
///
/// ## Postings with Cost
/// ```ignore
/// 2014-02-11 * "Bought shares of S&P 500"
///   Assets:ETrade:IVV                10 IVV {183.07 USD}
///   Assets:ETrade:Cash         -1830.70 USD
/// ```ignore
///
/// ## Postings with both Cost and Price
/// ```ignore
/// 2014-07-11 * "Sold shares of S&P 500"
///   Assets:ETrade:IVV               -10 IVV {183.07 USD} @ 197.90 USD
///   Assets:ETrade:Cash          1979.90 USD
///   Income:ETrade:CapitalGains
/// ```ignore
///
/// ## Balancing Rule - The "weight" of postings
/// The weight of a posting is calculated as follows:
/// 1. Amount only: Use the amount and currency as is.
/// 2. Price only: Multiply amount by price.
/// 3. Cost only: Multiply amount by cost.
/// 4. Cost and Price: Use cost for balancing, price for price database.
///
/// ## Important Notes
/// - Prices and costs are always unsigned.
/// - The sum of weights of all postings in a transaction must equal ZERO.
/// - Costs are used for inventory tracking and capital gains calculations.
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.mtqrwt24wnzs>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Posting {
    /// Account being posted to.
    pub account: Account,

    /// The amount being posted.
    pub units: IncompleteAmount,

    /// The cost of this posting.
    #[builder(default)]
    pub cost: Option<CostSpec>,

    /// The price of this posting.
    #[builder(default)]
    pub price: Option<IncompleteAmount>,

    #[builder(default)]
    pub flag: Option<Flag>,

    #[builder(default)]
    pub meta: Meta,
}

impl fmt::Display for Posting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用制表符分隔账户和单位
        let mut res = format!("{}\t{}", self.account, self.units);

        // 添加成本信息（如果有）
        if let Some(cost) = &self.cost {
            res.push_str(&format!("\t{}", cost));
        }

        // 添加价格信息（如果有）
        if let Some(price) = &self.price {
            res.push_str(&format!("\t@ {}", price));
        }

        write!(f, "{}", res)
    }
}
