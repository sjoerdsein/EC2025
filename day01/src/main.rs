fn parse_notes(filepath: &str) -> (Vec<String>, Vec<i32>) {
    let notes = std::fs::read_to_string(filepath).expect("Failed to read file");
    assert!(notes.is_ascii(), "Non-ASCII characters are not supported");

    let mut notes = notes.lines().filter(|line| !line.is_empty());

    let names: Vec<String> = notes
        .next()
        .expect("No names found")
        .split(',')
        .map(|name| name.to_string())
        .collect();

    let instructions: Vec<i32> = notes
        .next()
        .expect("No instructions found")
        .split(',')
        .map(|instr| {
            let direction = &instr[..1];
            let amount: i32 = instr[1..].parse().expect("Not an integer");
            match direction {
                "L" => -amount,
                "R" => amount,
                _ => panic!("Invalid direction"),
            }
        })
        .collect();

    (names, instructions)
}

fn part1(filepath: &str) {
    let (names, instructions) = parse_notes(filepath);
    let len_names = names.len() as i32;

    let name_idx = instructions
        .iter()
        .fold(0, |acc, instr| (acc + instr).clamp(0, len_names - 1));

    println!("{}", names[name_idx as usize])
}

fn part2(filepath: &str) {
    let (names, instructions) = parse_notes(filepath);
    let len_names = names.len() as i32;

    let name_idx: i32 = instructions.iter().sum();
    let name_idx = name_idx.rem_euclid(len_names);

    println!("{}", names[name_idx as usize])
}

fn part3(filepath: &str) {
    let (mut names, instructions) = parse_notes(filepath);
    let len_names = names.len() as i32;

    for instr in &instructions {
        names.swap(0, instr.rem_euclid(len_names) as usize);
    }

    println!("{}", names[0])
}

fn main() {
    part1("day01/part1.txt");
    part2("day01/part2.txt");
    part3("day01/part3.txt");
}
