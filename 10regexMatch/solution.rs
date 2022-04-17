pub fn is_match(s: String, p: String) -> bool {
    return is_byte_match(s.as_bytes(), p.as_bytes());
}

fn is_byte_match(s: &[u8], p: &[u8]) -> bool {
    // call parse, which returns the pattern type and the sub pattern
    match parse(p) {
        // if there's nothing there
        (Pattern::Empty, _) => return s.is_empty(),
        // if there's no kleeny star
        (Pattern::Single(c), subp) => return is_match_single(s, c, subp),
        // if there is a kleeny star
        (Pattern::Repeatable(c), subp) => return is_match_single(s, c, p) || is_byte_match(s, subp),
    }
}

fn is_match_single(s: &[u8], to_match: u8, p: &[u8]) -> bool {
    // if s matches the char in the sub pattern, or s matches '.'
    match s.split_first() {
        Some((c, s)) if to_match == b'.' || to_match == *c => return is_byte_match(s, p),
        // else false
        _ => return false,
    }
}

// for categorizing types of patterns
enum Pattern {
    Empty,
    Single(u8),
    Repeatable(u8),
}

// takes in a pattern, then returns a snippet of it, and then the rest of the pattern
fn parse(p: &[u8]) -> (Pattern, &[u8]) {
    match p.split_first() {
        // no pattern
        None => return (Pattern::Empty, p),
        // yes pattern
        Some((c, p)) => match p.split_first() {
            // kleeny star
            Some((b'*', p)) => return (Pattern::Repeatable(*c), p),
            // some lone character
            _ => return (Pattern::Single(*c), p),
        }
    }
}

fn main() {
    println!("s = aa, p = a\n{} == false\n----------\n", is_match("aa".to_string(), "a".to_string()));
    println!("s = aa, p = a*\n{} == true\n----------\n", is_match("aa".to_string(), "a*".to_string()));
    println!("s = ab, p = .*\n{} == true\n----------\n", is_match("ab".to_string(), ".*".to_string()));
    println!("s = aab, p = c*a*b\n{} == true\n----------\n", is_match("aab".to_string(), "c*a*b".to_string()));
    println!("s = mississippi, p = mis*is*ip*.\n{} == true\n----------\n", is_match("mississippi".to_string(), "mis*is*ip*.".to_string()));
    println!("s = ab, p = .*c\n{} == false\n----------\n", is_match("ab".to_string(), ".*c".to_string()));
    println!("s = aaa, p = aaaa\n{} == false\n----------\n", is_match("aaa".to_string(), "aaaa".to_string()));
    println!("s = aaa, p = a.a\n{} == true\n----------\n", is_match("aaa".to_string(), "a.a".to_string()));
    println!("s = aa, p = aa\n{} == true\n----------\n", is_match("aa".to_string(), "aa".to_string()));
    println!("s = a, p = ab*\n{} == true\n----------\n", is_match("a".to_string(), "ab*".to_string()));
    println!("s = a, p = .\n{} == true\n----------\n", is_match("a".to_string(), ".".to_string()));
    println!("s = a, p = ab*a\n{} == false\n----------\n", is_match("a".to_string(), "ab*a".to_string()));
    println!("s = abcd, p = d*\n{} == false\n----------\n", is_match("abcd".to_string(), "d*".to_string()));
    println!("s = a, p = .*..a*\n{} == false\n----------\n", is_match("a".to_string(), ".*..a*".to_string()));
    println!("s = ab, p = .*..\n{} == true\n----------\n", is_match("ab".to_string(), ".*..".to_string()));
    println!("s = a, p = a.\n{} == false\n----------\n", is_match("a".to_string(), "a.".to_string()));
}