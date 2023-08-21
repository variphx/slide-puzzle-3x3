pub const PUZZLE_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Swipe {
    Up,
    Down,
    Left,
    Right,
}

impl Swipe {
    pub const fn opposite(&self) -> Self {
        match *self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordination {
    x: u8,
    y: u8,
}

impl Coordination {
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Puzzle {
    tiles: Vec<Vec<u8>>,
    empty_tile: Coordination,
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len_minus_one = PUZZLE_SIZE - 1;
        for row in self.tiles[..len_minus_one].iter() {
            for tile in row[..len_minus_one].iter() {
                write!(f, "{} ", tile + 1)?;
            }
            writeln!(f, "{}", row[len_minus_one] + 1)?;
        }
        let last_row = &self.tiles[len_minus_one];
        for tile in last_row[..len_minus_one].iter() {
            write!(f, "{} ", tile + 1)?;
        }
        write!(f, "{}", last_row[len_minus_one] + 1)?;
        Ok(())
    }
}

impl Puzzle {
    pub fn new(tiles: Vec<Vec<u8>>, empty_tile: Coordination) -> Self {
        assert_eq!(
            tiles.len(),
            PUZZLE_SIZE,
            "Puzzle's rows number is not `{}` but `{}`",
            PUZZLE_SIZE,
            tiles.len()
        );
        for row in tiles.iter() {
            assert_eq!(
                row.len(),
                PUZZLE_SIZE,
                "Puzzle's columns number is not `{}` but `{}`",
                PUZZLE_SIZE,
                row.len()
            );
        }
        assert!(
            (empty_tile.x as usize) < PUZZLE_SIZE || (empty_tile.y as usize) < PUZZLE_SIZE,
            "Empty tile's coordination is x = `{}` and y = `{}`, not in puzzle's dimension",
            empty_tile.x,
            empty_tile.y
        );

        assert_eq!(
            tiles[empty_tile.y as usize][empty_tile.x as usize],
            (PUZZLE_SIZE * PUZZLE_SIZE) as u8 - 1,
            "Empty tile's coordination: x = `{}` and y = `{}`, does not match with inputted puzzle",
            empty_tile.x,
            empty_tile.y,
        );

        Self { tiles, empty_tile }
    }

    pub fn possible_actions(&self) -> Vec<Swipe> {
        let mut possible_actions = Vec::with_capacity(4);
        let Coordination { x, y } = self.empty_tile;

        match x {
            0 => possible_actions.push(Swipe::Left),
            x if x as usize == PUZZLE_SIZE - 1 => possible_actions.push(Swipe::Right),
            _ => {
                possible_actions.push(Swipe::Left);
                possible_actions.push(Swipe::Right);
            }
        }

        match y {
            0 => possible_actions.push(Swipe::Up),
            x if x as usize == PUZZLE_SIZE - 1 => possible_actions.push(Swipe::Down),
            _ => {
                possible_actions.push(Swipe::Up);
                possible_actions.push(Swipe::Down);
            }
        }

        possible_actions
    }

    pub fn transitional_state_with_action(&self, swipe: Swipe) -> Puzzle {
        let mut new_puzzle = self.clone();
        let empty_tile = new_puzzle.empty_tile;
        let mut tile_to_move = empty_tile;

        match swipe {
            Swipe::Up => tile_to_move.y += 1,
            Swipe::Down => tile_to_move.y -= 1,
            Swipe::Left => tile_to_move.x += 1,
            Swipe::Right => tile_to_move.x -= 1,
        }

        new_puzzle.tiles[empty_tile.y as usize][empty_tile.x as usize] =
            self.tiles[tile_to_move.y as usize][tile_to_move.x as usize];
        new_puzzle.tiles[tile_to_move.y as usize][tile_to_move.x as usize] =
            (PUZZLE_SIZE * PUZZLE_SIZE) as u8 - 1;
        new_puzzle.empty_tile = tile_to_move;

        new_puzzle
    }

    pub fn is_correct(&self) -> bool {
        for (row_idx, row) in self.tiles.iter().enumerate() {
            for (col_idx, &tile) in row.iter().enumerate() {
                if ((3 * row_idx) + col_idx) as u8 != tile {
                    return false;
                }
            }
        }

        true
    }
}
