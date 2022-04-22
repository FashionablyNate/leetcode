/*  Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

    An input string is valid if:
        Open brackets must be closed by the same type of brackets.
        Open brackets must be closed in the correct order.
 */

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for char in s.clone().chars() {
        match char {
            '(' => {
                stack.push(char);
            }
            ')' => {
                if stack.pop() == Some('(') { continue; }
                else { return false; }
            }
            '{' => {
                stack.push(char);
            }
            '}' => {
                if stack.pop() == Some('{') { continue; }
                else { return false; }
            }
            '[' => {
                stack.push(char);
            }
            ']' => {
                if stack.pop() == Some('[') { continue; }
                else { return false; }
            }
            _ => {
                return false;
            }
        }
    }
    if stack.is_empty() { return true; }
    else { return false; }
}

fn main() {
    println!("{}", is_valid("()".to_string()));
    println!("{}", is_valid("()[]{}".to_string()));
    println!("{}", is_valid("(]".to_string()));
    println!("{}", is_valid(")(".to_string()));
    println!("{}", is_valid("([)]".to_string()));
    println!("{}", is_valid("{[]}".to_string()));
}