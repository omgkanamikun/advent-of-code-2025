use anyhow::{Context, bail};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
enum DirectionParseError {
    #[error("unsupported direction '{0}'")]
    Unsupported(char),
}

#[derive(Debug, Error)]
enum RotationCommandParseError {
    #[error("empty input")]
    EmptyInput,

    #[error("invalid direction '{dir}' in '{input}'")]
    InvalidDirection {
        input: String,
        dir: char,
        #[source]
        source: DirectionParseError,
    },

    #[error("missing distance in '{input}'")]
    MissingDistance { input: String },

    #[error("invalid distance '{distance}' in '{input}'")]
    InvalidDistance {
        input: String,
        distance: String,
        #[source]
        source: std::num::ParseIntError,
    },
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn get_direction_literal(&self) -> &'static str {
        match self {
            Direction::Left => "L",
            Direction::Right => "R",
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = DirectionParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            other => Err(DirectionParseError::Unsupported(other)),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_direction_literal())
    }
}

#[derive(Debug)]
struct RotationCommand {
    direction: Direction,
    distance: i32,
}

impl RotationCommand {
    fn parse(input: &str) -> anyhow::Result<Self, RotationCommandParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(RotationCommandParseError::EmptyInput);
        }
        let mut chars = input.chars();
        let dir_ch = chars.next().ok_or(RotationCommandParseError::EmptyInput)?;

        let direction = Direction::try_from(dir_ch).map_err(|e| {
            RotationCommandParseError::InvalidDirection {
                input: input.to_string(),
                dir: dir_ch,
                source: e,
            }
        })?;

        let distance_str = chars.as_str();
        if distance_str.is_empty() {
            return Err(RotationCommandParseError::MissingDistance {
                input: input.to_string(),
            });
        }

        let distance: i32 =
            distance_str
                .parse()
                .map_err(|e| RotationCommandParseError::InvalidDistance {
                    input: input.to_string(),
                    distance: distance_str.to_string(),
                    source: e,
                })?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

impl Display for RotationCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}

struct SafeDialKnob {
    current_position: i32,
    zero_position_occurrence: u32,
}

impl Default for SafeDialKnob {
    fn default() -> Self {
        SafeDialKnob {
            current_position: 50,
            zero_position_occurrence: 0,
        }
    }
}

impl SafeDialKnob {
    fn init() -> Self {
        SafeDialKnob::default()
    }

    fn rotate_knob_solution_two(&mut self, command: &RotationCommand) {
        let mut current: i32 = self.current_position;
        let direction = &command.direction;
        let mut steps: i32 = command.distance;

        while steps != 0 {
            current = match direction {
                Direction::Right => (current + 1) % 100,
                Direction::Left => (current - 1) % 100,
            };

            if current == 0 {
                self.zero_position_occurrence += 1;
            }

            steps -= 1;
        }
        self.current_position = current;
    }

    fn rotate_knob_solution_one(&mut self, command: &RotationCommand) {
        let mut current: i32 = self.current_position;
        let direction = &command.direction;
        let steps: i32 = command.distance;

        current = match direction {
            Direction::Right => (current + steps) % 100,
            Direction::Left => (current - steps) % 100,
        };

        if current == 0 {
            self.zero_position_occurrence += 1;
        }

        self.current_position = current;
    }

    fn apply_rotation_commands_solution_two(&mut self, commands: &[RotationCommand]) {
        commands
            .iter()
            .for_each(|command| self.rotate_knob_solution_two(command));
    }

    fn apply_rotation_commands_solution_one(&mut self, commands: &[RotationCommand]) {
        commands
            .iter()
            .for_each(|command| self.rotate_knob_solution_one(command));
    }

    fn get_code_sequence(self) -> u32 {
        self.zero_position_occurrence
    }
}

/// --- Day 1: Secret Entrance ---
///
/// The Elves have good news and bad news.
///
/// The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.
///
/// The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!
///
/// To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
/// You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:
///
/// "Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."
///
/// The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.
///
/// The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.
///
/// So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.
///
/// Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.
///
/// So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.
///
/// The dial starts by pointing at 50.
///
/// You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.
///
/// For example, suppose the attached document contained the following rotations:
///
/// L68
/// L30
/// R48
/// L5
/// R60
/// L55
/// L1
/// L99
/// R14
/// L82
///
/// The following these rotations would cause the dial to move as follows:
///
/// The dial starts by pointing at 50.
/// The dial is rotated L68 to a point at 82.
/// The dial is rotated L30 to a point at 52.
/// The dial is rotated R48 to a point at 0.
/// The dial is rotated L5 to a point at 95.
/// The dial is rotated R60 to a point at 55.
/// The dial is rotated L55 to a point at 0.
/// The dial is rotated L1 to a point at 99.
/// The dial is rotated L99 to a point at 0.
/// The dial is rotated R14 to a point at 14.
/// The dial is rotated L82 to a point at 32.
/// Because the dial points at 0 a total of three times during this process, the password in this example is 3.
/// Analyze the rotations in your attached document. What's the actual password to open the door?
///
/// --- Part Two ---
///
/// You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.
///
/// As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:
///
/// "Due to newer security protocols, please use password method 0x434C49434B until further notice."
///
/// You remember from the training seminar that "method 0x434C49434B" means you're actually supposed to count the number of times any click causes the dial to point at 0, regardless of whether it happens during a rotation or at the end of one.
///
/// Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:
///
/// The dial starts by pointing at 50.
/// The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
/// The dial is rotated L30 to a point at 52.
/// The dial is rotated R48 to a point at 0.
/// The dial is rotated L5 to a point at 95.
/// The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
/// The dial is rotated L55 to a point at 0.
/// The dial is rotated L1 to a point at 99.
/// The dial is rotated L99 to a point at 0.
/// The dial is rotated R14 to a point at 14.
/// The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.
/// In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be 6.
///
/// Be careful: if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!
///
/// Using password method 0x434C49434B, what is the password to open the door?
fn main() -> anyhow::Result<()> {
    let rotation_commands =
        load_rotation_commands("puzzle_input").with_context(|| "failed in main")?;

    if rotation_commands.is_empty() {
        bail!("no commands to execute");
    }

    let mut safe_knob = SafeDialKnob::init();
    safe_knob.apply_rotation_commands_solution_one(&rotation_commands);

    println!(
        "The code for the fist puzzle, solution one is: {}",
        safe_knob.get_code_sequence()
    );

    let mut safe_knob = SafeDialKnob::init();
    safe_knob.apply_rotation_commands_solution_two(&rotation_commands);

    println!(
        "The code for the first puzzle, solution two is: {}",
        safe_knob.get_code_sequence()
    );

    Ok(())
}

