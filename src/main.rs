use crate::ast::{ast_to_rpn, normalize_ast, parse_boolean_rpn, print_ast};
use crate::rpn::{conjunctive_normal_form, negation_normal_form};

mod adder;
mod multiplier;
mod gray_code;
mod rpn;
mod truth_table;
mod ast;

fn main() {
    let result = adder::adder(57, 12);
    println!("57 + 12 = {}", result);

    let result = multiplier::multiplier(57, 12);
    println!("57 * 12 = {}", result);

    let result = gray_code::gray_code(57);
    println!("gray code of 57 = {}", result);

    let result = rpn::eval_formula("10|!");
    println!("rpn eval of '10|!' = {}", result);

    truth_table::print_truth_table("AZ&A!Z!&|");

    let formula = "AB=C=!";
    let ast = parse_boolean_rpn(formula).unwrap();
    print_ast(&ast, 0);
    println!("norm(\"{}\") = \"{}\"", formula, ast_to_rpn(&normalize_ast(&ast)));
    println!("NNF(\"{}\") = \"{}\"", formula, negation_normal_form(formula));
    println!("CNF(\"{}\") = \"{}\"", formula, conjunctive_normal_form(formula));
}
