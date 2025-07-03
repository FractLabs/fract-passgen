mod passgen;

fn main() {

    let pass_length: i32 = 9;

    let uppercase_letters: bool = false;
    let letters: bool = false;
    let numbers: bool = false;
    let special_characters: bool = false;

    let final_pass = passgen::passgen(pass_length, uppercase_letters, letters, numbers, special_characters);

    println!("{}", final_pass);
}
