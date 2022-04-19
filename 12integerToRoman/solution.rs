pub fn int_to_roman(mut num: i32) -> String {
    
    // We'll build this string in the loop then return it
    let mut roman: String = "".to_string();

    // The strategy here is to take the largest roman numeral we can out of the
    // number argument. Then we decrement it by what we took out, and continue
    // to iterate on it until it's zero.
    while num > 0 {
        match num {
            1000..=3999 => {
                roman += "M"; num -= 1000;
            } 900..=999 => {
                roman += "CM"; num -= 900;
            } 500..=899 => {
                roman += "D"; num -= 500;
            } 400..=499 => {
                roman += "CD"; num -= 400;
            } 100..=399 => {
                roman += "C"; num -= 100;
            } 90..=99 => {
                roman += "XC"; num -= 90;
            } 50..=89 => {
                roman += "L"; num -= 50;
            } 40..=49 => {
                roman += "XL"; num -= 40;
            } 10..=39 => {
                roman += "X"; num -= 10;
            } 9 => {
                roman += "IX"; num -= 9;
            } 5..=8 => {
                roman += "V"; num -= 5;
            } 4 => {
                roman += "IV"; num -= 4;
            } 0..=3 => {
                roman += "I"; num -= 1;
            } _ => {
                break;
            }
        }
    }
    return roman;
}

fn main() {
    println!("{}", int_to_roman(4));
    println!("{}", int_to_roman(9));
    println!("{}", int_to_roman(58));
    println!("{}", int_to_roman(1994));
}