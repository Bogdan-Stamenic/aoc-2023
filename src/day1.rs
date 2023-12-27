#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> String {
    input.to_string()
}

#[inline]
fn get_first_and_last_digit(line: &str) -> (u32, u32) {
    let first = line
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .take(1)
        .map(|d: u8| d - b'0')
        .map(|d: u8| u32::from(d))
        .next().unwrap();
    let last = line
        .bytes()
        .rev()
        .filter(|c| c.is_ascii_digit())
        .take(1)
        .map(|d: u8| d - b'0')
        .map(|d: u8| u32::from(d))
        .next().unwrap();
    (first, last)
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let parsed = input.lines()
        .map(|line| get_first_and_last_digit(line))
        .collect::<Vec<(u32,u32)>>();
    parsed.iter()
        .map(|(fst,sec)| fst*10 + sec)
        .sum()
}

const VALID_MATCHES: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn word_to_digit(word: &str) -> u8 {
    match word {
        "0" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid word: {}", word),
    }
}

fn first_match<'a>(haystack: &str, words: &[&'a str]) -> &'a str {
    for idx in 0..haystack.bytes().len() {
        for &word in words {
            if haystack.as_bytes()[idx..].starts_with(word.as_bytes()) {
                return word;
            }
        }
    }
    panic!("No word found in: {}", haystack);
}

fn last_match<'a>(haystack: &str, words: &[&'a str]) -> &'a str {
    for idx in (0..haystack.bytes().len()).rev() {
        for &word in words {
            if haystack.as_bytes()[idx..].starts_with(word.as_bytes()) {
                return word;
            }
        }
    }
    panic!("No word found in: {}", haystack);
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = word_to_digit(first_match(line, &VALID_MATCHES));
            let last_digit = word_to_digit(last_match(line, &VALID_MATCHES));
            (first_digit * 10 + last_digit) as u32
        })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_day1p1_parser() {
        let input = TEST_INPUT1
            .lines()
            .map(|line| get_first_and_last_digit(line))
            .collect::<Vec<(u32,u32)>>();
        assert_eq!(input[0], (1,2));
        assert_eq!(input[1], (3,8));
        assert_eq!(input[2], (1,5));
        assert_eq!(input[3], (7,7));
    }

    #[test]
    fn test_solve_day1p1() {
        let input = input_generator(TEST_INPUT1);
        let ans = solve_part1(&input);
        assert_eq!(ans, 142);
    }
}

