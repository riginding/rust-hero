use std::fmt;

fn main() {}

#[derive(Debug)]
struct Segment(i32, char);

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Segments(pub Vec<Segment>);

impl fmt::Display for Segments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, counter| {
            result.and_then(|_| write!(f, "{}", counter))
        })
    }
}

fn rust_hero_solve(input: &str) -> String {
    let mut result: Vec<Segment> = vec![];
    let mut iter = input.chars();
    let mut found: Option<char> = None;
    let mut counter: i32 = 0;
    while let Some(char) = iter.next() {
        if found.is_some() && char == found.unwrap() {
            counter = counter + 1;
        } else {
            if found.is_some() && counter > 0 {
                let s = Segment(counter, found.unwrap());
                result.push(s)
            }
            found = Some(char);
            counter = 1;
        }
    }
    if found.is_some() && counter > 0 {
        let s = Segment(counter, found.unwrap());
        result.push(s)
    }

    Segments(result).to_string()
}

fn recursive_super_duper_function(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    if chars.len() == 0 {
        return String::from("");
    } else {
        let value = chars[0];
        let length = chars.len();
        let mut i = 1;
        while i < length && value == chars[i] {
            i += i
        }
        let count: u8 = i as u8;
        let mut new_string = count.to_string();
        new_string.push(value);

        let rest = &chars[i..length].into_iter().collect::<String>();
        new_string.push_str(&recursive_super_duper_function(rest));

        new_string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hero_solve() {
        assert_eq!(rust_hero_solve("222"), String::from("32"))
    }

    #[test]
    fn hero_solve2() {
        assert_eq!(rust_hero_solve("111222333"), String::from("313233"))
    }

    #[test]
    fn sample1() {
        assert_eq!(recursive_super_duper_function("11"), String::from("21"))
    }

    #[test]
    fn sample2() {
        assert_eq!(recursive_super_duper_function("22"), String::from("22"))
    }

    #[test]
    fn sample3() {
        assert_eq!(
            recursive_super_duper_function("22333344155"),
            String::from("2243241125")
        )
    }
}
