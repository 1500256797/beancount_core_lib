use std::borrow::Cow;
use std::convert::TryFrom;

use typed_builder::TypedBuilder;

use crate::account::AccountType::*;
use crate::account::{Account, AccountType};
use crate::currency::Currency;
use crate::types::date::Date;
/// # Option Directive
///
/// The Option directive allows setting global options in a Beancount input file.
///
/// ## Syntax
/// ```
/// option "Name" "Value"
/// ```
///
/// ## Key Points
/// 1. Options are set using an undated "option" directive.
/// 2. Both Name and Value are strings.
/// 3. Some options set a single value, while others add to a list of existing values.
///
/// ## Viewing Options
/// 1. Refer to the updated documentation.
/// 2. Use `bean-doctor list-options` command for the installed version's options.
/// 3. Check the source code for the most up-to-date information.
///
/// ## Important Option: Operating Currency
/// - Set with: `option "operating_currency" "USD"`
/// - Declares the most common currencies used in the ledger.
/// - Used by reporting code, doesn't affect Beancount's processing or semantics.
/// - Multiple operating currencies can be declared.
///
/// ## Example
/// ```
/// option "title" "Ed's Personal Ledger"
/// option "operating_currency" "USD"
/// ```
///
/// ## Usage Notes
/// - Operating currency is crucial for useful reports and data export.
/// - Options are primarily used for customizing report generation and output.
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.e2iyrfrmstl>

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct BcOption<'a> {
    /// Name of the option.
    pub name: Cow<'a, str>,

    /// Value of the option.
    pub val: Cow<'a, str>,

    /// Source string from the parsed input
    #[builder(default)]
    pub source: Option<&'a str>,
}

impl<'a> BcOption<'a> {
    /// Determines if the current option specifies a root account name change.
    /// For example, the following line will rename the 'Assets' root account to 'Activa':
    /// ```beancount
    /// option "name_assets" "Activa"
    /// ```
    ///
    /// If this option is such a name change, this function will return the account type
    /// and the new account name. Otherwise, it will return `None`.
    pub fn root_name_change(&self) -> Option<(AccountType, String)> {
        match self.name.as_ref() {
            "name_assets" => Some((Assets, self.val.to_string())),
            "name_liabilities" => Some((Liabilities, self.val.to_string())),
            "name_equity" => Some((Equity, self.val.to_string())),
            "name_income" => Some((Income, self.val.to_string())),
            "name_expenses" => Some((Expenses, self.val.to_string())),
            _ => None,
        }
    }
}
