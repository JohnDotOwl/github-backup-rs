pub fn parse_next_link(link_header: &str) -> Option<String> {
    for entry in link_header.split(',') {
        let section = entry.trim();
        if !section.contains("rel=\"next\"") {
            continue;
        }

        let start = section.find('<')? + 1;
        let end = section[start..].find('>')? + start;
        return Some(section[start..end].to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::parse_next_link;

    #[test]
    fn extracts_next_link() {
        let link = "<https://api.github.com/resource?page=2>; rel=\"next\", <https://api.github.com/resource?page=4>; rel=\"last\"";
        assert_eq!(
            parse_next_link(link).as_deref(),
            Some("https://api.github.com/resource?page=2")
        );
    }
}
