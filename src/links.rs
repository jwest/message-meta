pub struct Link {
    pub url: String,
}

pub fn parse<T: ToString>(message: T) -> Vec<Link> {
    let links = Vec::new();

    links
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn should_not_found_links() {
        let links = parse("test message");

        assert_eq!(links.len(), 0);
    }
}
