// AST definitions for Fontana declarations

use serde::{Deserialize, Serialize};

/// Declaration types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Declaration {
    /// Simple declaration: decl name { ... }
    Decl {
        name: String,
        body: Vec<Statement>,
    },

    /// Let binding: let x = expr
    Let {
        name: String,
        value: Expression,
    },

    /// Expression statement
    Expr(Expression),

    /// Block: { ... }
    Block(Vec<Statement>),
}

/// Statement types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
    Return(Expression),
}

/// Expression types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal(Literal),

    /// Identifier
    Ident(String),

    /// Binary operation
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Function call
    Call {
        name: String,
        args: Vec<Expression>,
    },

    /// If expression
    If {
        cond: Box<Expression>,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },

    /// Verify expression
    Verify(Box<Expression>),

    /// Seal expression
    Seal(Box<Expression>),
}

/// Literal values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Integer(u64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

/// Binary operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
}

/// Program: list of declarations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
