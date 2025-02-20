pub mod create;
pub mod insert;
pub mod select;

pub use create::parse_create_table;
pub use insert::parse_insert;
pub use select::parse_select;

#[derive(Debug)]
pub enum ParserError {
    InvalidSyntax(String),
}

#[derive(Debug, Clone)]
pub enum Query {
    CreateTable { table_name: String, columns: Vec<String> },
    Insert { table_name: String, values: Vec<String> },
    Select {
        table_name: String,
        select_columns: Vec<String>,
        condition: Option<Condition>,
    },
    // ここにUpdate, Delete, Joinなどを後々拡張
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum Operator {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

/// The main entry point for parsing a query. This function dispatches to the appropriate
/// parser based on the query prefix.
pub fn parse_query(query: &str) -> Result<Query, ParserError> {
    let trimmed = query.trim();
    let upper = trimmed.to_uppercase();
    if upper.starts_with("CREATE TABLE") {
        parse_create_table(trimmed)
    } else if upper.starts_with("INSERT INTO") {
        parse_insert(trimmed)
    } else if upper.starts_with("SELECT") {
        parse_select(trimmed)
    } else {
        Err(ParserError::InvalidSyntax("Unknown command".into()))
    }
}