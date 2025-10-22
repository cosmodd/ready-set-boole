#[derive(Debug)]
pub enum ASTNode {
    Token(char),
    UnaryOp {
        operator: char,
        child: Box<ASTNode>
    },
    BinaryOp {
        operator: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>
    }
}

impl ASTNode {
    fn token(n: char) -> Self {
        ASTNode::Token(n)
    }

    fn unary_op(operator: char, child: Self) -> Self {
        ASTNode::UnaryOp {
            operator,
            child: Box::new(child)
        }
    }

    fn binary_op(operator: char, left: Self, right: Self) -> Self {
        ASTNode::BinaryOp {
            operator,
            left: Box::new(left),
            right: Box::new(right)
        }
    }
}

pub fn parse_boolean_rpn(formula: &str) -> Option<ASTNode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for c in formula.chars() {
        if c.is_ascii_alphanumeric() {
            stack.push(ASTNode::token(c.to_ascii_uppercase()));
        } else if "!".contains(c) {
            let child = stack.pop()?;
            stack.push(ASTNode::unary_op(c, child));
        } else if "&|^>=".contains(c) {
            let right = stack.pop()?;
            let left = stack.pop()?;
            stack.push(ASTNode::binary_op(c, left, right));
        }
    }

    if stack.is_empty() {
        None
    } else {
        Some(stack.pop().unwrap())
    }
}

pub fn print_ast(root: &ASTNode, level: usize) {
    let mut prefix = "  ".repeat(level);

    if level > 0 {
        prefix = "   ".repeat(level - 1);
        prefix += "└─ ";
    }

    match root {
        ASTNode::Token(c) => { println!("{}{}", prefix, c) },
        ASTNode::UnaryOp { operator, child } => {
            println!("{}{}", prefix, operator);
            print_ast(child, level + 1);
        }
        ASTNode::BinaryOp { operator, left, right } => {
            println!("{}{}", prefix, operator);
            print_ast(right, level + 1);
            print_ast(left, level + 1);
        }
    }
}