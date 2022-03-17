use std::{fmt::Display, str::FromStr};

pub enum PlaceError {
    Taken,
}

#[derive(Debug)]
pub struct Game {
    board: [Option<Player>; 9],
    turn: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [None; 9],
            turn: Player::O,
        }
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn get_result(&self) -> GameResult {
        if self.is_player_win(Player::O) {
            GameResult::Win(Player::O)
        } else if self.is_player_win(Player::X) {
            GameResult::Win(Player::X)
        } else if self.is_board_full() {
            GameResult::Draw
        } else {
            GameResult::Undetermined
        }
    }

    pub fn place(&mut self, point: &Point) -> Result<(), PlaceError> {
        if let Some(_) = self.get_player_at_point(point) {
            Err(PlaceError::Taken)
        } else {
            self.set_player_at_point(point);
            Ok(())
        }
    }

    pub fn proceed(&mut self) {
        self.turn = match self.turn {
            Player::O => Player::X,
            Player::X => Player::O,
        }
    }

    fn is_board_full(&self) -> bool {
        self.board.into_iter().all(|x| x.is_some())
    }

    fn is_player_win(&self, player: Player) -> bool {
        let check = self.board.map(|x| x.is_some() && x.unwrap() == player);

        (check[0] && check[1] && check[2])
            || (check[3] && check[4] && check[5])
            || (check[6] && check[7] && check[8])
            || (check[0] && check[3] && check[6])
            || (check[1] && check[4] && check[7])
            || (check[2] && check[5] && check[8])
            || (check[0] && check[4] && check[8])
            || (check[2] && check[4] && check[6])
    }

    fn get_player_at_point(&self, point: &Point) -> Option<Player> {
        self.board[point.x + point.y * 3]
    }

    fn set_player_at_point(&mut self, point: &Point) {
        self.board[point.x + point.y * 3] = Some(self.turn);
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        self.board
            .map(|x| match x {
                None => "_",
                Some(Player::O) => "O",
                Some(Player::X) => "X",
            })
            .windows(3)
            .step_by(3)
            .map(|x| x.join("  "))
            .fold(String::new(), |acc, x| acc + "\n\n" + &x)
    }
}

pub enum GameResult {
    Win(Player),
    Draw,
    Undetermined,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    O,
    X,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::O => write!(f, "Player O"),
            Player::X => write!(f, "Player X"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PointError {
    BadLen,
    OutOfRange,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Result<Point, PointError> {
        let is_point_valid = (0..=2).contains(&x) && (0..=2).contains(&y);
        if is_point_valid {
            Ok(Point { x, y })
        } else {
            Err(PointError::OutOfRange)
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = (65 + self.x as u8) as char;
        let y = self.y + 1;
        write!(f, "{}{}", x, y)
    }
}

impl FromStr for Point {
    type Err = PointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [col, row] = s.chars().collect::<Vec<_>>()[..] {
            let col = match col {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => return Err(PointError::OutOfRange),
            };
            let row = match row {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                _ => return Err(PointError::OutOfRange),
            };

            Point::new(col, row)
        } else {
            Err(PointError::BadLen)
        }
    }
}

mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn point_valid() {
        let point = Point::from_str("A1");
        assert!(point.is_ok());
        assert_eq!(point.unwrap(), Point { x: 0, y: 0 });
    }

    #[test]
    fn point_out_of_range() {
        let point = Point::from_str("D1");
        assert!(point.is_err());
        assert_eq!(point.unwrap_err(), PointError::OutOfRange);
    }

    #[test]
    fn point_bad_len() {
        let point = Point::from_str("A");
        assert!(point.is_err());
        assert_eq!(point.unwrap_err(), PointError::BadLen);
    }
}
