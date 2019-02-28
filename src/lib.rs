extern crate regex;

mod links;

#[derive(Debug)]
pub struct MessageMeta {
    pub message: String,
    pub links: Vec<links::Link>,
}

pub fn parse<T: ToString>(msg: T) -> MessageMeta {
    let message = msg.to_string();
    let links = links::parse(message.clone());

    MessageMeta { message, links }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn should_parse_message_without_meta() {
        let meta = parse("test message");

        assert_eq!(meta.message, "test message");
        assert_eq!(meta.links.len(), 0);
    }

    #[test]
    fn should_parse_message_with_link() {
        let meta = parse("test message https://google.com/ test");

        assert_eq!(meta.message, "test message https://google.com/ test");
        assert_eq!(meta.links.len(), 1);
        assert_eq!(meta.links[0].url, "https://google.com/");
    }
}
