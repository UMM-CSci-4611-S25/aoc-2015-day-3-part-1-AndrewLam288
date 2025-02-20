use std::{collections::HashSet, hash::Hash, ops::Add, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {x: i32, y: i32}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {x,y}
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c:char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => return Err(IllegalChar(c)),
        })
    }
}


pub struct Moves {
    moves: Vec<Direction>,
}

impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        Ok(Self { moves })
    }
}


pub struct SantaTracker {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

impl SantaTracker {
    #[must_use]
    pub fn new() -> Self {
        let initial_position = Pos::new(0,0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_position);
        
        Self {
            visited_houses,
            current_position: initial_position,
        }
    }

    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    #[must_use]
    pub const fn current_pos(&self) -> Pos {
        self.current_position
    }

    pub fn perform_move(&mut self, direction: Direction) {
        let new_position = self.current_position + direction;
        self.current_position = new_position;
        self.visited_houses.insert(new_position);
    }

    pub fn perform_moves(&mut self, moves: Moves) {
        for m in moves.moves {
            self.perform_move(m);
        }
    }

}

impl Default for SantaTracker {
    fn default() -> Self {
        Self::new()
    }
}


fn main() {
    let input_file_name = "input.txt";
    let file_contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    let moves = Moves::from_str(file_contents.trim()).expect("Failed to parse the input file to list of moves");
    let mut santa_tracker = SantaTracker::new();
    santa_tracker.perform_moves(moves);
    println!(
        "Santa visited and delivered presents to {} houses",
        santa_tracker.num_visited_houses()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_tracker_new() {
        let santa_tracker = SantaTracker::new();
        assert_eq!(santa_tracker.num_visited_houses(), 1);
        assert_eq!(santa_tracker.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_move_east() {
        let mut santa_tracker = SantaTracker::new();
        santa_tracker.perform_move(Direction::East);
        assert_eq!(santa_tracker.num_visited_houses(), 2);
        assert_eq!(santa_tracker.current_pos(), Pos::new(1, 0));
    }

    #[test]
    fn test_square_moves() {
        let mut santa_tracker = SantaTracker::new();
        let moves = Moves::from_str("^>v<").unwrap();
        santa_tracker.perform_moves(moves);
        assert_eq!(santa_tracker.num_visited_houses(), 4);
        assert_eq!(santa_tracker.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_up_down_moves() {
        let mut santa_tracker = SantaTracker::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        santa_tracker.perform_moves(moves);
        assert_eq!(santa_tracker.num_visited_houses(), 2);
        assert_eq!(santa_tracker.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_aoc_input() {
        let mut santa_tracker = SantaTracker::new();
        let moves = Moves::from_str(include_str!("../input.txt").trim()).unwrap();
        santa_tracker.perform_moves(moves);
        assert_eq!(santa_tracker.num_visited_houses(), 2565);
    }
}
