use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Unit(Token),
    Table(Table),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(AssignmentStatement),
}

#[derive(Debug)]
pub struct Table {
    pub elements: Vec<TableElement>,
}

#[derive(Debug)]
pub enum TableElement {
    KeyValueElement(KeyValue),
    ArrayElement(Token),
}

#[derive(Debug)]
pub struct KeyValue {
    pub key: Token,
    pub value: Expression,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub identifier: Token,
    pub value: Expression,
}
