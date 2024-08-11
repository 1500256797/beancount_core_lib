use core::fmt;

use typed_builder::TypedBuilder;

/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.17ry42rqbuiu>
#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum AccountType {
    Assets,
    Liabilities,
    Equity,
    Income,
    Expenses,
}

impl AccountType {
    pub fn default_name(&self) -> &'static str {
        use AccountType::*;
        match self {
            Assets => "Assets",
            Liabilities => "Liabilities",
            Equity => "Equity",
            Income => "Income",
            Expenses => "Expenses",
        }
    }
}

impl From<&str> for AccountType {
    fn from(s: &str) -> Self {
        match s {
            "Assets" => AccountType::Assets,
            "Liabilities" => AccountType::Liabilities,
            "Equity" => AccountType::Equity,
            "Income" => AccountType::Income,
            "Expenses" => AccountType::Expenses,
            _ => panic!("Invalid account type: {}", s),
        }
    }
}

// account type to String
impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.default_name())
    }
}

/// Accounts
///
/// Beancount accumulates commodities in accounts. The names of these accounts do not have to be
/// declared before being used in the file, they are recognized as "accounts" by virtue of their
/// syntax alone. An account name is a colon-separated list of capitalized words which begin with
/// a letter, and whose first word must be one of five account types:
///
/// - Assets
/// - Liabilities
/// - Equity
/// - Income
/// - Expenses
///
/// Each component of the account names begin with a capital letter or a number and are followed
/// by letters, numbers or dash (-) characters. All other characters are disallowed.
///
/// Here are some realistic example account names:
///
/// - Assets:US:BofA:Checking
/// - Liabilities:CA:RBC:CreditCard
/// - Equity:Retained-Earnings
/// - Income:US:Acme:Salary
/// - Expenses:Food:Groceries
///
/// The set of all names of accounts seen in an input file implicitly define a hierarchy of
/// accounts (sometimes called a chart-of-accounts), similarly to how files are organized in a
/// file system. For example, the following account names:
///
/// - Assets:US:BofA:Checking
/// - Assets:US:BofA:Savings
/// - Assets:US:Vanguard:Cash
/// - Assets:US:Vanguard:RGAGX
/// - Assets:Receivables
///
/// implicitly declare a tree of accounts that looks like this:
///
/// ```ignore
/// `-- Assets
///     |-- Receivables
///     `-- US
///         |-- BofA
///         |   |-- Checking
///         |   `-- Savings
///         `-- Vanguard
///             |-- Cash
///             `-- RGAG
/// ```ignore
///
/// 我们可以说"Assets:US:BofA"是"Assets:US:BofA:Checking"的父账户，而后者是前者的子账户。
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.17ry42rqbuiu>
#[derive(Clone, Debug, Eq, PartialEq, Hash, TypedBuilder)]
pub struct Account {
    /// Type of the account.
    pub account_type: AccountType,
    /// parts of the account following the account type.
    #[builder(default)]
    pub parts: Vec<String>,
}
// "Assets:US:BofA:Checking" => AccountType::Assets, vec!["US", "BofA", "Checking"]
impl From<&str> for Account {
    fn from(s: &str) -> Self {
        let parts: Vec<String> = s.split(":").map(|s| s.to_string()).collect();
        let account_type = AccountType::from(parts[0].as_str());
        let parts = parts[1..].to_vec();
        Account {
            account_type,
            parts,
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.parts.is_empty() {
            write!(f, "{}", self.account_type)
        } else {
            write!(f, "{}:{}", self.account_type, self.parts.join(":"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_name() {
        assert_eq!(AccountType::Assets.default_name(), "Assets");
        assert_eq!(AccountType::Liabilities.default_name(), "Liabilities");
        assert_eq!(AccountType::Equity.default_name(), "Equity");
        assert_eq!(AccountType::Income.default_name(), "Income");
        assert_eq!(AccountType::Expenses.default_name(), "Expenses");
    }

    #[test]
    fn test_new_account() {
        let account = Account::builder()
            .account_type(AccountType::Assets)
            .parts(vec![
                "US".to_string(),
                "BofA".to_string(),
                "Checking".to_string(),
            ])
            .build();
        assert_eq!(account.account_type, AccountType::Assets);
        assert_eq!(
            account.parts,
            vec!["US".to_string(), "BofA".to_string(), "Checking".to_string()]
        );
    }

    #[test]
    fn test_new_account_with_default_parts() {
        let account = Account::builder().account_type(AccountType::Assets).build();
        assert_eq!(account.account_type, AccountType::Assets);
        assert_eq!(account.parts, Vec::<String>::new());
    }

    #[test]
    fn test_account_type_to_str() {
        let res = AccountType::Assets.to_string();
        assert_eq!(res, "Assets");
    }

    #[test]
    fn test_account_to_str() {
        let account = Account::builder()
            .account_type(AccountType::Assets)
            .parts(vec![
                "US".to_string(),
                "BofA".to_string(),
                "Checking".to_string(),
            ])
            .build();
        assert_eq!(account.to_string(), "Assets:US:BofA:Checking");

        let account = Account::builder().account_type(AccountType::Assets).build();
        assert_eq!(account.to_string(), "Assets");
    }
}
