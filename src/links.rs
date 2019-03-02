use regex::Regex;

#[derive(Debug)]
pub struct Link {
    pub url: String,
}

pub fn parse<T: ToString>(msg: T) -> Vec<Link> {
    let message = msg.to_string();
    let mut links: Vec<Link> = Vec::new();

    let re = Regex::new(r###"(?:(?:https?|ftp)://|\b(?:[a-z\d]+\.))(?:(?:[^\s()<>]+|\((?:[^\s()<>]+|(?:\([^\s()<>]+\)))?\))+(?:\((?:[^\s()<>]+|(?:\(?:[^\s()<>]+\)))?\)|[^\s`!()\[\]{};:'".,<>?«»“”‘’]))?"###).unwrap();
 
    for caps in re.captures_iter(&*message) {
        links.push(Link { url: caps.get(0).unwrap().as_str().to_string() });
    }

    links
}

#[cfg(test)]
mod tests {
    use super::parse;
    // @TODO https://github.com/BurntSushi/quickcheck

    #[test]
    fn should_not_find_links() {
        let links = parse("test message");

        assert_eq!(links.len(), 0);
    }

    #[test]
    fn should_find_simply_link() {
        let links = parse("https://google.com");

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://google.com");
    }

    #[test]
    fn should_find_link_with_other_text() {
        let links = parse("text https://google.com/?test#hash text");

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url, "https://google.com/?test#hash");
    }

    #[test]
    fn should_find_two_links_in_text() {
        let links = parse("text https://google.com/?test#hash http://google.pl/ text");

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].url, "https://google.com/?test#hash");
        assert_eq!(links[1].url, "http://google.pl/");
    }
}
