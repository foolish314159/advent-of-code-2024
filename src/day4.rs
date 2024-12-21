use std::fs::read_to_string;

// XMAS

pub fn xmas_count(filename: &str) -> usize {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'X' {
                count += spells_xmas_count(&grid, x as i32, y as i32);
            }
        }
    }

    count
}

fn spells_xmas_count(grid: &Vec<Vec<char>>, x: i32, y: i32) -> usize {
    const DIRS: [i32; 3] = [-1, 0, 1];

    let mut count = 0;
    for dir_x in DIRS {
        for dir_y in DIRS {
            if dir_x == 0 && dir_y == 0 {
                continue;
            }

            if spells_xmas(&grid, x, y, dir_x, dir_y) {
                count += 1;
            }
        }
    }

    count
}

fn spells_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32, dir_x: i32, dir_y: i32) -> bool {
    let grid_len_x = grid[0].len() as i32;
    let grid_len_y = grid.len() as i32;

    const MAS: &str = "MAS";
    const MAS_LEN: i32 = MAS.len() as i32;

    if y + MAS_LEN * dir_y < 0
        || y + MAS_LEN * dir_y >= grid_len_y
        || x + MAS_LEN * dir_x < 0
        || x + MAS_LEN * dir_x >= grid_len_x
    {
        return false;
    }

    for (u, c) in MAS.chars().enumerate() {
        let i = u as i32;
        if grid[(y + dir_y * (i + 1)) as usize][(x + dir_x * (i + 1)) as usize] != c {
            return false;
        }
    }

    true
}

// X-MAS (two MAS in form of X)

pub fn x_mas_count(filename: &str) -> usize {
    let input = read_to_string(filename).unwrap_or(String::from(""));

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            // The center character for our X shape
            if c == 'A' {
                count += spells_x_mas_count(&grid, x as i32, y as i32);
            }
        }
    }

    count
}

fn spells_x_mas_count(grid: &Vec<Vec<char>>, x: i32, y: i32) -> usize {
    // Check if both diagonals spell MAS
    if spells_mas(&grid, x, y, -1, -1) && spells_mas(&grid, x, y, -1, 1) {
        return 1;
    }

    0
}

fn spells_mas(grid: &Vec<Vec<char>>, x: i32, y: i32, dir_x: i32, dir_y: i32) -> bool {
    let grid_len_x = grid[0].len() as i32;
    let grid_len_y = grid.len() as i32;

    // Needs to be atleast 1 from each border to form an X shape
    if y == 0 || y == grid_len_y - 1 || x == 0 || x == grid_len_x - 1 {
        return false;
    }

    let c1 = grid[(y + dir_y) as usize][(x + dir_x) as usize];
    let c2 = grid[(y - dir_y) as usize][(x - dir_x) as usize];

    // Forwards or backwards
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}
