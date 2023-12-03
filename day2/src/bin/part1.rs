fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_content = read_file(args).expect("Could not read file");

    const MAX_GEMS: GemCount = GemCount {
        green: 13,
        red: 12,
        blue: 14,
    };

    let games = file_content
        .lines()
        .map(|line| read_game(line))
        .collect::<Vec<Game>>();

    part_one(&games);
    part_two(&games);
}

fn part_one(games: &Vec<Game>) {
    const MAX_GEMS: GemCount = GemCount {
        green: 13,
        red: 12,
        blue: 14,
    };
    let legal_games = games
        .iter()
        .filter(|game| is_game_legal(game, &MAX_GEMS))
        .collect::<Vec<&Game>>();
    let mut legal_games_id_acum = 0;
    legal_games.clone().into_iter().for_each(|game| {
        legal_games_id_acum += game.game_id;
    });
    println!(
        "Part 1: Total games: {}, Legal games: {}. Sum of their IDs {}",
        games.len(),
        legal_games.len(),
        legal_games_id_acum
    );
}

fn part_two(games: &Vec<Game>) {
    let fewest_gems_per_game = games
        .iter()
        .map(|game| fewest_gems_per_game(game))
        .collect::<Vec<GemCount>>();

    let power_for_each_game = fewest_gems_per_game
        .iter()
        .map(|gem_count| gem_count.green * gem_count.red * gem_count.blue)
        .collect::<Vec<u32>>();

    let sum_of_powers = power_for_each_game.iter().sum::<u32>();

    println!("Part 2: Sum of powers of fewest games: {}", sum_of_powers);
}

#[derive(Debug, PartialEq)]
struct Game {
    game_id: u32,
    rounds: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
struct Draw {
    green: u32,
    red: u32,
    blue: u32,
}

struct GemCount {
    green: u32,
    red: u32,
    blue: u32,
}

fn fewest_gems_per_game(game: &Game) -> GemCount {
    let mut fewest_gems = GemCount {
        green: 0,
        red: 0,
        blue: 0,
    };

    for round in &game.rounds {
        fewest_gems.red = fewest_gems.red.max(round.red);
        fewest_gems.green = fewest_gems.green.max(round.green);
        fewest_gems.blue = fewest_gems.blue.max(round.blue);
    }
    return fewest_gems;
}

#[test]
fn test_is_draw_legal() {
    const MAX_GEMS: GemCount = GemCount {
        green: 13,
        red: 12,
        blue: 14,
    };
    assert_eq!(
        is_draw_legal(
            &Draw {
                green: 8,
                red: 20,
                blue: 6,
            },
            &MAX_GEMS
        ),
        false
    );
    assert_eq!(
        is_draw_legal(
            &Draw {
                red: 1,
                blue: 14,
                green: 1
            },
            &MAX_GEMS
        ),
        true
    )
}

fn is_draw_legal(draw: &Draw, gem_count: &GemCount) -> bool {
    if draw.green > gem_count.green || draw.red > gem_count.red || draw.blue > gem_count.blue {
        return false;
    }
    return true;
}

fn is_game_legal(game: &Game, gem_count: &GemCount) -> bool {
    for round in &game.rounds {
        let is_legal = is_draw_legal(round, gem_count);
        if !is_legal {
            return false;
        }
    }
    return true;
}

#[test]
fn test_read_game() {
    assert_eq!(
        read_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
        Game {
            game_id: 3,
            rounds: vec![
                Draw {
                    green: 8,
                    red: 20,
                    blue: 6,
                },
                Draw {
                    green: 13,
                    red: 4,
                    blue: 5,
                },
                Draw {
                    green: 5,
                    red: 1,
                    blue: 0,
                },
            ],
        }
    );
    assert_eq!(
        read_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        Game {
            game_id: 5,
            rounds: vec![
                Draw {
                    red: 6,
                    blue: 1,
                    green: 3
                },
                Draw {
                    red: 1,
                    blue: 2,
                    green: 2
                },
            ]
        }
    );
    assert_eq!(
        read_game("Game 11: 1 red"),
        Game {
            game_id: 11,
            rounds: vec![Draw {
                red: 1,
                green: 0,
                blue: 0
            }]
        }
    );
    assert_eq!(
        read_game("Game 95: 1 red, 7 blue, 2 green; 3 red, 14 blue, 2 green; 1 red; 1 red, 14 blue, 1 green; 4 blue, 10 red, 2 green; 9 blue, 7 red"),
        Game {
            game_id: 95,
            rounds: vec![
                Draw {
                    red: 1,
                    blue: 7,
                    green: 2
                },
                Draw {
                    red: 3,
                    blue: 14,
                    green: 2
                },
                Draw {
                    red: 1,
                    blue: 0,
                    green: 0
                },
                Draw {
                    red: 1,
                    blue: 14,
                    green: 1
                },
                Draw {
                    red: 10,
                    blue: 4,
                    green: 2
                },
                Draw {
                    red: 7,
                    blue: 9,
                    green: 0
                },
            ]
        }
    );
}

fn read_game(line: &str) -> Game {
    let game_and_draw = line.split(": ").collect::<Vec<&str>>();
    let game_id = game_and_draw[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();

    let round_draws = game_and_draw[1].split("; ").collect::<Vec<&str>>();
    let mut rounds: Vec<Draw> = vec![];

    if round_draws.len() == 0 {
        let draw = round_draw(game_and_draw[1]);
        rounds.push(draw);
    }

    for i in 0..round_draws.len() {
        let round = round_draws[i as usize];
        let draw = round_draw(round);
        rounds.push(draw);
    }

    return Game { game_id, rounds };
}

fn round_draw(round: &str) -> Draw {
    let colors = round.split(", ").collect::<Vec<&str>>();
    let mut draw = Draw {
        green: 0,
        red: 0,
        blue: 0,
    };

    for color in colors {
        let color_and_count = color.split(" ").collect::<Vec<&str>>();
        let color_name = color_and_count[1];
        let count = color_and_count[0].parse::<u32>().unwrap_or(0);

        match color_name {
            "green" => draw.green += count,
            "red" => draw.red += count,
            "blue" => draw.blue += count,
            _ => (),
        }
    }

    draw
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