fn load_rotation_commands(file_name: &str) -> anyhow::Result<Vec<RotationCommand>> {
    let puzzle_input = read_input_file(input_path(file_name))?;
    let mut converted: Vec<RotationCommand> = Vec::new();
    for entry in puzzle_input {
        let element = RotationCommand::parse(&entry)
            .with_context(|| format!("failed to parse rotation command '{entry}'"))?;
        converted.push(element);
    }
    Ok(converted)
}

fn read_input_file(input_path: PathBuf) -> anyhow::Result<Vec<String>> {
    let lines = read_files_lines(input_path)?;
    let mut puzzle_input: Vec<String> = Vec::new();
    for line in lines {
        puzzle_input.push(line?);
    }
    Ok(puzzle_input)
}

fn input_path(file_name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join(file_name)
}

fn read_files_lines<P: AsRef<Path>>(filename: P) -> anyhow::Result<Lines<BufReader<File>>> {
    let path = filename.as_ref();
    let file = File::open(path)
        .with_context(|| format!("failed to open input file {}", path.display()))?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_command_right_direction() {
        let r = RotationCommand::parse("R12").unwrap();
        assert_eq!(r.direction, Direction::Right);
        assert_eq!(r.distance, 12);
    }

    #[test]
    fn test_rotation_command_left_direction() {
        let r = RotationCommand::parse("L21").unwrap();
        assert_eq!(r.direction, Direction::Left);
        assert_eq!(r.distance, 21);
    }

    #[test]
    fn test_invalid_rotation_commands() {
        assert!(matches!(
            RotationCommand::parse("").unwrap_err(),
            RotationCommandParseError::EmptyInput
        ));
        assert!(matches!(
            RotationCommand::parse("X99").unwrap_err(),
            RotationCommandParseError::InvalidDirection { .. }
        ));
        assert!(matches!(
            RotationCommand::parse("R").unwrap_err(),
            RotationCommandParseError::MissingDistance { .. }
        ));
        assert!(matches!(
            RotationCommand::parse("Rabc").unwrap_err(),
            RotationCommandParseError::InvalidDistance { .. }
        ));
    }

    #[test]
    fn test_read_input() {
        let test_puzzle_input = load_rotation_commands("test_input").unwrap();
        assert!(!test_puzzle_input.is_empty());
    }

    #[test]
    fn test_solution_one_small_puzzle_input() {
        let first_expected_answer = 3;
        let test_puzzle_input = load_rotation_commands("test_input").unwrap();
        let mut safe = SafeDialKnob::init();
        safe.apply_rotation_commands_solution_one(&test_puzzle_input);

        assert_eq!(first_expected_answer, safe.get_code_sequence());
    }

    #[test]
    fn test_solution_two_small_puzzle_input() {
        let second_expected_answer = 6;
        let test_puzzle_input = load_rotation_commands("test_input").unwrap();
        let mut safe = SafeDialKnob::init();
        safe.apply_rotation_commands_solution_two(&test_puzzle_input);

        assert_eq!(second_expected_answer, safe.get_code_sequence());
    }

    #[test]
    fn test_solution_one_puzzle_input() {
        let first_star_answer = 1135;
        let test_puzzle_input = load_rotation_commands("puzzle_input").unwrap();
        let mut safe = SafeDialKnob::init();
        safe.apply_rotation_commands_solution_one(&test_puzzle_input);

        assert_eq!(first_star_answer, safe.get_code_sequence());
    }

    #[test]
    fn test_solution_two_puzzle_input() {
        let second_start_answer = 6558;
        let test_puzzle_input = load_rotation_commands("puzzle_input").unwrap();
        let mut safe = SafeDialKnob::init();
        safe.apply_rotation_commands_solution_two(&test_puzzle_input);

        assert_eq!(second_start_answer, safe.get_code_sequence());
    }
}
