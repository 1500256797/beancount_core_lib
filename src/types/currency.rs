/// # Commodities / Currencies
///
/// Accounts contain currencies, which we sometimes also call commodities (we use both terms
/// interchangeably). Like account names, currency names are recognized by their syntax, though,
/// unlike account names, they need not be declared before being used). The syntax for a currency
/// is a word all in capital letters, like these:
///
/// ```ignore
/// USD
/// CAD
/// EUR
/// MSFT
/// IBM
/// AIRMILE
/// ```ignore
///
/// Technically, a currency name may be up to 24 characters long, and it must start with a capital
/// letter, must end with with a capital letter or number, and its other characters must only be
/// capital letters, numbers, or punctuation limited to these characters: "'._-" (apostrophe,
/// period, underscore, dash.)
///
/// The first three might evoke real world currencies to you (US dollars, Canadian dollars, Euros);
/// the next two, stock ticker names (Microsoft and IBM). And the last: rewards points (airmiles).
/// Beancount knows of no such thing; from its perspective all of these instruments are treated
/// similarly. There is no built-in notion of any previously existing currency. These currency
/// names are just names of "things" that can be put in accounts and accumulated in inventories
/// associated with these accounts.
///
/// There is something elegant about the fact that there is no "special" currency unit, that all
/// commodities are treated equally the same: Beancount is inherently a multi-currency system. You
/// will appreciate this if, like many of us, you are an expat and your life is divided between two
/// or three continents. You can handle an international ledger of accounts without any problems.
///
/// And your use of currencies can get quite creative: you can create a currency for your home,
/// for example (e.g. MYLOFT), a currency to count accumulated vacation hours (VACHR), or a
/// currency to count potential contributions to your retirement accounts allowed annually
/// (IRAUSD). You can actually solve many problems this way. The cookbook describes many such
/// concrete examples.
///
/// Beancount does not support the dollar sign syntax, e.g., "$120.00". You should always use
/// names for currencies in your input file. This makes the input more regular and is a design
/// choice. For monetary units, I suggest that you use the standard ISO 4217 currency code as a
/// guideline; these quickly become familiar. However, as detailed above, you may include some
/// other characters in currency names, like underscores (_), dashes (-), periods (.), or
/// apostrophes ('), but no spaces.
///
/// Finally, you will notice that there exists a "commodity" directive that can be used to declare
/// currencies. It is entirely optional: currencies come into being as you use them. The purpose of
/// the directive is simply to attach metadata to it.
pub type Currency = String;
