fn main() {
    compress(" the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my wayhello I was on my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way.  my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. the more reptetitive a thing is the better job we can do of compressing it the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my wayhello I was on my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way.  my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. the more reptetitive a thing is the better job we can do of compressing it");

    // debug 2.28s
    // release 1.02s
}

fn compress(input: &str) {
    let max_window_len: usize = 1024;
    let mut pos: usize = 0;

    while pos < input.len() {
        let lookahead_buf = &input[pos..];
        let window_start = pos.checked_sub(max_window_len).unwrap_or(0);
        let window = &input[window_start..pos];

        let out = match find_match(&window, &lookahead_buf) {
            Some(code) => {
                pos += code.len;
                Encoding::Code(code)
            }
            None => {
                let c = lookahead_buf.chars().nth(0).unwrap();
                pos += 1;
                Encoding::Literal(c)
            }
        };
        println!("{:?}", out);
    }
}

#[derive(Debug)]
enum Encoding {
    Literal(char),
    Code(Code),
}

#[derive(Debug, Eq, PartialEq)]
struct Code {
    dist: usize,
    len: usize,
}

fn find_match(search: &str, lookahead: &str) -> Option<Code> {
    let mut code: Option<Code> = None;
    for needle_len in 1..=lookahead.len() {
        let needle = &lookahead[0..needle_len];
        if let Some(found_at) = search.find(needle) {
            code = Some(Code {
                len: needle_len,
                dist: search.len() - found_at,
            });
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
