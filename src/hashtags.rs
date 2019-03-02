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

#[cfg(test)]
mod tests {
    use super::parse;
    // @TODO https://github.com/BurntSushi/quickcheck

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
