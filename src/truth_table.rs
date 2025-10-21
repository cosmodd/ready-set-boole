use crate::rpn::eval_formula;

pub fn print_truth_table(formula: &str) {
    let mut letters: Vec<char> = Vec::new();

    for c in formula.chars() {
        if c >= 'A' && c <= 'Z' {
            letters.push(c);
        }
    }

    let letters_count = letters.len();
    if letters_count < 1 {
        println!("Need at least one letter to generate truth table!");
        return;
    }

    let mut columns = letters.clone();
    columns.push('=');

    let header = columns
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" | ");
    println!("| {} |", header);

    println!("|{}", "---|".repeat(letters.len() + 1));

    for i in 0..(1 << letters_count) {
        let mut row: Vec<u32> = Vec::new();
        let mut row_formula = formula.to_string();

        for it in (0..letters_count).rev().zip(letters.iter()) {
            let (j, letter) = it;
            let bit = (i >> j) & 1;

            row.push(bit);
            row_formula = row_formula.replace(*letter, bit.to_string().as_str());
        }

        row.push(eval_formula(&row_formula) as u32);

        println!("| {} |", row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" | "));
    }
}
