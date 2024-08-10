use typed_builder::TypedBuilder;

use crate::account::Account;
use crate::types::date::Date;
/// # Document Directive
///
/// The Document directive attaches external files to specific accounts in the journal.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD document Account PathToDocument
/// ```ignore
///
/// ## Key Points
/// 1. Attaches external files (e.g., statements, receipts) to accounts.
/// 2. Provides clickable links in web interfaces to view documents.
/// 3. Can be manually entered or automatically generated from a directory structure.
/// 4. Helps organize and easily access financial documents.
///
/// ## Example
/// ```ignore
/// 2013-11-03 document Liabilities:CreditCard "/home/joe/stmts/apr-2014.pdf"
/// ```ignore
///
/// ## Automatic Document Generation
/// - Use the "documents" option to specify root directories:
///   ```ignore
///   option "documents" "/home/joe/stmts"
///   ```
/// - Directory structure should mirror account hierarchy.
/// - Files should be named with a date prefix (YYYY-MM-DD).
///
/// ## Usage Notes
/// - Integrates account statements and other financial documents into the ledger.
/// - Facilitates easy access to relevant documents during financial review.
/// - Can be used by scripts for automated document processing.
///
///
/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.w1ins9jk4mq3>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Document {
    /// Date the document was linked.
    pub date: Date,

    /// Account document is added to.
    pub account: Account,

    /// Filesystem path to the document.
    pub path: String,
}
