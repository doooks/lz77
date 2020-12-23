fn main() {
    compress("the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my wayhello I was on my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way.  my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. the more reptetitive a thing is the better job we can do of compressing it the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my wayhello I was on my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way.  my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. my way to say hello to you but I missed you on the way. hello I was on my way to say hello to you but I missed you on the way. the more reptetitive a thing is the better job we can do of compressing it");

    // using string.find
    // debug 2.28s
    // release 1.02s

    // using homebrew string search
    // debug 1.3s
    // release 0.14s
}

fn compress(input: &str) {
    let max_window_len: usize = 1024;
    let mut pos: usize = 0;

    while pos < input.len() {
        let lookahead_buf = &input[pos..];
        let window_start = pos.saturating_sub(max_window_len);
        let window = &input[window_start..pos];

        let o = &find_match(&window, &lookahead_buf);
        match o {
            Encoding::Code(code) => {
                pos += code.len;
            }
            Encoding::Literal(_ch) => {
                pos += 1;
            }
        };
        println!(" {:?}", o);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Encoding {
    Literal(char),
    Code(Code),
}

#[derive(Debug, Eq, PartialEq)]
struct Code {
    dist: usize,
    len: usize,
}

fn find_match(search: &str, lookahead: &str) -> Encoding {
    let first_lookahead = lookahead.chars().next().unwrap();
    let mut code = Encoding::Literal(first_lookahead);

    let mut search = search;

    while search.len() > 0 {
        let mut lookahead_it = lookahead.chars();
        let mut match_iter = search
            .chars()
            .enumerate()
            .skip_while(|(_i, ch)| ch != &first_lookahead) // next line could use .contains
            .take_while(|(_i, search_ch)| {
                if let Some(la_ch) = lookahead_it.next() {
                    la_ch == *search_ch
                } else {
                    false
                }
            });

        if let Some((start_index, _start_ch)) = match_iter.next() {
            let match_len = match_iter.count() + 1; // we've consumed one already
            let new_code = Encoding::Code(Code {
                len: match_len,
                dist: search.len() - start_index,
            });
            match &code {
                Encoding::Code(existing_code) => {
                    if existing_code.len < match_len {
                        code = new_code;
                    }
                }
                _ => code = new_code,
            }
        }
        search = &search[1..];
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
        assert_eq!(Encoding::Literal('a'), find_match(search, lookahead));
    }

    #[test]
    fn test_trivial_match() {
        let lookahead = "a";
        let search = "a";
        assert_eq!(
            Encoding::Code(Code { dist: 1, len: 1 }),
            find_match(search, lookahead)
        );
    }

    #[test]
    fn test_find_first_match() {
        let haystack = "barbazbim";
        let needle = "baz";
        assert_eq!(
            Encoding::Code(Code { dist: 6, len: 3 }),
            find_match(haystack, needle)
        );
    }

    #[test]
    fn test_find_longest_match() {
        let haystack = "aaaaaaaaadeaaadefaaadegaaa";
        let needle = "defb";
        assert_eq!(
            Encoding::Code(Code { dist: 12, len: 3 }),
            find_match(haystack, needle)
        )
    }

    #[test]
    fn test_find_longest_match_wrong_order() {
        let haystack = "aaadefxaadeaaadefbaaadefaaa";
        let needle = "defb";
        assert_eq!(
            Encoding::Code(Code { dist: 13, len: 4 }),
            find_match(haystack, needle)
        )
    }
}
