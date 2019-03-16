//! # Message meta parser
//!
//! [![Build Status](https://travis-ci.com/jwest/message-meta.svg?branch=master)](https://travis-ci.com/jwest/message-meta)
//! [![](https://meritbadge.herokuapp.com/message-meta)](https://crates.io/crates/message-meta)
//! 
//! Parsing message and find meta information like hashtags or links.
//!
//! ## Usage
//!
//! To use the library just add:
//!
//! ```text
//! message-meta = "^0.2"
//! ```
//!
//! to your dependencies.
//!
//! ```no_run
//! extern crate message_meta;
//!
//! fn main() {
//!   let meta = message_meta::parse("test message https://google.com/ #hashtag test");
//!     
//!   assert_eq!(meta.message, "test message https://google.com/ #hashtag test");
//!   assert_eq!(meta.links.len(), 1);
//!   assert_eq!(meta.links[0].url, "https://google.com/");
//!   assert_eq!(meta.hashtags.len(), 1);
//!   assert_eq!(meta.hashtags[0].name, "#hashtag1");
//! }
//! ```

extern crate regex;

mod links;
mod hashtags;

#[derive(Debug)]
pub struct MessageMeta {
    pub message: String,
    pub links: Vec<links::Link>,
    pub hashtags: Vec<hashtags::Hashtag>,
}

pub fn parse<T: ToString>(msg: T) -> MessageMeta {
    let message = msg.to_string();
    let links = links::parse(message.clone());
    let hashtags = hashtags::parse(message.clone());

    MessageMeta { message, links, hashtags }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn should_parse_message_without_meta() {
        let meta = parse("test message");

        assert_eq!(meta.message, "test message");
        assert_eq!(meta.links.len(), 0);
        assert_eq!(meta.hashtags.len(), 0);
    }

    #[test]
    fn should_parse_with_all_features() {
        let meta = parse("test message https://google.com/ #hashtag test");

        assert_eq!(meta.message, "test message https://google.com/ #hashtag test");
        assert_eq!(meta.links.len(), 1);
        assert_eq!(meta.links[0].url, "https://google.com/");
        assert_eq!(meta.hashtags.len(), 1);
        assert_eq!(meta.hashtags[0].name, "#hashtag");
    }
}
