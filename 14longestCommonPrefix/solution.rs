pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut strs_iter = strs.iter();
    let mut prev_str = strs_iter.next().unwrap_or(&"0".to_string()).to_string();
    let mut curr_str = strs_iter.next().unwrap_or(&"0".to_string()).to_string();
    
    loop {

        // if I reach the end of the vector I break so I can return the prefixed string
        if prev_str == "0".to_string() || curr_str == "0".to_string() { break; }
        
        // if there's an empty element in the vector, I return an empty string
        // since there's no common prefix
        if curr_str == "".to_string() { return "".to_string(); }

        // if the current string is bigger than the previous I need to truncate it
        // because if the current string is longer then the whole thing could be returned
        // when the beginning happens to match the previous string
        if curr_str.len() > prev_str.len() { curr_str.truncate(prev_str.len()); }

        // we loop through both strings simultaneously, and keep track of the index
        for chars in (prev_str.chars().zip(curr_str.clone().chars())).enumerate() {
            
            // println!("{} {} {}", chars.0, chars.1.0, chars.1.1);

            // chars.1.0 is a char in prev_str, cahrs.1.1 is a char in curr_str
            // if they're not equal, we've reached the end of a prefix
            if chars.1.0 != chars.1.1 {

                // if I'm at index one, then there's no common prefix since
                // not even the first char was common
                if chars.0 == 0 { return "".to_string(); }

                // otherwise we truncate the current string by our current index
                // to preserve only the common prefix
                curr_str.truncate(chars.0);
                break;
            }
        }

        // I clone the common prefix and store it in prev_str
        prev_str = curr_str.clone();
        // then fetch the next string in the iterator
        curr_str = strs_iter.next().unwrap_or(&"0".to_string()).to_string();
    }

    // return
    return prev_str;
}

fn main() {
    println!("{}", longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]));
    println!("{}", longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]));
    println!("{}", longest_common_prefix(vec!["".to_string(), "b".to_string()]));
    println!("{}", longest_common_prefix(vec!["aaa".to_string(),"aa".to_string(),"aaa".to_string()]));
    println!("{}", longest_common_prefix(vec!["abab".to_string(),"aba".to_string(),"".to_string()]));
}