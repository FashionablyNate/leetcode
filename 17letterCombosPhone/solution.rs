pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    for digit in digits.chars() {
        let chars: Vec<char> = match digit {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _   => continue,
        };
        if ret.len() == 0 {
            // return one char for each element in chars
            ret = chars.into_iter().map(|x| x.to_string()).collect();
        } else {
            ret = ret
                // turn ret into an iterator
                .iter()
                // map x onto an iterator for each element
                .flat_map(|x| {
                    // that iterator is y mapped onto a clone of x
                    chars
                    .iter().
                    map(|y| {
                        let mut ret = x.clone();
                        ret.push(y.clone());
                        return ret;
                    })
                    // turn into a collection (a string)
                    .collect::<Vec<_>>()
                })
                // turn into a collection (a Vec<String>)
                .collect()
        }
    }
    // return result
    return ret;
}

fn main() {
    println!("{:?}", letter_combinations("3".to_string()));
}