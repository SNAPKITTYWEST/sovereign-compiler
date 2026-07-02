// Admissibility validator for Fontana AST
//
// Non-recursive staged traversal. No recursion in validation rules.

use crate::ast::{Declaration, Expression, Statement};

/// Validation error.
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
}

/// Admissibility validator.
pub struct AdmissibilityValidator {
    max_depth: usize,
}

impl AdmissibilityValidator {
    pub fn new() -> Self {
        Self { max_depth: 32 }
    }

    pub fn with_max_depth(max_depth: usize) -> Self {
        Self { max_depth }
    }

    /// Validate a program.
    pub fn validate_program(&self, program: &crate::ast::Program) -> Result<(), ValidationError> {
        for decl in &program.declarations {
            self.validate_declaration(decl, 0)?;
        }
        Ok(())
    }

    /// Validate a declaration (non-recursive staged traversal).
    fn validate_declaration(&self, decl: &Declaration, depth: usize) -> Result<(), ValidationError> {
        if depth > self.max_depth {
            return Err(ValidationError {
                code: "ADM001".to_string(),
                message: format!("exceeds max depth {}", self.max_depth),
            });
        }

        match decl {
            Declaration::Decl { name, body } => {
                if name.is_empty() {
                    return Err(ValidationError {
                        code: "ADM002".to_string(),
                        message: "empty declaration name".to_string(),
                    });
                }
                for stmt in body {
                    self.validate_statement(stmt, depth + 1)?;
                }
            }
            Declaration::Let { name, value } => {
                if name.is_empty() {
                    return Err(ValidationError {
                        code: "ADM003".to_string(),
                        message: "empty let binding name".to_string(),
                    });
                }
                self.validate_expression(value, depth + 1)?;
            }
            Declaration::Expr(expr) => {
                self.validate_expression(expr, depth + 1)?;
            }
            Declaration::Block(stmts) => {
                for stmt in stmts {
                    self.validate_statement(stmt, depth + 1)?;
                }
            }
        }

        Ok(())
    }

    /// Validate a statement.
    fn validate_statement(&self, stmt: &Statement, depth: usize) -> Result<(), ValidationError> {
        match stmt {
            Statement::Declaration(decl) => self.validate_declaration(decl, depth + 1),
            Statement::Expression(expr) => self.validate_expression(expr, depth + 1),
            Statement::Return(expr) => self.validate_expression(expr, depth + 1),
        }
    }

    /// Validate an expression.
    fn validate_expression(&self, expr: &Expression, depth: usize) -> Result<(), ValidationError> {
        if depth > self.max_depth {
            return Err(ValidationError {
                code: "ADM004".to_string(),
                message: "expression exceeds max depth".to_string(),
            });
        }

        match expr {
            Expression::Binary { op: _, left, right } => {
                self.validate_expression(left, depth + 1)?;
                self.validate_expression(right, depth + 1)?;
            }
            Expression::Call { name: _, args } => {
                for arg in args {
                    self.validate_expression(arg, depth + 1)?;
                }
            }
            Expression::If { cond, then_branch, else_branch } => {
                self.validate_expression(cond, depth + 1)?;
                for stmt in then_branch {
                    self.validate_statement(stmt, depth + 1)?;
                }
                if let Some(else_branch) = else_branch {
                    for stmt in else_branch {
                        self.validate_statement(stmt, depth + 1)?;
                    }
                }
            }
            Expression::Verify(inner) | Expression::Seal(inner) => {
                self.validate_expression(inner, depth + 1)?;
            }
            _ => {}
        }

        Ok(())
    }
}
