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
}
