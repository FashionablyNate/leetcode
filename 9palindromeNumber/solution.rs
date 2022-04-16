
pub fn is_palindrome(x: i32) -> bool {
    
    // if the number is negative, it's not a palindrome,
    // if no remainder after mod 10 and it's not 0, it's not a palindrome
    if (x < 0) || (x % 10 == 0 && x != 0) { return false; }

    // initialization
    let mut stack: Vec<i32> = Vec::new();
    let mut skip_mid: bool = true;
    let base: i32 = 10;
    let mut power: i32 = base;
    let mut count: i32 = 1;

    // while x is greater than the current power
    while x >= power {
        count += 1;

        // multiplies power by 10 and checks for overflow, assigns to power
        if let Some(new_power) = power.checked_mul(base) { power = new_power; }
        else { break; }
    }
    
    // length of number is count
    let x_len: i32 = count;

    // if x is even, no need to skip the middle
    if x_len % 2 == 0 { skip_mid = false; }

    // get mutable version of x, calling it y
    let mut y: i32 = x;
    // reset count
    count = 0;
    
    // while y isn't 0
    while y != 0 {
        // skip the middle number (if odd)
        if (count == x_len / 2) & skip_mid {
            y /= 10; count += 1; continue;
            
        // otherwise not in the middle
        } else {
            
            // if we're still reversing the first half of the number
            if count < x_len / 2 {
                // push digit onto stack
                stack.push(y % 10);
                // begin getting next digit
                y /= 10; count += 1;
                
            // now we're comparing first half of number to latter half
            } else {
                // if what we pop from the stack isn't equal to this digit,
                // it's not a palindrome
                if stack.pop().unwrap() != y % 10 { return false; }
                // begin getting next digit
                y /= 10; count += 1;
            }
        }
    }
    // the latter half of the number is equal to the reversed half of the
    // beggining half, so it's a palindrome
    return true;
}

fn main() {
    println!("121");
    println!("{}", is_palindrome(121));
    println!("-----");

    println!("-121");
    println!("{}", is_palindrome(-121));
    println!("-----");

    println!("1221");
    println!("{}", is_palindrome(1221));
    println!("-----");

    println!("10");
    println!("{}", is_palindrome(10));
    println!("-----");
}