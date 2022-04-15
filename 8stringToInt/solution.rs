use std::convert::TryFrom;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if (s.len() == 0) { return 0; }
        let mut prefix: bool = true;
        let mut buffer: String = "".to_string();
        let mut positive: bool = true;
        for char in s.chars() {
            match char {
                '+' => {
                    if (prefix) { buffer.push(char); }
                    else {
                        break;
                    }
                    prefix = false;
                }
                '-' => {
                    if (prefix) { buffer.push(char); positive = false; }
                    else { break; }
                    prefix = false;
                }
                '0'..='9' => {
                    buffer.push(char);
                    prefix = false;
                }
                ' ' => {
                    if (prefix) { continue; }
                    else { break; }
                }
                _ => {
                    break;
                }
            }
        }
        if (buffer.len() == 0) { buffer = "0".to_string(); }
        if ((buffer.len() == 1) & (buffer.chars().nth(0).unwrap() == '+')) { buffer = "0".to_string(); }
        
        match buffer.parse::<i32>() {
            Ok(n) => {
                return buffer.parse::<i32>().unwrap();
            },
            Err(e) => {
                if (e.to_string() == "number too large to fit in target type") {
                    let base: i32 = 2;
                    return base.pow(31) - 1;
                }
                if (e.to_string() == "number too small to fit in target type") {
                    let base: i32 = 2;
                    return base.pow(31) * -1;
                }
                return 0;
            },
        }
    }
}