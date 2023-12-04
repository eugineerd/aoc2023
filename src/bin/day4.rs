use std::collections::HashSet;

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day4.txt"));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(l, r)| (l.split_once(":").unwrap().1, r))
                .unwrap()
        })
        .fold(0, |acc, (l, r)| {
            acc + l
                .split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(&r.split_whitespace().collect())
                .collect::<Vec<_>>()
                .len()
                .checked_sub(1)
                .map(|p| 2u32.pow(p as u32))
                .unwrap_or(0)
        })
        .to_string()
}

fn part2(input: &str) -> String {
    let mut cards = Vec::new();
    input
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(l, r)| (l.split_once(":").unwrap().1, r))
                .unwrap()
        })
        .enumerate()
        .fold(0u32, |acc, (i, (l, r))| {
            if i >= cards.len() {
                cards.push(1)
            }
            let matches = l
                .split_whitespace()
                .collect::<HashSet<_>>()
                .intersection(&r.split_whitespace().collect())
                .collect::<Vec<_>>()
                .len();
            let curr_card_num = cards[i];
            for j in 1..=matches {
                if let Some(x) = cards.get_mut(i + j) {
                    *x += curr_card_num
                } else {
                    cards.push(curr_card_num + 1)
                }
            }
            acc + curr_card_num
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn part1_answer() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day4.txt"));
        assert_eq!(part1(input), "24542");
    }

    #[test]
    fn part2_example() {
        let input = concat!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(part2(input), "30");
    }

    #[test]
    fn part2_answer() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day4.txt"));
        assert_eq!(part2(input), "8736438");
    }
}
