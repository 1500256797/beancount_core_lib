use typed_builder::TypedBuilder;

use crate::amount::Amount;
use crate::currency::Currency;
use crate::types::date::Date;
/// # Price Directive
///
/// The Price directive establishes exchange rates between commodities in Beancount.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD price Commodity Price
/// ```ignore
///
/// ## Key Points
/// 1. Creates an in-memory database of prices for commodities.
/// 2. Used for various purposes, including reporting unrealized gains.
/// 3. Can be manually entered or automatically generated from postings.
/// 4. Applies to any type of commodity, including currencies and custom units.
///
/// ## Examples
/// ```ignore
/// 2014-07-09 price HOOL  579.18 USD
/// 2014-07-09 price USD   1.08 CAD
/// 2014-07-09 price VACHR 38.46 USD
/// ```ignore
///
/// ## Automatic Price Generation
/// - Use the `beancount.plugins.implicit_prices` plugin.
/// - Automatically creates Price directives from transaction postings with costs or prices.
///
/// ## Usage Notes
/// - Prices are stored per day, without intra-day time information.
/// - For multiple prices on the same day, the last one in the file is used.
/// - Useful for currency exchange rates, stock prices, and custom commodity valuations.
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.f78ym1dxtemh>
#[derive(Clone, Debug, PartialEq, TypedBuilder)]
pub struct Price {
    /// Date of the price specification.
    pub date: Date,

    /// The commodity being priced (a.k.a the base commodity).
    pub currency: Currency,

    /// Value the currency is being quoted at.
    pub amount: Amount,
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} price {} {}", self.date, self.currency, self.amount)
    }
}



#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn test_display() {
        let price = Price::builder().date(Date::from_str_unchecked("2014-07-09")).currency(Currency::from("HOOL")).amount(Amount::builder().num(Decimal::from_str("579.18").unwrap()).currency(Currency::from("USD")).build()).build();
        assert_eq!(price.to_string(), "2014-07-09 price HOOL 579.18 USD");
    }
}
