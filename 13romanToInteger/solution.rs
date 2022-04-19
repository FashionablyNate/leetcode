use std::str::Chars;

pub fn roman_to_int(s: String) -> i32 {
    let s_chars: Chars = s.chars();
    let mut last_char: char = ' ';
    let mut ret: i32 = 0;

    // The strategy is to add the corresponding value to a roman numeral when
    // I read it in. I store the previous number, that way I can subtract double
    // it's value when it preceeds a larger number. This enables the proper
    // reading of IV, IX, XL, etc.
    for char in s_chars {
        match char {
            'M' => { ret += 1000; if last_char == 'C' { ret -= 200; }},
            'D' => { ret += 500; if last_char == 'C' { ret -= 200; }},
            'C' => { ret += 100; if last_char == 'X' { ret -= 20; }},
            'L' => { ret += 50; if last_char == 'X' { ret -= 20; }},
            'X' => { ret += 10; if last_char == 'I' { ret -= 2; }},
            'V' => { ret += 5; if last_char == 'I' { ret -= 2; }},
            'I' => { ret += 1 },
             _  => break,
        }
        last_char = char;
    }
    return ret;
}

fn main() {
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("LVIII".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}