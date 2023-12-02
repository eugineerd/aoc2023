fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day2.txt"));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// Common
#[derive(Debug, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input
        .lines()
        .filter_map(|l| Some(l.trim()).filter(|l| !l.is_empty()))
    {
        let (game_raw, rounds) = line.split_once(":").unwrap();
        let mut game = Game {
            id: game_raw.split_once(" ").unwrap().1.parse().unwrap(),
            ..Default::default()
        };
        for round_raw in rounds.split(";") {
            let mut round = Round::default();
            for cube in round_raw.split(",") {
                let (num_raw, color) = cube.trim().split_once(" ").unwrap();
                let num: u32 = num_raw.parse().unwrap();
                match color {
                    "red" => round.red += num,
                    "blue" => round.blue += num,
                    "green" => round.green += num,
                    _ => unreachable!(),
                }
            }
            game.rounds.push(round);
        }
        games.push(game);
    }
    games
}

// Part 1
fn part1(input: &str) -> String {
    let games = parse_input(input);
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let answer = games.iter().fold(0, |acc, game| {
        game.rounds
            .iter()
            .find(|round| round.red > max_red || round.green > max_green || round.blue > max_blue)
            .map(|_| acc)
            .unwrap_or(acc + game.id)
    });
    answer.to_string()
}

// Part 2
fn part2(input: &str) -> String {
    let games = parse_input(input);
    let answer = games.iter().fold(0, |acc, game| {
        let max_round = game
            .rounds
            .iter()
            .fold(Round::default(), |max_round, round| Round {
                red: max_round.red.max(round.red),
                blue: max_round.blue.max(round.blue),
                green: max_round.green.max(round.green),
            });
        acc + max_round.red * max_round.green * max_round.blue
    });
    answer.to_string()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(part1(input), "8");
    }

    #[test]
    fn part1_answer() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day2.txt"));
        assert_eq!(part1(input), "2541")
    }

    #[test]
    fn part2_example() {
        let input = r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;
        assert_eq!(part2(input), "2286");
    }

    #[test]
    fn part2_answer() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day2.txt"));
        assert_eq!(part2(input), "66016")
    }
}
