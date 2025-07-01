mod passgen;

fn main() {

    let pass_length: i32 = 9;

    let uppercase_letters: bool = true;
    let letters: bool = true;
    let numbers: bool = true;
    let special_characters: bool = true;

    let final_pass = passgen::passgen(pass_length, uppercase_letters, letters, numbers, special_characters);

    println!("{}", final_pass);
}
