mod adder;
mod multiplier;

fn main() {
    let result = adder::adder(57, 12);
    println!("57 + 12 = {}", result);

    let result = multiplier::multiplier(57, 12);
    println!("57 * 12 = {}", result);
}
