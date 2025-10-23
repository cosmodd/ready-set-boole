#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(u32),
    Variable(char),
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
    fn number(n: u32) -> ASTNode {
        ASTNode::Number(n)
    }

    fn variable(c: char) -> ASTNode {
        ASTNode::Variable(c)
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
        if c.is_ascii_digit() {
            stack.push(ASTNode::Number(c.to_digit(10).unwrap()));
        } else if c.is_ascii_alphabetic() {
            stack.push(ASTNode::variable(c.to_ascii_uppercase()));
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

pub fn normalize_ast(node: &ASTNode) -> ASTNode {
    match node {
        ASTNode::BinaryOp { operator: '>', left, right} => {
            let left_norm = normalize_ast(left);
            let right_norm = normalize_ast(right);
            ASTNode::BinaryOp {
                operator: '|',
                left: Box::new(ASTNode::UnaryOp {
                    operator: '!',
                    child: Box::new(left_norm),
                }),
                right: Box::new(right_norm)
            }
        },

        ASTNode::BinaryOp { operator: '=', left, right} => {
            let left_norm = normalize_ast(left);
            let right_norm = normalize_ast(right);
            ASTNode::BinaryOp {
                operator: '&',
                left: Box::new(normalize_ast(&ASTNode::BinaryOp {
                    operator: '>',
                    left: Box::new(left_norm.clone()),
                    right: Box::new(right_norm.clone())
                })),
                right: Box::new(normalize_ast(&ASTNode::BinaryOp {
                    operator: '>',
                    left: Box::new(right_norm),
                    right: Box::new(left_norm)
                }))
            }
        }

        ASTNode::UnaryOp { operator, child } => {
            ASTNode::UnaryOp {
                operator: *operator,
                child: Box::new(normalize_ast(child))
            }
        }

        ASTNode::BinaryOp { operator, left, right } if "&|".contains(*operator) => {
            ASTNode::BinaryOp {
                operator: *operator,
                left: Box::new(normalize_ast(left)),
                right: Box::new(normalize_ast(right)),
            }
        }

        _ => node.clone()
    }
}

pub fn print_ast(root: &ASTNode, level: usize) {
    let mut prefix = "  ".repeat(level);

    if level > 0 {
        prefix = "   ".repeat(level - 1);
        prefix += "└─ ";
    }

    match root {
        ASTNode::Number(c) => { println!("{}{} (number)", prefix, c) },
        ASTNode::Variable(c) => { println!("{}{} (var)", prefix, c) },
        ASTNode::UnaryOp { operator, child } => {
            println!("{}{} (un_op)", prefix, operator);
            print_ast(child, level + 1);
        }
        ASTNode::BinaryOp { operator, left, right } => {
            println!("{}{} (bin_op)", prefix, operator);
            print_ast(right, level + 1);
            print_ast(left, level + 1);
        }
    }
}