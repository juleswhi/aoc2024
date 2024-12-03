#![allow(unused_assignments)]
use std::fs;

fn parse_mul(arr: &[u8], idx: usize) -> Option<()> {
    if arr[idx] != b'm' {
        return None;
    }

    if arr[idx + 1] != b'u' {
        return None;
    }

    if arr[idx + 2] != b'l' {
        return None;
    }

    if arr[idx + 3] != b'(' {
        return None;
    }

    Some(())
}

fn parse_do(arr: &[u8], idx: usize) -> Option<usize> {
    if arr[idx] != b'd' {
        return None;
    }

    if arr[idx + 1] != b'o' {
        return None;
    }

    if arr[idx + 2] != b'(' {
        return None;
    }

    if arr[idx + 3] != b')' {
        return None;
    }

    Some(idx + 4)
}

fn parse_dont(arr: &[u8], idx: usize) -> Option<usize> {
    if arr[idx] != b'd' {
        return None;
    }

    if arr[idx + 1] != b'o' {
        return None;
    }

    if arr[idx + 2] != b'n' {
        return None;
    }

    if arr[idx + 3] != b'\'' {
        return None;
    }

    if arr[idx + 4] != b't' {
        return None;
    }

    if arr[idx + 5] != b'(' {
        return None;
    }

    if arr[idx + 6] != b')' {
        return None;
    }

    Some(idx + 7)
}

fn parse_bracket(arr: &[u8], mut idx: usize) -> Option<(usize, i32, i32)> {
    let mut half: bool = false;

    let mut x: String = String::new();
    let mut y: String = String::new();

    loop {
        if arr[idx] as char == ',' {
            half = true;
            idx += 1;
            continue;
        }

        if (arr[idx] as char) == ')' {
            println!("Parsed closing bracket");
            if x == "" || y == "" {
                return None;
            }
            return Some((idx, x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
        }

        if !(arr[idx] as char).is_numeric() {
            return None;
        }

        let c = arr[idx].clone();
        if half {
            y.push(c as char);
        } else {
            x.push(c as char);
        }

        idx += 1;
    }
}

pub fn run() {
    let read = fs::read_to_string("inputs/three.txt").unwrap().to_string();
    let file = read.as_bytes();

    let mut pairs: Vec<(i32, i32)> = Vec::new();

    // do() or don't()
    let mut state: bool = true;

    // take 5 to allow for idx problems when parsing mul()
    for mut i in 0..file.len() - 5 {
        if let Some(v) = parse_do(file, i) {
            i = v - 1;
            state = true;
        }

        if let Some(v) = parse_dont(file, i) {
            i = v - 1;
            state = false;
        }

        if !state {
            continue;
        }

        if let None = parse_mul(file, i) {
            continue;
        }

        if let Some(v) = parse_bracket(file, i + 4) {
            i = v.0;
            pairs.push((v.1, v.2));
        }
    }

    println!("{}", pairs.iter().map(|f| f.0 * f.1).sum::<i32>())
}
