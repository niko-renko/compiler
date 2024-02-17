pub trait Parse
where
    Self: Sized,
{
    fn try_parse(string: &str) -> Result<(&str, Self), String>;

    fn parse(string: &str, allow_whitespace: bool) -> Result<(&str, Self), String> {
        if !allow_whitespace && string.chars().next().unwrap().is_whitespace() {
            return Err(String::from("Unexpected whitespace"));
        }

        let trimmed = string.trim_start();

        if trimmed.is_empty() {
            return Err(String::from("Unexpected end of string"));
        }

        Self::try_parse(trimmed)
    }

    fn parse_until<'str>(
        string: &'str str,
        until: &str,
        separator: &str,
    ) -> Result<(&'str str, Vec<Self>), String> {
        let mut next = string;
        let mut items = vec![];
        let mut first = true;

        while let Err(_) = Self::consume_string(next, until, true) {
            if first {
                first = false;
            } else {
                next = Self::consume_string(next, separator, true)?;
            }
            let (current, item) = Self::parse(next, true)?;
            next = current;
            items.push(item);
        }

        let next = Self::consume_string(next, until, true)?;
        Ok((next, items))
    }

    fn parse_while(string: &str) -> Result<(&str, Vec<Self>), String> {
        let mut current = string;
        let mut items = vec![];

        while let Ok((next, item)) = Self::parse(current, true) {
            current = next;
            items.push(item);
        }

        Ok((current, items))
    }

    fn consume(string: &str, count: usize) -> Result<&str, String> {
        if count > string.chars().count() {
            return Err(String::from("Not enough characters to consume"));
        }
        Ok(&string[count..])
    }

    fn consume_string<'from>(
        from: &'from str,
        string: &str,
        allow_whitespace: bool,
    ) -> Result<&'from str, String> {
        if string.chars().count() == 0 {
            return Err(String::from("Cannot consume empty string"));
        }

        if from.chars().count() < string.chars().count() {
            return Err(String::from("Cannot consume a larger string"));
        }

        if !allow_whitespace && from.chars().next().unwrap() != string.chars().next().unwrap() {
            return Err(String::from("Unexpected whitespace"));
        }

        let first_char = string.chars().next().unwrap();

        let first = from
            .find(|c: char| c == first_char || (!c.is_whitespace() && c != first_char))
            .unwrap();

        let trimmed = &from[first..];

        if !trimmed.starts_with(string) {
            return Err(format!("Unexpected {} while parsing {}", from, string));
        }

        Self::consume(trimmed, string.chars().count())
    }
}
