use std::fs;

fn parse_line(line: String) -> Option<(i32, i32)> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 2 {
        return None;
    }

    match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        (Ok(num1), Ok(num2)) => Some((num1, num2)),
        _ => None,
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("inputs/one.txt")?;

    let mut lefts: Vec<i32> = Vec::new();
    let mut rights: Vec<i32> = Vec::new();

    for line in file.split('\n').collect::<Vec<&str>>() {
        if let Some(pair) = parse_line(line.into()) {
            lefts.push(pair.0);
            rights.push(pair.1);
        }
    }

    let mut products: Vec<i32> = Vec::new();

    for i in 0..lefts.len() {
        products.push(lefts[i] * rights.iter().filter(|&&x| x == lefts[i]).cloned().collect::<Vec<i32>>().len() as i32);
    }

    let sum: i32 = products.iter().sum();

    println!("sum: {}", sum);

    Ok(())
}
