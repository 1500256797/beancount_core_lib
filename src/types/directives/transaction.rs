use std::collections::HashSet;
use std::fmt;

use typed_builder::TypedBuilder;

use crate::flags::Flag;
use crate::metadata::{Link, Tag};
use crate::types::date::Date;

use super::posting::Posting;

/// # Transaction Directive
///
/// Transactions are the most common type of directives in a Beancount ledger.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD [txn|Flag] [[Payee] Narration]
///    [Flag] Account       Amount [{Cost}] [@ Price]
///    [Flag] Account       Amount [{Cost}] [@ Price]
///    ...
/// ```ignore
///
/// ## Key Points
/// 1. Can use 'txn' keyword or a flag (* or !) instead.
/// 2. Flags indicate transaction status (* for completed, ! for needs revision).
/// 3. Can have multiple postings (account entries) per transaction.
/// 4. The sum of all posting amounts must equal zero.
/// 5. Amounts can be arithmetic expressions.
///
/// ## Examples
/// ```ignore
/// 2014-05-05 * "Cafe Mogador" "Lamb tagine with wine"
///   Liabilities:CreditCard:CapitalOne         -37.45 USD
///   Expenses:Restaurant
///
/// 2014-03-19 * "Acme Corp" "Bi-monthly salary payment"
///   Assets:MyBank:Checking             3062.68 USD
///   Income:AcmeCorp:Salary            -4615.38 USD
///   Expenses:Taxes:TY2014:Federal       920.53 USD
///   ...
///
/// 2014-10-05 * "Costco" "Shopping for birthday"
///   Liabilities:CreditCard:CapitalOne         -45.00          USD
///   Assets:AccountsReceivable:John            ((40.00/3) + 5) USD
///   Assets:AccountsReceivable:Michael         40.00/3         USD
///   Expenses:Shopping
/// ```ignore
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.up4dj751q84w>
#[derive(Clone, Debug, PartialEq, TypedBuilder)]
pub struct Transaction {
    pub date: Date,

    /// Whether or not a transaction is considered complete.
    ///
    /// `*` or `txn`: Completed transaction, known amounts, “this looks correct.”
    /// `!`: Incomplete transaction, needs confirmation or revision, “this looks incorrect.”
    #[builder(default=Flag::Okay)]
    pub flag: Flag,

    /// Payee of this transaction.
    #[builder(default)]
    pub payee: Option<String>,

    /// # Payee & Narration
    ///
    /// A transaction may have an optional "payee" and/or a "narration."
    ///
    /// ## Payee
    /// The payee is a string that represents an external entity involved in the transaction.
    /// Payees are useful for transactions posting amounts to Expense accounts, accumulating
    /// expenses from multiple businesses under one category.
    ///
    /// ## Narration
    /// A narration is a description of the transaction that you write. It can include context,
    /// accompanying persons, product details, or any other relevant information.
    ///
    /// ## Syntax Examples
    ///
    /// 1. Single string becomes narration:
    ///    ```ignore
    ///    2014-05-05 * "Lamb tagine with wine"
    ///       ...
    ///    ```ignore
    ///
    /// 2. Setting just a payee (with empty narration):
    ///    ```ignore    
    ///    2014-05-05 * "Cafe Mogador" ""
    ///       ...
    ///    ```ignore
    ///
    /// 3. Legacy syntax with pipe symbol (to be removed in future):
    ///    ```ignore
    ///    2014-05-05 * "Cafe Mogador" | ""
    ///       ...
    ///    ```ignore
    ///
    /// 4. Transaction with neither payee nor narration (flag required):
    ///    ```ignore
    ///    2014-05-05 *
    ///       ...
    ///    ```ignore
    ///
    /// ## Note for Ledger Users
    /// Unlike Ledger, which has a single field referred to by the "Payee" metadata tag,
    /// Beancount's Transaction object has separate payee and narration fields.
    ///
    /// For detailed discussion on using payees, refer to "Payees, Subaccounts, and Assets".
    pub narration: String,

    /// Tags associated with the transaction.
    #[builder(default)]
    pub tags: HashSet<Tag>,

    /// Links associated with the transactions.
    #[builder(default)]
    pub links: HashSet<Link>,

    /// Postings belonging to this transaction.
    #[builder(default)]
    pub postings: Vec<Posting>,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = format!("{}", self.date);
        res.push_str(&format!(" {}", self.flag));
        if let Some(payee) = &self.payee {
            res.push_str(&format!(" {}", payee));
        }
        // add "" for narration

        res.push_str(&format!(" \"{}\"", self.narration));

        // add tags
        if !self.tags.is_empty() {
            res.push_str(&format!(
                " {}",
                self.tags
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        // add links
        if !self.links.is_empty() {
            res.push_str(&format!(
                " {}",
                self.links
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }
        // new line
        res.push('\n');

        for posting in &self.postings {
            res.push_str(&format!("  {}", posting));
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}
