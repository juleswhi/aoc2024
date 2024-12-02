use std::fs;

fn get_asc(nums: &[i32]) -> bool {
    if nums
        .windows(2)
        .all(|window| window[0] < window[1] && window[1] - window[0] <= 3)
    {
        return true;
    }

    for i in 0..nums.len() {
        let mut modified_nums = nums.to_vec();
        modified_nums.remove(i);

        if modified_nums
            .windows(2)
            .all(|window| window[0] < window[1] && window[1] - window[0] <= 3)
        {
            return true;
        }
    }

    false
}

fn get_desc(nums: &[i32]) -> bool {
    if nums
        .windows(2)
        .all(|window| window[0] > window[1] && window[0] - window[1] <= 3)
    {
        return true;
    }

    // Try removing each number and check if the resulting sequence is valid
    for i in 0..nums.len() {
        let mut modified_nums = nums.to_vec();
        modified_nums.remove(i);

        if modified_nums
            .windows(2)
            .all(|window| window[0] > window[1] && window[0] - window[1] <= 3)
        {
            return true;
        }
    }

    false
}

pub fn run() {
    let file = fs::read_to_string("inputs/two.txt").unwrap();

    let mut safes: Vec<bool> = Vec::new();

    let lines = file.split('\n').collect::<Vec<&str>>();

    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            continue;
        }

        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let asc = get_asc(&nums);
        let desc = get_desc(&nums);

        if asc || desc {
            safes.push(true)
        }
    }

    let count = safes.iter().filter(|&&f| f).count();

    println!("{count}");
}
