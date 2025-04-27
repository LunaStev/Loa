#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Text(String),
}

#[derive(Debug, Clone)]
pub enum WaveType {
    Int(u16),
    Uint(u16),
    Float(u16),
    Bool,
    Char,
    Byte,
    String,
    Pointer(Box<WaveType>),
    Array(Box<WaveType>, u32),
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Function(FunctionNode),
    Program(ParameterNode),
    Statement(StatementNode),
    Variable(VariableNode),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub parameters: Vec<ParameterNode>,
    pub return_type: Option<WaveType>,
    pub body: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct ParameterNode {
    pub name: String,
    pub param_type: WaveType,
    pub initial_value: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum FormatPart {
    Literal(String),
    Placeholder,
}

#[derive(Debug, Clone)]
pub enum Expression {
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Literal(Literal),
    Variable(String),
    Deref(Box<Expression>),
    AddressOf(Box<Expression>),
    BinaryExpression {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Grouped(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterEqual,
    LessEqual,
    Greater,
    Less,
    Equal,
    NotEqual,
    LogicalAnd,
    BitwiseAnd,
    LogicalOr,
    BitwiseOr,
    Assign,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Print(String),
    PrintFormat {
        format: String,
        args: Vec<Expression>,
    },
    Println(String),
    PrintlnFormat {
        format: String,
        args: Vec<Expression>,
    },
    Variable(String),
    If {
        condition: Expression,
        body: Vec<ASTNode>,
        else_if_blocks: Option<Box<Vec<ASTNode>>>,
        else_block: Option<Box<Vec<ASTNode>>>,
    },
    For {
        initialization: Expression,
        condition: Expression,
        increment: Expression,
        body: Vec<ASTNode>,
    },
    While {
        condition: Expression,
        body: Vec<ASTNode>,
    },
    Assign {
        variable: String,
        value: Expression,
    },
    Break,
    Continue,
    Return(Option<Expression>),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Var,
    Let,
    LetMut,
}

#[derive(Debug, Clone)]
pub struct VariableNode {
    pub name: String,
    pub type_name: WaveType,
    pub initial_value: Option<Expression>,
    pub mutability: Mutability,
}