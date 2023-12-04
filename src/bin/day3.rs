fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day3.txt"));
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}

const DETAIL_FLAG: u8 = 1 << 0;
const NUMBER_FLAG: u8 = 1 << 1;
const NUMBER_START_FLAG: u8 = 1 << 2;
const NUMBER_OFFSET: u8 = 3;

fn part1(input: &str) -> String {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();
    let line_length = lines[0].len();
    let mut map = vec![0u8; lines.len() * line_length];
    // Fill map
    for (i, line) in lines.iter().enumerate() {
        let mut is_number = false;
        for (j, ch) in line.iter().enumerate() {
            match ch {
                b'0'..=b'9' => {
                    map[j + i * line_length] |= NUMBER_FLAG | (ch - b'0') << NUMBER_OFFSET;
                    if !is_number {
                        map[j + i * line_length] |= NUMBER_START_FLAG;
                        is_number = true;
                    }
                }
                b'.' => {
                    is_number = false;
                }
                _ => {
                    is_number = false;
                    for (map_i, map_j) in [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ] {
                        if let Some(v) = map.get_mut(
                            ((j as isize + map_j) + (i as isize + map_i) * line_length as isize)
                                as usize,
                        ) {
                            *v |= DETAIL_FLAG;
                        }
                    }
                }
            }
        }
    }
    // Sum all numbers
    let mut sum = 0;
    for chunk in map.chunks_exact(line_length) {
        let mut j = 0;
        while j < chunk.len() {
            if (chunk[j] & DETAIL_FLAG) != 0 {
                let mut number = 0;
                while j > 0 && (chunk[j] & NUMBER_FLAG) != 0 && (chunk[j] & NUMBER_START_FLAG) == 0
                {
                    j -= 1
                }
                while j < chunk.len() && (chunk[j] & NUMBER_FLAG) != 0 {
                    number = number * 10 + (chunk[j] >> NUMBER_OFFSET) as usize;
                    j += 1;
                }
                sum += number;
            }
            j += 1;
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = concat!(
            "467..114..\n",
            "...*......\n",
            "..35..633.\n",
            "......#...\n",
            "617*......\n",
            ".....+.58.\n",
            "..592.....\n",
            "......755.\n",
            "...$.*....\n",
            ".664.598.."
        );
        assert_eq!(part1(input), "4361");
    }

    #[test]
    fn part1_answer() {
        let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day3.txt"));
        assert_eq!(part1(input), "521515");
    }
}
