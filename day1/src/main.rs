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

struct FirstLast {
    first: u32,
    last: u32,
}

fn first_and_last_number(line: String) -> FirstLast {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    for (_i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first == 0 {
                first = digit
            }
            last = digit;
        }
    }

    return FirstLast { first, last };
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
