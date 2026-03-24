use std::collections::HashMap;
use std::fs;
use std::ops::Deref;

const START_CHAR: char = 'S';

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn step(&self) -> Position {
        match self {
            Direction::N => (1, 0),
            Direction::E => (0, 1),
            Direction::S => (-1, 0),
            Direction::W => (0, -1),
        }
    }
}

type Position = (isize, isize);

#[derive(Debug)]
struct Tile {
    raw_char: char,
    pipe: Option<(Direction, Direction)>,
}

impl TryFrom<&char> for Tile {
    type Error = String;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Ok(Tile {
            raw_char: *c,
            pipe: Tile::build_pipe(c)?,
        })
    }
}

impl Tile {
    fn build_pipe(
        c: &char,
    ) -> Result<Option<(Direction, Direction)>, <Tile as TryFrom<&char>>::Error> {
        match c {
            '|' => Ok(Some((Direction::N, Direction::S))),
            '-' => Ok(Some((Direction::E, Direction::W))),
            'L' => Ok(Some((Direction::N, Direction::E))),
            'J' => Ok(Some((Direction::N, Direction::W))),
            '7' => Ok(Some((Direction::S, Direction::W))),
            'F' => Ok(Some((Direction::S, Direction::E))),
            '.' => Ok(None),
            'S' => Ok(None),
            default => Err(format!("Unrecognized pipe character: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Map(HashMap<Position, Tile>);

impl Deref for Map {
    type Target = HashMap<Position, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Map {
    type Error = String;

    fn try_from(filename: &str) -> Result<Self, Self::Error> {
        let contents: String = fs::read_to_string(filename).map_err(|e| e.to_string())?;
        let lines: Vec<&str> = contents.split("\n").collect();

        let mut m = HashMap::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, tile_char) in line.chars().enumerate() {
                m.insert((row as isize, col as isize), Tile::try_from(&tile_char)?);
            }
        }

        Ok(Map(m))
    }
}

impl Map {
    fn find_start(&self) -> Result<Position, String> {
        for (pos, tile) in self.iter() {
            if tile.raw_char == START_CHAR {
                return Ok(*pos);
            }
        }
        Err("Could not find start position".to_owned())
    }

    fn step(&self, start: &Position, entered_from_dir: &Direction) -> Result<Position, String> {
        let tile = self
            .get(&start)
            .ok_or_else(|| format!("No position {:?}", start))?;

        let pipe = tile
            .pipe
            .as_ref()
            .ok_or_else(|| format!("Position {:?} has no pipe: \"{}\"", start, tile.raw_char))?;

        if &pipe.0 != entered_from_dir && &pipe.1 != entered_from_dir {
            return Err(format!(
                "Can't have entered pipe {:?} from {:?}",
                pipe, entered_from_dir
            ));
        }

        let dir_to_step: Direction = match entered_from_dir {
            _ if entered_from_dir == &pipe.0 => pipe.1,
            _ if entered_from_dir == &pipe.1 => pipe.0,
            _ => {
                return Err(format!(
                    "Entered from a direction ({:?}) not supported by the pipe {:?}",
                    entered_from_dir, pipe
                ))
            }
        };

        let pos_delta: Position = dir_to_step.step();
        Ok((start.0 - pos_delta.0, start.1 - pos_delta.1))
    }
}

fn main() {
    let map = Map::try_from("src/test.txt").expect("Could not load map");
    println!("{:?}", map);
    println!("start: {:?}", map.find_start());

    let start_pos = map.find_start();
    println!("{:?}", start_pos);

    let step = map.step(&(2 as isize, 1 as isize), &Direction::W);
    println!("stepped to {:?}", step);
}
