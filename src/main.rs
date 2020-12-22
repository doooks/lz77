fn main() {
    compress("e re redcar red car goo books good book");
}

fn compress(input: &str) {
    let max_window_len: usize = 8;

    for pos in 0..input.len() {
        let lookahead_buf = &input[pos..];
        let window_start = std::cmp::max(0, pos - max_window_len);
        let window = &input[window_start..pos];
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Code {
    dist: usize,
    len: usize,
    // next: Option<char>,
}

fn find_match(search: &str, lookahead: &str) -> Code {
    let mut code = Code {
        dist: 0,
        len: 0,
        //        next: None,
    };
    println!("search is {}", search);
    println!("lookahead is {}", lookahead);
    for needle_len in 1..=lookahead.len() {
        let needle = &lookahead[0..needle_len];

        println!("searching for {:?} in {:?}", needle, search);
        if let Some(found_at) = search.find(needle) {
            code.len = needle_len;
            code.dist = search.len() - found_at;
            println!("found at {:?}", code);
        } else {
            println!("not found, anywhere");
        }
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_match() {
        let lookahead = "abc";
        let search = "xyz";
        assert_eq!(
            Code {
                dist: 0,
                len: 0,
                // next: Some('b')
            },
            find_match(lookahead, search)
        );
    }

    #[test]
    fn test_find_first_match() {
        let haystack = "foobarbaz";
        let needle = "bar";
        assert_eq!(
            Code {
                dist: 6,
                len: 3,
                //   next: None
            },
            find_match(haystack, needle)
        )
    }

    #[test]
    fn test_find_longest_match() {
        let haystack = "aaaaaaaaadeaaadefaaadegaaa";
        let needle = "defb";
        assert_eq!(
            Code {
                dist: 12,
                len: 3,
                //   next: None
            },
            find_match(haystack, needle)
        )
    }

    #[test]
    fn test_find_longest_match_wrong_order() {
        let haystack = "aaadefxaadeaaadefbaaadegaaa";
        let needle = "defb";
        assert_eq!(
            Code {
                dist: 13,
                len: 4,
                //   next: None
            },
            find_match(haystack, needle)
        )
    }
}
