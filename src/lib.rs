use typed_builder::TypedBuilder;
pub mod types;
pub use types::*;

/// # Directives
///
/// Beancount is a declarative language. The input consists of a text file containing mainly a list
/// of directives, or entries (we use these terms interchangeably in the code and documentation);
/// there is also syntax for defining various options. Each directive begins with an associated
/// date, which determines the point in time at which the directive will apply, and its type, which
/// defines which kind of event this directive represents. All the directives begin with a syntax
/// that looks like this:
///
/// ```
/// YYYY-MM-DD <type> …
/// ```
///
/// where YYYY is the year, MM is the numerical month, and DD the numerical date. All digits are
/// required, for example, the 7th of May 2007 should be "2007-05-07", including its zeros.
/// Beancount supports the international ISO 8601 standard format for dates, with dashes
/// (e.g., "2014-02-03"), or the same ordering with slashes (e.g., "2014/02/03").
///
/// Here are some example directives, just to give you an idea of the aesthetics:
///
/// ```
/// 2014-02-03 open Assets:US:BofA:Checking
/// 2014-04-10 note Assets:US:BofA:Checking "Called to confirm wire transfer."
/// 2014-05-02 balance Assets:US:BofA:Checking   154.20 USD
/// ```
///
/// The end product of a parsed input file is a simple list of these entries, in a data structure.
/// All operations in Beancount are performed on these entries.
///
/// Each particular directive type is documented in a section below.
///
/// ## Ordering of Directives
///
/// The order of declaration of the directives is not important. In fact, the entries are re-sorted
/// chronologically after parsing and before being processed. This is an important feature of the
/// language, because it makes it possible for you to organize your input file any way you like
/// without having to worry about affecting the meaning of the directives.
///
/// Except for transactions, each directive is assumed to occur at the beginning of each day. For
/// example, you could declare an account being opened on the same day as its first transaction:
///
/// ```
/// 2014-02-03 open Assets:US:BofA:Checking
///
/// 2014-02-03 * "Initial deposit"
///   Assets:US:BofA:Checking         100 USD
///   Assets:Cash                    -100 USD
/// ```
///
/// However, if you hypothetically closed that account immediately, you could not declare it closed
/// on the same day, you would have to fudge the date forward by declaring the close on 2/4:
///
/// ```
/// 2014-02-04 close Assets:US:BofA:Checking
/// ```
///
/// This also explains why balance assertions are verified before any transactions that occur on
/// the same date. This is for consistency.
#[derive(Clone, Debug, PartialEq, Default, TypedBuilder)]
pub struct Ledger<'a> {
    pub directives: Vec<directives::Directive<'a>>,
}
