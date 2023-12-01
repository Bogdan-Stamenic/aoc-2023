fn get_first_and_last_digit(line: &str) -> (u32, u32) {
    let first = line
        .bytes()
        .skip_while(|c| (*c < b'0') || (*c > b'9'))
        .take(1)
        .map(|d: u8| d - b'0')
        .map(|d: u8| u32::from(d))
        .next().unwrap();
    let last = line
        .bytes()
        .rev()
        .skip_while(|c| (*c < b'0') || (*c > b'9'))
        .take(1)
        .map(|d: u8| d - b'0')
        .map(|d: u8| u32::from(d))
        .next().unwrap();
    (first, last)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(u32,u32)> {
    input.lines()
        .map(|line| get_first_and_last_digit(line))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[(u32,u32)]) -> u32 {
    input.iter()
        .map(|(fst,sec)| fst*10 + sec)
        .sum()
}

//#[aoc(day1, part2)]
//pub fn solve_part2(input: &[u32]) -> u32 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_day1_generator() {
        let input = input_generator(TEST_INPUT1);
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

