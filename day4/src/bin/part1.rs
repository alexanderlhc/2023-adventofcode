use std::{collections::HashSet, u128};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = read_file(args).expect("Could not read file");
    let points = total_points(file_content);
    println!("{}", points);
}

fn read_file(args: Vec<String>) -> Result<String, std::io::Error> {
    let filename = match args.get(1) {
        Some(filename) => filename,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No filename provided",
            ));
        }
    };

    let file_contents = std::fs::read_to_string(filename).expect("Could not read file");
    return Ok(file_contents);
}

#[test]
fn test_read_numbers() {
    assert_eq!(
        read_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()),
        (
            [41, 48, 83, 86, 17].iter().cloned().collect(),
            [83, 86, 6, 31, 17, 9, 48, 53].iter().cloned().collect(),
        )
    );

    assert_eq!(
        read_numbers("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string()),
        (
            [13, 32, 20, 16, 61].iter().cloned().collect(),
            [61, 30, 68, 82, 17, 32, 24, 19].iter().cloned().collect(),
        )
    );
    assert_eq!(
        read_numbers("Card 3: 1 21 53 59 44 | 69 82 63 72 16 21 14 1".to_string()),
        (
            [1, 21, 53, 59, 44].iter().cloned().collect(),
            [69, 82, 63, 72, 16, 21, 14, 1].iter().cloned().collect(),
        )
    );
    assert_eq!(
        read_numbers("Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83".to_string()),
        (
            [41, 92, 73, 84, 69].iter().cloned().collect(),
            [59, 84, 76, 51, 58, 5, 54, 83].iter().cloned().collect(),
        )
    );
    assert_eq!(
        read_numbers("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string()),
        (
            [87, 83, 26, 28, 32].iter().cloned().collect(),
            [88, 30, 70, 12, 93, 22, 82, 36].iter().cloned().collect(),
        )
    );
    assert_eq!(
        read_numbers("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string()),
        (
            [31, 18, 13, 56, 72].iter().cloned().collect(),
            [74, 77, 10, 23, 35, 67, 36, 11].iter().cloned().collect(),
        )
    );
}

fn read_numbers(line: String) -> (HashSet<u8>, HashSet<u8>) {
    let numbers = line.split(":").nth(1);
    let cards = numbers
        .unwrap_or("")
        .split("|")
        .map(String::from)
        .collect::<Vec<String>>();

    let lotto_card = to_map(cards.get(0).cloned().unwrap_or_else(String::new));
    let winnin_card = to_map(cards.get(1).cloned().unwrap_or_else(String::new));
    return (lotto_card, winnin_card);
}

fn to_map(line: String) -> HashSet<u8> {
    let mut set = HashSet::new();

    let numbers = line.split(" ");
    for number in numbers {
        if number.parse::<u8>().is_err() {
            continue;
        }
        set.insert(number.parse::<u8>().unwrap());
    }

    set
}

fn find_duplicates(lotto_card: HashSet<u8>, winnin_card: HashSet<u8>) -> HashSet<u8> {
    let mut duplicates = HashSet::new();
    for number in lotto_card.intersection(&winnin_card) {
        duplicates.insert(*number);
    }
    duplicates
}

fn count_points(duplicates: HashSet<u8>) -> u128 {
    if duplicates.len() == 0 {
        return 0;
    }
    2_usize.pow((duplicates.len() - 1).try_into().unwrap()) as u128
}

fn total_points(game: String) -> u128 {
    let mut total_points = 0;

    for line in game.lines() {
        let numbers = read_numbers(line.to_string());
        let numbers_with_win = find_duplicates(numbers.0, numbers.1);
        let points = count_points(numbers_with_win);
        println!("{} {} points", line, points);
        total_points += points;
    }

    total_points
}
