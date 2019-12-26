#[macro_use] extern crate text_io;


fn main() {
    println!("{}", conversion::scanning());
}

pub mod conversion {

    const morse_alphabet: [&str; 26] = ["*-", "-***", "-*-*", "-**", "*", "**-*", "--*", "****", "**", "*---", "-*-", "*-**", "--", "-*", "---", "*--*", "--*-", "*-*", "***", "-", "**-", "***-", "*--", "-**-", "-*--", "--**"];
    const morse_numbers: [&str; 10] = ["-----", "*----", "**---", "***--", "****-", "*****", "-****", "--***", "---**", "----*"];

    pub fn scanning() -> String {
        //Scanning for input until a new line is created
        let word: String = read!("{}\n");
        if word.chars().next() == Some('*') || word.chars().next() == Some('-') {
            //Convert to not morse
            return conversion_main_to_not_morse(word);
        }
        else {
            //Convert to morse
            return conversion_main_to_morse(word);
        }
    }
    
    fn fetch_morse(input: char) -> String {
        let mut value = input as u32;

        // Logic for outputs in Morse

        if input >= 'a' && input <= 'z' {
            value -= 'a' as u32;
            return String::from(morse_alphabet[value as usize]);
        }
        else if input == ' ' {
            return String::from("       ");
        }
        else if input >= '0' && input <= '9' {
            value -= '0' as u32;
            return String::from(morse_numbers[value as usize])
        }
        else if input == '`' {
            return String::from(" ");
        }
        else {
            return String::from("");
        }

        //value -= 'a' as u32; 

    }
    pub fn conversion_main_to_morse(input: String) -> String{
        let mut output: String = "".to_string();

        for ch in input.chars() {
            let indiv_char = ch.to_ascii_lowercase();
            output.push_str(&fetch_morse(indiv_char));
            if ch != ' ' {
                output.push_str(&fetch_morse('`'));
            }
        }

        return output;
    }

    fn human_characters_conv(input: String) -> char { //Converting everything to Human characters
        use std::char;
        let mut output: char = ' ';
        if input.chars().count() == 5 {
            //number
            for mut x in 0..morse_numbers.len() {
                if input == morse_numbers[x] {
                    output = char::from_digit(x as u32, 36).unwrap_or(' ');
                }
            }
        }
        else {
            //letters
            for mut x in 0..morse_alphabet.len() {
                if input == morse_alphabet[x] {
                    x += 10;
                    output = char::from_digit(x as u32, 36).unwrap_or(' ');
                }
            }
        }
        return output;
    }

    pub fn conversion_main_to_not_morse(input: String) -> String{
        let mut output: String = "".to_string();
        let mut empty: String = "".to_string();

        for ch in input.chars() {
            if ch == ' ' {
                output.push(human_characters_conv(empty));
                empty = "".to_string();
            }
            else {
                empty.push(ch);
            }
        }
        output.push(human_characters_conv(empty));
        return output;
    }
}