use core::{fmt, panic};
use std::fs::read_to_string;

pub fn distinct_positions(filename: &str) -> usize {
    let mut grid = parse_grid(filename);

    // Uncomment prints for grid visualization before/after tracing the guards movement
    // println!("{grid}");
    trace_guard(&mut grid).ok();
    // println!("{grid}");

    visited_positions(&grid)
}

pub fn obstacles_causing_a_loop(filename: &str) -> usize {
    let mut grid = parse_grid(filename);
    possible_obstacles_causing_a_loop(&mut grid).unwrap_or(0)
}

fn possible_obstacles_causing_a_loop(grid: &mut Grid) -> Result<usize, TraceError> {
    let guard_start = guard_pos(&grid)?;

    // Instead of checking every single empty position on the grid, we can
    // just check all the positions where the guard walked without the
    // additional obstacle, limiting the amount of calculations we have to do.
    let original_grid = grid.clone();
    trace_guard(grid)?;

    let mut count = 0;
    for (y, row) in grid.0.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            // Not allowed to put an obstacle at the guards starting position
            if (x, y) == guard_start {
                continue;
            }

            if let Pos::Visited(_) = pos {
                let mut new_grid = original_grid.clone();
                new_grid.0[y][x] = Pos::ExtraObstacle;

                if causes_a_loop(&mut new_grid) {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn causes_a_loop(grid: &mut Grid) -> bool {
    match trace_guard(grid) {
        Err(TraceError::Loop) => true,
        _ => false,
    }
}

fn parse_grid(filename: &str) -> Grid {
    Grid(
        read_to_string(filename)
            .unwrap_or(String::from(""))
            .lines()
            .map(|line| line.chars().map(|c| Pos::from(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

fn visited_positions(grid: &Grid) -> usize {
    grid.0.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc_row, pos| {
            if let Pos::Visited(_) = pos {
                acc_row + 1
            } else {
                acc_row
            }
        })
    })
}

fn trace_guard(grid: &mut Grid) -> Result<(), TraceError> {
    let (mut x, mut y) = guard_pos(grid)?;
    let mut dir = match &grid.0[y][x] {
        Pos::Guard(dir) => dir.clone(),
        _ => return Err(TraceError::GuardNotFound),
    };

    while let Some(new_coords) = next_coords(grid, x, y, &dir) {
        match grid.0[new_coords.1][new_coords.0] {
            Pos::Obstacle | Pos::ExtraObstacle => dir = turn_right(&dir),
            _ => {
                // There is probably a smarter way to check for a loop. We just assume that a
                // loop happens if a position is hit more than 4 times (once every direction)
                if let Pos::Visited(visit_count) = grid.0[y][x] {
                    if visit_count >= 4 {
                        return Err(TraceError::Loop);
                    }
                }

                mark_visited(grid, x, y);
                (x, y) = new_coords;
            }
        }
    }

    mark_visited(grid, x, y);
    Ok(())
}

fn guard_pos(grid: &Grid) -> Result<(usize, usize), TraceError> {
    let y = grid
        .0
        .iter()
        .position(|row| row.iter().any(|pos| is_guard(pos)))
        .ok_or(TraceError::GuardNotFound)?;

    let x = grid.0[y]
        .iter()
        .position(|pos| is_guard(pos))
        .ok_or(TraceError::GuardNotFound)?;

    Ok((x, y))
}

fn mark_visited(grid: &mut Grid, x: usize, y: usize) {
    if let Pos::Visited(visit_count) = grid.0[y][x] {
        grid.0[y][x] = Pos::Visited(visit_count + 1);
    } else {
        grid.0[y][x] = Pos::Visited(1);
    }
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

// Return the next coordinates after moving in direction or None if leaving the area
fn next_coords(grid: &mut Grid, x: usize, y: usize, dir: &Direction) -> Option<(usize, usize)> {
    let len_x = grid.0[0].len();
    let len_y = grid.0.len();

    match dir {
        Direction::Up => (y > 0).then(|| (x, y - 1)),
        Direction::Down => (y < len_y - 1).then(|| (x, y + 1)),
        Direction::Left => (x > 0).then(|| (x - 1, y)),
        Direction::Right => (x < len_x - 1).then(|| (x + 1, y)),
    }
}

fn is_guard(pos: &Pos) -> bool {
    match pos {
        Pos::Guard(_) => true,
        _ => false,
    }
}

enum TraceError {
    GuardNotFound,
    Loop,
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
enum Pos {
    NotVisited,
    Visited(i32),
    Obstacle,
    ExtraObstacle,
    Guard(Direction),
}

#[derive(Clone)]
struct Grid(Vec<Vec<Pos>>);

const CHAR_UP: char = '^';
const CHAR_DOWN: char = 'v';
const CHAR_LEFT: char = '<';
const CHAR_RIGHT: char = '>';

const CHAR_NOT_VISITED: char = '.';
const CHAR_VISITED: char = 'X';
const CHAR_OBSTACLE: char = '#';
const CHAR_EXTRA_OBSTACLE: char = 'O';

impl From<char> for Pos {
    fn from(value: char) -> Pos {
        match value {
            CHAR_NOT_VISITED => Pos::NotVisited,
            CHAR_VISITED => Pos::Visited(1),
            CHAR_OBSTACLE => Pos::Obstacle,
            CHAR_EXTRA_OBSTACLE => Pos::ExtraObstacle,
            CHAR_UP => Pos::Guard(Direction::Up),
            CHAR_DOWN => Pos::Guard(Direction::Down),
            CHAR_LEFT => Pos::Guard(Direction::Left),
            CHAR_RIGHT => Pos::Guard(Direction::Right),
            _ => panic!("Can't convert from '{value}' to Pos"),
        }
    }
}

// Display for debugging / visualization

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Direction::Up => CHAR_UP,
                Direction::Down => CHAR_DOWN,
                Direction::Left => CHAR_LEFT,
                Direction::Right => CHAR_RIGHT,
            }
        )
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pos::NotVisited => write!(f, "{CHAR_NOT_VISITED}"),
            Pos::Visited(_) => write!(f, "{CHAR_VISITED}"),
            Pos::Obstacle => write!(f, "{CHAR_OBSTACLE}"),
            Pos::ExtraObstacle => write!(f, "{CHAR_EXTRA_OBSTACLE}"),
            Pos::Guard(dir) => write!(f, "{dir}"),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for pos in row {
                write!(f, "{pos}")?
            }
            writeln!(f, "")?
        }

        Ok(())
    }
}
