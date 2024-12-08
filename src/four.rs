const XMAS_STR: &str = "XMAS";

pub fn run() {
    let xmas: &[u8] = XMAS_STR.as_bytes();

    let file = std::fs::read_to_string("inputs/four.txt").unwrap();

    let mut lines = file.split('\n').collect::<Vec<&str>>();
    lines.pop();

    let mut count: usize = 0;

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect();

    let dirs: Vec<i32> = vec![-1, 0, 1];

    println!("Grid 0 length: {}", grid[0].len());

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                continue;
            }

            for dir_row in &dirs {
                for dir_col in &dirs {
                    if *dir_row == 0 || *dir_col == 0 {
                        continue;
                    }
                    if !((0 <= row as i32 + 3 * *dir_row)
                        && ((row as i32 + 3 * *dir_row ) < grid.len() as i32)
                        && (0 <= col as i32 + 3 * *dir_col)
                        && ((col as i32 + 3 * *dir_col) < grid[0].len() as i32))
                    {
                        continue;
                    }
                    if grid[(row as i32 + *dir_row) as usize][(col as i32 + *dir_col) as usize]
                        == 'M'
                        && grid[(row as i32 + 2 * *dir_row) as usize]
                            [(col as i32 + 2 * *dir_col) as usize]
                            == 'A'
                        && grid[(row as i32 + 3 * *dir_row) as usize]
                            [(col as i32 + 3 * *dir_col) as usize]
                            == 'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Count: {}", count);
}
