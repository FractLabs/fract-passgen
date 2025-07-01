use rand::random_range;

pub fn passgen(pass_length: i32, uppercase_letters: bool, letters: bool, numbers: bool, special_characters: bool) -> String {

    let spechar: [char; 6] = ['!', '@', '#', '$', '%', '&'];
    let mut times: i32 = 0;
    let mut final_pass = String::new();

    loop {
        let ranw:i32 = random_range(0..=3);
        

        times += 1;
        if ranw == 0 && numbers {
            let ranber: i32 = random_range(0..=9);
            final_pass.push_str(&ranber.to_string());
        } else if ranw == 1 && letters {
            let rancar: char = random_range('a'..='z');
            final_pass.push(rancar);
        } else if ranw == 2 && special_characters {
            let rannumspe = random_range(0..6);
            final_pass.push(spechar[rannumspe]);
            
        } else if ranw == 3 && uppercase_letters {
            let rancarm: char = random_range('a'..='z');
            final_pass.push(rancarm.to_uppercase().next().unwrap());
        }

        if times >= pass_length {
            break;
        }
    }
    return final_pass;
}