use crate::ast::{ast_to_rpn, normalize_ast, parse_boolean_rpn, ASTNode};

pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for c in formula.chars() {
        if c.is_numeric() {
            stack.push(c == '1');
            continue;
        }

        let a = stack.pop();
        if a.is_none() {
            println!("Invalid formula! Missing a number for '{}'", c);
            return false;
        }
        let val_a = a.unwrap();

        if c == '!' {
            stack.push(!val_a);
            continue;
        }

        let b = stack.pop();
        if b.is_none() {
            println!("Invalid formula! Missing a number for '{}'", c);
            return false;
        }
        let val_b = b.unwrap();

        match c {
            '&' => stack.push(val_a && val_b),
            '|' => stack.push(val_a || val_b),
            '^' => stack.push(val_a ^ val_b),
            '>' => stack.push(!val_a || val_b),
            '=' => stack.push(val_a == val_b),
            _ => {
                println!("Invalid formula! Unknown operation '{}'", c);
                return false;
            }
        }
    }

    stack.last().unwrap().clone()
}

fn expand_negations(node: &ASTNode) -> ASTNode {
    match node {
        ASTNode::UnaryOp { operator: '!', child } => {
            match &**child {
                ASTNode::UnaryOp { operator: '!', child: inner } => expand_negations(inner),
                ASTNode::BinaryOp { operator: '&', left, right } => {
                    ASTNode::BinaryOp {
                        operator: '|',
                        left: Box::new(expand_negations(&ASTNode::UnaryOp {
                            operator: '!',
                            child: left.clone(),
                        })),
                        right: Box::new(expand_negations(&ASTNode::UnaryOp {
                            operator: '!',
                            child: right.clone()
                        })),
                    }
                },
                ASTNode::BinaryOp { operator: '|', left, right } => {
                    ASTNode::BinaryOp {
                        operator: '&',
                        left: Box::new(expand_negations(&ASTNode::UnaryOp {
                            operator: '!',
                            child: left.clone(),
                        })),
                        right: Box::new(expand_negations(&ASTNode::UnaryOp {
                            operator: '!',
                            child: right.clone()
                        })),
                    }
                }
                _ => node.clone()
            }
        },
        ASTNode::BinaryOp { operator, left, right} => {
            ASTNode::BinaryOp {
                operator: *operator,
                left: Box::new(expand_negations(left)),
                right: Box::new(expand_negations(right)),
            }
        },
        _ => node.clone()
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let ast = parse_boolean_rpn(formula).unwrap();
    let norm = normalize_ast(&ast);
    let exp_negations = expand_negations(&norm);
    ast_to_rpn(&exp_negations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula() {
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("11=!"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("11>"), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("1011||="), true);
    }

    #[test]
    fn test_negation_normal_form() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "A!B|B!A|&");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }
}
