use self::{
    balance::Balance, beancount_option::BcOption, close::Close, commodity::Commodity,
    custom::Custom, document::Document, event::Event, include::Include, note::Note, open::Open,
    pad::Pad, plugin::Plugin, prices::Price, query::Query, transaction::Transaction,
};

pub mod balance;
pub mod beancount_option;
pub mod close;
pub mod commodity;
pub mod custom;
pub mod document;
pub mod event;
pub mod include;
pub mod note;
pub mod open;
pub mod pad;
pub mod plugin;
pub mod position;
pub mod posting;
pub mod prices;
pub mod query;
pub mod transaction;

/// Enum of all directive types.
#[derive(Clone, Debug, PartialEq)]
pub enum Directive<'a> {
    Open(Open<'a>),
    Close(Close<'a>),
    Commodity(Commodity<'a>),
    Transaction(Transaction<'a>),
    Balance(Balance<'a>),
    Pad(Pad<'a>),
    Note(Note<'a>),
    Document(Document<'a>),
    Price(Price<'a>),
    Event(Event<'a>),
    Query(Query<'a>),
    Custom(Custom<'a>),
    // other directives
    Include(Include<'a>),
    Option(BcOption<'a>),
    Plugin(Plugin<'a>),
    Unsupported,
}
