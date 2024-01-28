#[derive(PartialEq, Debug)]
pub enum Command<'a> {
    Invalid,
    Single(&'a str),
    Numeric {
        namespace: &'a str,
        operation: Operation,
        value: u32,
    },
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Set,
}

pub fn parse_command(cmd: &str) -> Command {
    let parts: Vec<&str> = cmd.split(' ').collect();

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
            Err(_) => return Command::Invalid,
        };

        return Command::Numeric {
            namespace,
            operation,
            value,
        };
    }

    Command::Invalid
}

#[cfg(test)]
mod tests {
    use crate::parse::{Command, Operation};

    use super::parse_command;

    #[test]
    fn it_parses_invalid_commands() {
        assert_eq!(Command::Invalid, parse_command("free-ice-cream +32.8"));
    }

    #[test]
    fn it_parses_single_commands() {
        match parse_command("swap") {
            Command::Single(v) => assert_eq!("swap", v),
            _ => panic!("parser fail"),
        };
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
}
