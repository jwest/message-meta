use regex::Regex;

#[derive(Debug)]
pub struct Hashtag {
    pub name: String,
}

pub fn parse<T: ToString>(msg: T) -> Vec<Hashtag> {
    let message = msg.to_string();
    let mut hashtags: Vec<Hashtag> = Vec::new();

    let re = Regex::new(r###"\B#\w*[a-zA-Z]+\w*"###).unwrap();
 
    for caps in re.captures_iter(&*message) {
        hashtags.push(Hashtag { name: caps.get(0).unwrap().as_str().to_string() });
    }

    hashtags
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        parse_date(&s);
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck};

    use super::parse;
    use super::Hashtag;
    // @TODO https://github.com/BurntSushi/quickcheck

    fn check_parse(msg: String) -> bool {
        dbg!(&msg);
        if !msg.contains("#") {
            return parse(msg).len() == 0;
        }
        let res = parse(msg);
        if res.len() > 0 {
            return res[0].name.len() > 1 && res[0].name.get(0..1).unwrap() == "#";
        }
        return true;
    }

    #[test]
    fn should_quickcheck() {
        quickcheck::quickcheck(check_parse as fn(String) -> bool);
    }

    #[test]
    fn should_not_find_hashtag() {
        let hashtags = parse("test message");

        assert_eq!(hashtags.len(), 0);
    }

    #[test]
    fn should_find_simply_hashtag() {
        let hashtags = parse("#hashtag");

        assert_eq!(hashtags.len(), 1);
        assert_eq!(hashtags[0].name, "#hashtag");
    }

    #[test]
    fn should_not_find_hashtag_because_contains_only_numbers() {
        let hashtags = parse("#1");

        assert_eq!(hashtags.len(), 0);
    }

    #[test]
    fn should_find_hashtag_with_other_text() {
        let hashtags = parse("text https://google.com/?test#hash #hashtag text");

        assert_eq!(hashtags.len(), 1);
        assert_eq!(hashtags[0].name, "#hashtag");
    }

    #[test]
    fn should_find_two_hashtags_in_text() {
        let hashtags = parse("text #hashtag1 https://google.com/?test#hash #hashtag2 http://google.pl/ text");

        assert_eq!(hashtags.len(), 2);
        assert_eq!(hashtags[0].name, "#hashtag1");
        assert_eq!(hashtags[1].name, "#hashtag2");
    }
}
