pub fn is_palindrome(x: i32) -> bool {

    if x < 0 { return false; }

    let mut stack: Vec<i32> = Vec::new();
    let mut skip_mid: bool = true;
    let x_len: i32 = length(x, 10);

    if x_len % 2 == 0 { skip_mid = false; }

    let mut y: i32 = x;
    let mut count: i32 = 0;
    while y != 0 {
        if (count == x_len / 2) & skip_mid {
            y /= 10; count += 1; continue;
        } else {
            if count < x_len / 2 {
                stack.push(y % 10);
                y /= 10; count += 1;
            } else {
                if stack.pop().unwrap() != y % 10 { return false; }
                y /= 10; count += 1;
            }
        }
    }
    return true;
}

pub fn length(n: i32, base: i32) -> i32 {
    
    let mut power = base;
    let mut count = 1;

    while n >= power {
        count += 1;

        if let Some(new_power) = power.checked_mul(base) { power = new_power; }
        else { break; }
    }

    return count;
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