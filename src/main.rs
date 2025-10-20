mod adder;
mod multiplier;
mod gray_code;

fn main() {
    let result = adder::adder(57, 12);
    println!("57 + 12 = {}", result);

    let result = multiplier::multiplier(57, 12);
    println!("57 * 12 = {}", result);

    let result = gray_code::gray_code(57);
    println!("gray code of 57 = {}", result);
}
