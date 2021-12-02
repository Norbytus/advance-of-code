use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum SubmarineMoveDirection {
    Forward(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, PartialEq, Eq)]
struct DirectionParseError(String);

impl FromStr for SubmarineMoveDirection {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ").into_iter();

        let first_arg = match iter.next() {
            Some(arg) => {
                match arg {
                    "forward" | "down" | "up" => arg,
                    _ => return Err(DirectionParseError(String::from("First argument must be one of 'forward, up, down'"))),
                }
            },
            None => return Err(DirectionParseError(String::from("Not found first arg"))),
        };

        let value = match iter.next() {
            Some(value) => {
                match value.parse::<i32>() {
                    Ok(value) => value,
                    Err(e) => return  Err(DirectionParseError(e.to_string())),
                }
            },
            None => return Err(DirectionParseError(String::from("Not found second value"))),
        };

        match first_arg {
            "forward" => Ok(SubmarineMoveDirection::Forward(value)),
            "up" => Ok(SubmarineMoveDirection::Up(value)),
            "down" => Ok(SubmarineMoveDirection::Down(value)),
            _ => Err(DirectionParseError(String::from("Wrong direction")))
        }
    }
}

#[test]
fn parse_forward() {
    assert_eq!(Ok(SubmarineMoveDirection::Forward(10)), SubmarineMoveDirection::from_str("forward 10"));
}

#[test]
fn parse_down() {
    assert_eq!(Ok(SubmarineMoveDirection::Down(10)), SubmarineMoveDirection::from_str("down 10"));
}

#[test]
fn parse_up() {
    assert_eq!(Ok(SubmarineMoveDirection::Up(10)), SubmarineMoveDirection::from_str("up 10"));
}

struct SubmarinePosition {
    horizontal: i32,
    vertical: i32,
}

impl SubmarinePosition {
    fn new() -> Self {
        SubmarinePosition { horizontal: 0, vertical: 0 }
    }

    fn r#move(&mut self, direction: &SubmarineMoveDirection) {
        match direction {
            &SubmarineMoveDirection::Up(value) => self.vertical += value,
            &SubmarineMoveDirection::Down(value) => self.vertical -= value,
            &SubmarineMoveDirection::Forward(value) => self.horizontal += value,
        }
    }
}

struct SubmarinePositionWithAim {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl SubmarinePositionWithAim {
    fn new() -> Self {
        SubmarinePositionWithAim { horizontal: 0, vertical: 0, aim: 0 }
    }

    fn r#move(&mut self, direction: &SubmarineMoveDirection) {
        match direction {
            &SubmarineMoveDirection::Up(value) => self.aim -= value,
            &SubmarineMoveDirection::Down(value) => self.aim += value,
            &SubmarineMoveDirection::Forward(value) => {
                self.horizontal += value;
                self.vertical += value * self.aim;
            },
        }
    }
}

pub fn calculate_v1(data: &str) -> i32 {
    let mut init_submarine_position = SubmarinePositionWithAim::new();

    for line in data.lines() {
        let command = SubmarineMoveDirection::from_str(line)
            .expect("Error while parse line to command");
        init_submarine_position.r#move(&command);
    }

    let result = init_submarine_position.horizontal * init_submarine_position.vertical;

    if result.is_negative() {
        result * -1
    } else {
        result
    }
}

pub fn calculate(data: &str) -> i32 {
    let mut init_submarine_position = SubmarinePosition::new();

    for line in data.lines() {
        let command = SubmarineMoveDirection::from_str(line)
            .expect("Error while parse line to command");
        init_submarine_position.r#move(&command);
    }

    let result = init_submarine_position.horizontal * init_submarine_position.vertical;

    if result.is_negative() {
        result * -1
    } else {
        result
    }
}

#[test]
fn test_calculate() {
    let data = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    assert_eq!(150, calculate(data));
}

#[test]
fn test_calculate_v1() {
    let data = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    assert_eq!(900, calculate_v1(data));
}
