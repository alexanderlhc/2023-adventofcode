fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = read_file(args).expect("Could not read file");
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
