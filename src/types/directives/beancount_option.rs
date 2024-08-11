use typed_builder::TypedBuilder;

use crate::account::AccountType;
use crate::account::AccountType::*;
/// # Option Directive
///
/// The Option directive allows setting global options in a Beancount input file.
///
/// ## Syntax
/// ```ignore
/// option "Name" "Value"
/// ```ignore
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
/// ```ignore
/// option "title" "Ed's Personal Ledger"
/// option "operating_currency" "USD"
/// ```ignore
///
/// ## Usage Notes
/// - Operating currency is crucial for useful reports and data export.
/// - Options are primarily used for customizing report generation and output.
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.e2iyrfrmstl>

#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct BcOption {
    /// Name of the option.
    pub name: String,

    /// Value of the option.
    pub val: String,

    /// Source string from the parsed input
    #[builder(default)]
    pub source: Option<String>,
}

impl BcOption {
    /// Determines if the current option specifies a root account name change.
    /// For example, the following line will rename the 'Assets' root account to 'Activa':
    /// ```ignorebeancount
    /// option "name_assets" "Activa"
    /// ```ignore
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


impl std::fmt::Display for BcOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "option {} {}", self.name, self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let option = BcOption::builder().name("title".to_string()).val("Ed's Personal Ledger".to_string()).build();
        assert_eq!(option.to_string(), "option title Ed's Personal Ledger");
    }
}