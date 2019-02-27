mod links;

pub struct MessageMeta {
    pub message: String,
    pub links: Vec<links::Link>,
}

pub fn parse<T: ToString>(message: T) -> MessageMeta {
    let msg = message.to_string();
    let links = links::parse(msg.clone());

    MessageMeta { message: msg.to_string(), links }
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
}
