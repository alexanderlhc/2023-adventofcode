fn main() {
    let file_content = read_file(std::env::args().collect()).expect("Could not read file");
    let board = read_board(&file_content);
    let numbers = find_numbers(&board);
    println!("Numbers: {:?}", numbers);
    let sum = numbers.iter().sum::<u128>();
    println!("Sum: {}", sum);
    // let board = read_board(BOARD);
    // let board = read_board(BOARD);
}

type Board = Vec<Vec<char>>;

#[test]
fn test_find_numbers() {
    const BOARD: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    assert_eq!(
        find_numbers(&read_board(
            r#"467..114..
...*......"#
        )),
        vec![467]
    );
    assert_eq!(
        find_numbers(&read_board(&BOARD)),
        vec![467, 35, 633, 617, 592, 755, 664, 598]
    );
    assert_eq!(find_numbers(&read_board("467..114..")), vec![]);
    assert_eq!(
        find_numbers(&read_board(
            r#"........
.24..4..
......*."#
        )),
        vec![4]
    );

    assert_eq!(
        find_numbers(&read_board(
            r#"........
.24$-4..
......*."#
        )),
        vec![24, 4]
    );

    assert_eq!(
        find_numbers(&read_board(
            r#"11....11
..$..$..
11....11"#
        )),
        vec![11, 11, 11, 11]
    );

    assert_eq!(find_numbers(&read_board(r#"11.$."#)), vec![]);
}

fn find_numbers(board: &Board) -> Vec<u128> {
    let mut numbers: Vec<u128> = vec![];
    for (_x, row) in board.iter().enumerate() {
        let mut number: u128 = 0;
        let mut number_started = false;
        let mut number_has_adjacent_symbol = false;
        for (_y, c) in row.iter().enumerate() {
            if c.is_numeric() {
                number = number * 10 + c.to_digit(10).unwrap() as u128;
                number_started = true;
                if has_adjacent_symbol(board, (_x as u128, _y as u128)) {
                    number_has_adjacent_symbol = true;
                }
            } else {
                if number_started && number_has_adjacent_symbol {
                    numbers.push(number.into());
                }
                number_started = false;
                number = 0;
                number_has_adjacent_symbol = false;
            }
        }
        if number_started && number_has_adjacent_symbol {
            numbers.push(number.into());
        }
    }
    return numbers;
}

#[test]
fn test_has_adjacent_symbol() {
    assert_eq!(
        has_adjacent_symbol(
            &read_board(
                r#"467..114..
...*......"#
            ),
            (0, 5)
        ),
        false
    );

    assert_eq!(
        has_adjacent_symbol(
            &read_board(
                r#"467..114..
...*......"#
            ),
            (0, 2)
        ),
        true
    );

    assert_eq!(
        has_adjacent_symbol(
            &read_board(
                r#"467..114..
...*......"#
            ),
            (0, 1)
        ),
        false
    )
}

fn has_adjacent_symbol(board: &Board, coordinate: (u128, u128)) -> bool {
    let walk_range = [-1, 0, 1];
    let (x, y) = coordinate;
    for &dx in &walk_range {
        for &dy in &walk_range {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some(row) = board.get((x as i32 + dx) as usize) {
                if let Some(&candidate) = row.get((y as i32 + dy) as usize) {
                    if is_symbol(candidate) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn is_symbol(c: char) -> bool {
    return !c.is_numeric() && c != '.';
}

fn read_board(text: &str) -> Board {
    let mut board: Board = vec![];
    for line in text.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        board.push(row);
    }
    return board;
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
