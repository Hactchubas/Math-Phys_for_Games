mod structs;
mod tests;

use structs::binary::Binary;

fn main() {
    println!("");

    let value: &str = "100110";
    if Binary::is_valid_binary(value) == true {
        let bin_1 = Binary::new(value);
        println!("{:?}", bin_1)
    }
 
}
