use typed_builder::TypedBuilder;

use crate::types::date::Date;
/// # Query Directive
///
/// The Query directive allows embedding SQL queries directly in Beancount files.
///
/// ## Syntax
/// ```ignore
/// YYYY-MM-DD query Name SqlContents
/// ```ignore
///
/// ## Key Points
/// 1. Experimental feature for associating SQL queries with Beancount files.
/// 2. Allows automatic running of queries as reports.
/// 3. Queries are inserted into the transaction stream.
/// 4. Each query has a name and an associated date.
///
/// ## Example
/// ```ignore
/// 2014-07-09 query "france-balances" "
///   SELECT account, sum(position) WHERE 'trip-france-2014' in tags"
/// ```ignore
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
pub struct Query {
    /// Date on which the query should be run.
    pub date: Date,

    /// Name of the query.
    pub name: String,

    /// Query contents.
    pub query_string: String,
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} query \"{}\" \"{}\"", self.date, self.name, self.query_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let query = Query::builder().date(Date::from_str_unchecked("2014-07-09")).name("france-balances".to_string()).query_string("SELECT account, sum(position) WHERE 'trip-france-2014' in tags".to_string()).build();
        assert_eq!(query.to_string(), r#"2014-07-09 query "france-balances" "SELECT account, sum(position) WHERE 'trip-france-2014' in tags""#);
    }
}