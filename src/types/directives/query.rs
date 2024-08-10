use std::borrow::Cow;

use typed_builder::TypedBuilder;

use crate::types::date::Date;
/// # Query Directive
///
/// The Query directive allows embedding SQL queries directly in Beancount files.
///
/// ## Syntax
/// ```
/// YYYY-MM-DD query Name SqlContents
/// ```
///
/// ## Key Points
/// 1. Experimental feature for associating SQL queries with Beancount files.
/// 2. Allows automatic running of queries as reports.
/// 3. Queries are inserted into the transaction stream.
/// 4. Each query has a name and an associated date.
///
/// ## Example
/// ```
/// 2014-07-09 query "france-balances" "
///   SELECT account, sum(position) WHERE 'trip-france-2014' in tags"
/// ```
///
/// ## Usage Notes
/// - The date specifies when the query should be run (transactions after this date are ignored).
/// - Query names may be used to invoke the query as a report type.
/// - The date acts like an implicit CLOSE in SQL terms.
///
/// ## Current Status
/// - Early development / experimental feature.

/// <https://docs.google.com/document/d/1wAMVrKIA2qtRGmoVDSUBJGmYZSygUaR0uOMW1GV3YE0/edit#heading=h.nw8fgvy4ub1w>
#[derive(Clone, Debug, Eq, PartialEq, TypedBuilder)]
pub struct Query<'a> {
    /// Date on which the query should be run.
    pub date: Date<'a>,

    /// Name of the query.
    pub name: Cow<'a, str>,

    /// Query contents.
    pub query_string: Cow<'a, str>,
}
