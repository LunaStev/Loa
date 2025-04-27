use std::collections::HashMap;
use parser::ast::*;

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Float(f64),
    String(String),
    Bool(bool),
    None,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, ast: &[ASTNode]) {
        for node in ast {
            self.execute_node(node);
        }
    }

    fn execute_node(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Statement(stmt) => self.execute_statement(stmt),
            _ => {}
        }
    }

    fn execute_statement(&mut self, stmt: &StatementNode) {
        match stmt {
            StatementNode::PrintArgs(args) => {
                for expr in args {
                    let value = self.evaluate_expression(expr);
                    match value {
                        Value::Number(n) => println!("{}", n),
                        Value::Float(f) => println!("{}", f),
                        Value::String(s) => println!("{}", s),
                        Value::Bool(b) => println!("{}", b),
                        Value::None => println!("None"),
                    }
                }
            }
            StatementNode::Assign { variable, value } => {
                let val = self.evaluate_expression(value);
                self.variables.insert(variable.clone(), val);
            }
            StatementNode::While { condition, body } => {
                while let Value::Bool(true) = self.evaluate_expression(condition) {
                    self.execute(body);
                }
            }
            StatementNode::Break => {
                // 나중에 추가할 수 있음
            }
            StatementNode::Continue => {
                // 나중에 추가할 수 있음
            }
            StatementNode::Return(_) => {
                // 나중에 추가할 수 있음
            }
            _ => {}
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Value {
        match expr {
            Expression::Literal(lit) => match lit {
                Literal::Number(n) => Value::Number(*n),
                Literal::Float(f) => Value::Float(*f),
                Literal::String(s) => Value::String(s.clone()),
            },
            Expression::Variable(name) => {
                self.variables.get(name).cloned().unwrap_or(Value::None)
            }
            Expression::BinaryExpression { left, operator, right } => {
                let l = self.evaluate_expression(left);
                let r = self.evaluate_expression(right);
                self.evaluate_binary_op(l, operator, r)
            }
            _ => Value::None,
        }
    }

    fn evaluate_binary_op(&self, l: Value, op: &Operator, r: Value) -> Value {
        match (l, r) {
            (Value::Number(a), Value::Number(b)) => match op {
                Operator::Add => Value::Number(a + b),
                Operator::Subtract => Value::Number(a - b),
                Operator::Multiply => Value::Number(a * b),
                Operator::Divide => Value::Number(a / b),
                Operator::Less => Value::Bool(a < b),
                Operator::Greater => Value::Bool(a > b),
                Operator::Equal => Value::Bool(a == b),
                Operator::NotEqual => Value::Bool(a != b),
                _ => Value::None,
            },
            _ => Value::None,
        }
    }
}
