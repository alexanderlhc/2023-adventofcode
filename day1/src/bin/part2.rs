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

const NUMBERS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

#[derive(Debug, PartialEq)]
struct FirstLast {
    first: u32,
    last: u32,
}

fn first_and_last_number(line: String) -> FirstLast {
    let first: Option<u32> = find_first(&line);
    let last: Option<u32> = find_last(&line);

    return FirstLast {
        first: first.unwrap_or(0),
        last: last.unwrap_or(0),
    };
}

#[test]
fn test_next_word_is_number() {
    assert_eq!(next_word_is_number(&"one".to_string()), Some(1));
    assert_eq!(next_word_is_number(&"two".to_string()), Some(2));
    assert_eq!(next_word_is_number(&"three".to_string()), Some(3));
    assert_eq!(next_word_is_number(&"four".to_string()), Some(4));
    assert_eq!(next_word_is_number(&"five".to_string()), Some(5));
    assert_eq!(next_word_is_number(&"six".to_string()), Some(6));
    assert_eq!(next_word_is_number(&"seven".to_string()), Some(7));
    assert_eq!(next_word_is_number(&"eight".to_string()), Some(8));
    assert_eq!(next_word_is_number(&"nine".to_string()), Some(9));
    assert_eq!(next_word_is_number(&"ten".to_string()), Some(10));
    assert_eq!(next_word_is_number(&"onetwo".to_string()), Some(1));
    assert_eq!(next_word_is_number(&"two2".to_string()), Some(2));
}

fn next_word_is_number(word: &String) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if word.starts_with(n) {
            return Some((i + 1) as u32);
        }
    }
    return None;
}

fn find_first(line: &String) -> Option<u32> {
    let first_char = line.chars().next();
    if first_char.is_none() {
        return None;
    }
    if first_char.unwrap().is_numeric() {
        return first_char.unwrap().to_digit(10);
    }
    if next_word_is_number(line).is_some() {
        return next_word_is_number(line);
    }
    return find_first(&line.chars().skip(1).collect::<String>());
}

fn ends_with_number(word: &String) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if word.ends_with(n) {
            return Some((i + 1) as u32);
        }
    }
    None
}

fn find_last(line: &String) -> Option<u32> {
    let last_char = line.chars().last();
    if last_char.is_none() {
        return None;
    }
    if last_char.unwrap().is_numeric() {
        return last_char.unwrap().to_digit(10);
    }
    if ends_with_number(line).is_some() {
        return ends_with_number(line);
    }
    return find_last(&line.chars().take(line.len() - 1).collect::<String>());
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
    assert_eq!(
        first_and_last_number("mtqxjrcn1two9fourncghmnbsseight".to_string()),
        FirstLast { first: 1, last: 8 }
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
