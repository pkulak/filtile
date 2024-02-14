#[derive(PartialEq, Debug)]
pub enum Command<'a> {
    Invalid,
    Single(&'a str),
    Numeric {
        namespace: &'a str,
        operation: Operation,
        value: u32,
    },
    Textual {
        namespace: &'a str,
        value: &'a str,
    },
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Set,
}

pub fn parse_output(cmd: &str) -> Option<&str> {
    find_option("--output", cmd)
}

pub fn parse_tags(cmd: &str) -> Option<u32> {
    let tags = find_option("--tags", cmd);

    match tags {
        Some("all") => Some(0),
        Some(i) => i.parse::<u32>().ok(),
        _ => Option::None,
    }
}

pub fn parse_command(cmd: &str) -> Command {
    let parts: Vec<&str> = cmd.split(' ').collect();
    let parts = remove_options(&parts);

    if parts.len() == 1 {
        return Command::Single(parts[0]);
    };

    if parts.len() == 2 {
        let namespace = parts[0];
        let mut value = parts[1];

        let operation = if value.starts_with('+') {
            value = &value[1..];
            Operation::Add
        } else if value.starts_with('-') {
            value = &value[1..];
            Operation::Subtract
        } else {
            Operation::Set
        };

        let value: u32 = match value.parse() {
            Ok(v) => v,
            Err(_) => return Command::Textual { namespace, value },
        };

        return Command::Numeric {
            namespace,
            operation,
            value,
        };
    }

    Command::Invalid
}

// rip the first command off a string, leave the rest alone
pub fn split_commands(cmd: &str) -> (&str, Option<&str>) {
    let parts: Vec<&str> = cmd.splitn(2, ',').collect();

    let car = parts[0];

    let cdr = match parts.get(1) {
        Some(m) => {
            let trimmed = m.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }
        _ => None,
    };

    (car, cdr)
}

fn find_option<'a>(option: &'a str, cmd: &'a str) -> Option<&'a str> {
    let parts: Vec<&str> = cmd.split(' ').collect();

    if let Some(i) = parts.iter().position(|s| *s == option) {
        if let Some(next) = parts.get(i + 1) {
            return Some(next);
        }
    }

    Option::None
}

fn remove_options<'a>(parts: &[&'a str]) -> Vec<&'a str> {
    let mut ret: Vec<&str> = Vec::new();
    let mut remove_next = false;

    for item in parts.iter() {
        if item.starts_with("--") {
            remove_next = true;
        } else if remove_next {
            remove_next = false;
        } else {
            ret.push(item);
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use crate::parse::{Command, Operation};

    use super::{parse_command, parse_output, parse_tags, split_commands};

    #[test]
    fn it_splits_commands() {
        let (car, cdr) = split_commands("hi there you");

        assert_eq!(car, "hi there you");
        assert_eq!(cdr, None);

        let (car, cdr) = split_commands("first, and the second, third");

        assert_eq!(car, "first");
        assert_eq!(cdr, Some("and the second, third"));
    }

    #[test]
    fn it_parses_invalid_commands() {
        assert_eq!(
            Command::Invalid,
            parse_command("free-ice-cream for you and me")
        );
    }

    #[test]
    fn it_parses_single_commands() {
        match parse_command("flip") {
            Command::Single(v) => assert_eq!("flip", v),
            _ => panic!("parser fail"),
        };
    }

    #[test]
    fn it_ignores_options() {
        match parse_command("--output HD1 flip --tags 1") {
            Command::Single(v) => assert_eq!("flip", v),
            _ => panic!("parser fail"),
        };
    }

    #[test]
    fn it_parses_options() {
        match parse_output("--output HD1 flip") {
            Some(o) => assert_eq!("HD1", o),
            _ => panic!("parser fail"),
        }

        match parse_tags("flip --tags all") {
            Some(t) => assert_eq!(0, t),
            _ => panic!("parser fail"),
        }

        match parse_tags("flip --tags 32") {
            Some(t) => assert_eq!(32, t),
            _ => panic!("parser fail"),
        }
    }

    #[test]
    fn it_parses_numeric_commands() {
        match parse_command("outer-padding +3") {
            Command::Numeric {
                namespace: ns,
                operation: op,
                value: v,
            } => {
                assert_eq!("outer-padding", ns);
                assert_eq!(Operation::Add, op);
                assert_eq!(3, v);
            }
            _ => panic!("parser fail"),
        };

        match parse_command("inner-padding -3") {
            Command::Numeric {
                namespace: ns,
                operation: op,
                value: v,
            } => {
                assert_eq!("inner-padding", ns);
                assert_eq!(Operation::Subtract, op);
                assert_eq!(3, v);
            }
            _ => panic!("parser fail"),
        };

        match parse_command("main-ratio 75") {
            Command::Numeric {
                namespace: ns,
                operation: op,
                value: v,
            } => {
                assert_eq!("main-ratio", ns);
                assert_eq!(Operation::Set, op);
                assert_eq!(75, v);
            }
            _ => panic!("parser fail"),
        };
    }

    #[test]
    fn it_parses_textual_commands() {
        match parse_command("main-location left") {
            Command::Textual {
                namespace: ns,
                value: v,
            } => {
                assert_eq!("main-location", ns);
                assert_eq!("left", v);
            }
            _ => panic!("parser fail"),
        };
    }
}
