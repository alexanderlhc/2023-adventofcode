fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = read_file(args).expect("Could not read file");

    let mut total_sum: u32 = 0;

    for line in file_content.lines() {
        let first_last = first_and_last_number(line.to_string());
        let line_sum = first_last_to_num(first_last);
        total_sum += line_sum;
    }

    println!("Total sum: {}", total_sum);
}

#[derive(Debug, PartialEq)]
struct FirstLast {
    first: u32,
    last: u32,
}

#[derive(Debug, PartialEq)]
enum WordNumberStatus {
    NotNumber,
    PartialNumber,
    Number(u32),
}

fn word_is_number_or_part_of(word: &str) -> WordNumberStatus {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    for (i, number) in numbers.iter().enumerate() {
        if word == *number {
            return WordNumberStatus::Number((i + 1) as u32);
        }
        if number.starts_with(word) {
            return WordNumberStatus::PartialNumber;
        }
    }

    return WordNumberStatus::NotNumber;
}

#[test]
fn test_word_is_number_or_part_of() {
    assert_eq!(
        word_is_number_or_part_of("one"),
        WordNumberStatus::Number(1)
    );
    assert_eq!(
        word_is_number_or_part_of("onehundred"),
        WordNumberStatus::NotNumber
    );
    assert_eq!(
        word_is_number_or_part_of("onehundredone"),
        WordNumberStatus::NotNumber
    );
    assert_eq!(
        word_is_number_or_part_of("two"),
        WordNumberStatus::Number(2)
    );
    assert_eq!(
        word_is_number_or_part_of("three"),
        WordNumberStatus::Number(3)
    );
    assert_eq!(
        word_is_number_or_part_of("four"),
        WordNumberStatus::Number(4)
    );
    assert_eq!(
        word_is_number_or_part_of("five"),
        WordNumberStatus::Number(5)
    );
    assert_eq!(
        word_is_number_or_part_of("six"),
        WordNumberStatus::Number(6)
    );
    assert_eq!(
        word_is_number_or_part_of("seven"),
        WordNumberStatus::Number(7)
    );
    assert_eq!(
        word_is_number_or_part_of("eight"),
        WordNumberStatus::Number(8)
    );
    assert_eq!(
        word_is_number_or_part_of("nine"),
        WordNumberStatus::Number(9)
    );
    assert_eq!(
        word_is_number_or_part_of("ten"),
        WordNumberStatus::Number(10)
    );
}

fn first_and_last_number(line: String) -> FirstLast {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    let mut word = String::new();

    for (_i, c) in line.chars().enumerate() {
        word.push(c);
        println!("{} {} {}", line, c, word);
        let is_number_or_part_of = word_is_number_or_part_of(word.as_str());

        match is_number_or_part_of {
            WordNumberStatus::NotNumber => {
                word.clear();
                word.push(c);
                if c.is_digit(10) {
                    let digit = c.to_digit(10).unwrap();
                    if first == 0 {
                        first = digit
                    }
                    last = digit;
                }
            }
            WordNumberStatus::PartialNumber => continue,
            WordNumberStatus::Number(number) => {
                if first == 0 {
                    first = number
                }
                last = number;
                word.clear();
                word.push(c);
            }
        }
    }

    return FirstLast { first, last };
}

#[test]
fn test_first_and_last_number() {
    assert_eq!(
        first_and_last_number("two1nine".to_string()),
        FirstLast { first: 2, last: 9 }
    );
    assert_eq!(
        first_and_last_number("eightwothree".to_string()),
        FirstLast { first: 8, last: 3 }
    );
    assert_eq!(
        first_and_last_number("abcone2threexyz".to_string()),
        FirstLast { first: 1, last: 3 }
    );
    assert_eq!(
        first_and_last_number("xtwone3four".to_string()),
        FirstLast { first: 2, last: 4 }
    );
    assert_eq!(
        first_and_last_number("4nineeightseven2".to_string()),
        FirstLast { first: 4, last: 2 }
    );
    assert_eq!(
        first_and_last_number("zoneight234".to_string()),
        FirstLast { first: 1, last: 4 }
    );
    assert_eq!(
        first_and_last_number("7pqrstsixteen".to_string()),
        FirstLast { first: 7, last: 6 }
    );
}

fn first_last_to_num(first_last: FirstLast) -> u32 {
    return first_last.first * 10 + first_last.last;
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
