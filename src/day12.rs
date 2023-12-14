use std::collections::HashMap;
use itertools::Itertools;

use nom::{
    Parser,
    IResult,
    combinator::all_consuming,
    sequence::separated_pair,
    bytes::complete::{tag, take_while1, take_till},
    multi::separated_list1,
};

#[allow(dead_code)]
pub struct SpringConditionRecord {
    record: String,
    hints: Vec<u64>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<SpringConditionRecord> {
    match all_consuming(separated_list1(tag("\n"), parse_spring_line))
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_spring_line(input: &str) -> IResult<&str, SpringConditionRecord> {
    separated_pair(parse_spring_record, tag(" "), parse_spring_groups)
        .map(|(rec, grp)| SpringConditionRecord {record: rec, hints: grp})
        .parse(input)
}

fn parse_spring_record(input: &str) -> IResult<&str, String> {
    take_till(char::is_whitespace)
        .map(|str: &str| str.to_string())
        .parse(input)
}

fn parse_spring_groups(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), parse_num_to_u64)(input)
}

fn parse_num_to_u64(input: &str) -> IResult<&str, u64> {
    take_while1(char::is_numeric)
        .map(|num: &str| num.parse::<u64>().unwrap())
        .parse(input)
}

#[allow(unused_assignments)]
fn count_possibilities(hints: &[u64], record: &str, num_springs: u64) -> u64 {
    let mut out = 0;
    if hints.len() == 0 {
        if record.chars().any(|el| el == '#') {
            return 0;
        } else {//can only end in trailing SpringCondition::Damaged no matter what
            return 1;
        }
    } else {
        if record.len() == 0 {
            return 0; //sequence doesn't fulfill all hints
        }

        let mut rec_iter = record.chars();
        match rec_iter.next().unwrap() {
            '#'=> {
                if num_springs + 1 == *hints.first().unwrap() {
                    /* check if consecutive SpringCondition::Operational exceeds hint length */
                    if record.len() == 1 {//is last entry -> okay
                        out = count_possibilities(&hints[1..], &record[1..], 0);
                    } else if rec_iter.next().unwrap() == '#' {
                        out = 0;
                    } else {
                        out = count_possibilities(&hints[1..], &record[2..], 0);
                    }
                } else {
                    out = count_possibilities(&hints, &record[1..], num_springs + 1);
                }
            },
            '.' => {
                if num_springs == 0 {
                    out = count_possibilities(&hints, &record[1..], 0);
                } else {//failed to match a hint
                    out = 0;
                }
            },
            '?' => {
                /* Count if Operational */
                out = if num_springs + 1 == *hints.first().unwrap() {
                    /* check if consecutive SpringCondition::Operational exceeds hint length */
                    if record.len() == 1 {
                        count_possibilities(&hints[1..], &record[1..], 0)
                    } else if rec_iter.next().unwrap() == '#' {
                        0
                    } else {
                        count_possibilities(&hints[1..], &record[2..], 0)
                    }
                } else {
                    count_possibilities(&hints, &record[1..], num_springs + 1)
                };
                /* Count if Damaged */
                out += if num_springs == 0 {
                    count_possibilities(&hints, &record[1..], 0)
                } else {
                    0
                };
            },
            _ => unreachable!("Expected one of : \'#\', \'.\', \'?\'"),
        }
    }
    out
}

#[allow(unused_assignments)]
fn count_possibilities_memoized<'a>(hints: &'a [u64], record: &'a str, num_springs: u64, memo: &mut HashMap<(&'a [u64], &'a str,u64), u64>) -> u64 {
    let mut out = 0;
    if memo.contains_key(&(hints, record, num_springs)) {
        out = *memo.get(&(hints, record, num_springs)).expect("Memo should've had the value");
    } else if hints.len() == 0 {
        if record.chars().any(|el| el == '#') {
            return 0;
        } else {//can only end in trailing SpringCondition::Damaged no matter what
            return 1;
        }
    } else {
        if record.len() == 0 {
            return 0; //sequence doesn't fulfill all hints
        }

        let mut rec_iter = record.chars();
        match rec_iter.next().unwrap() {
            '#'=> {
                if num_springs + 1 == *hints.first().unwrap() {
                    /* check if consecutive SpringCondition::Operational exceeds hint length */
                    if record.len() == 1 {//is last entry -> okay
                        out = count_possibilities_memoized(&hints[1..], &record[1..], 0, memo);
                    } else if rec_iter.next().unwrap() == '#' {
                        out = 0;
                    } else {
                        out = count_possibilities_memoized(&hints[1..], &record[2..], 0, memo);
                    }
                } else {
                    out = count_possibilities_memoized(&hints, &record[1..], num_springs + 1, memo);
                }
            },
            '.' => {
                if num_springs == 0 {
                    out = count_possibilities_memoized(&hints, &record[1..], 0, memo);
                } else {//failed to match a hint
                    out = 0;
                }
            },
            '?' => {
                /* Count if Operational */
                out = if num_springs + 1 == *hints.first().unwrap() {
                    /* check if consecutive SpringCondition::Operational exceeds hint length */
                    if record.len() == 1 {
                        count_possibilities_memoized(&hints[1..], &record[1..], 0, memo)
                    } else if rec_iter.next().unwrap() == '#' {
                        0
                    } else {
                        count_possibilities_memoized(&hints[1..], &record[2..], 0, memo)
                    }
                } else {
                    count_possibilities_memoized(&hints, &record[1..], num_springs + 1, memo)
                };
                /* Count if Damaged */
                out += if num_springs == 0 {
                    count_possibilities_memoized(&hints, &record[1..], 0, memo)
                } else {
                    0
                };
            },
            _ => unreachable!("Expected one of : \'#\', \'.\', \'?\'"),
        }
    }
    memo.insert((hints, record,num_springs), out);
    out
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[SpringConditionRecord]) -> u64 {
    input.iter()
        .map(|cond_rec| {
            count_possibilities(&cond_rec.hints, &cond_rec.record, 0)
        })
    .sum()
}

#[allow(dead_code)]
fn solve_part1_memoized(input: &[SpringConditionRecord]) -> u64 {
    let mut memo = HashMap::<(&[u64],&str,u64),u64>::new();
    input.iter()
        .map(|cond_rec| {
            count_possibilities_memoized(&cond_rec.hints, &cond_rec.record, 0, &mut memo)
        })
    .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[SpringConditionRecord]) -> u64 {
    let unfolded_input = input.iter()
        .map(|cond_rec| SpringConditionRecord{
            record: (0..5).into_iter().map(|_| cond_rec.record.clone()).join("?"),
            hints: (0..5).flat_map(|_|  cond_rec.hints.clone()).collect::<Vec<_>>(),
        })
    .collect::<Vec<SpringConditionRecord>>();
    /* Solve problem */
    let mut memo = HashMap::<(&[u64],&str,u64),u64>::new();
    unfolded_input.iter()
        .map(|cond_rec| {
            count_possibilities_memoized(&cond_rec.hints, &cond_rec.record, 0, &mut memo)
        })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    #[test]
    fn test_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 6);
    }

    #[test]
    fn test_count_possibilities_1() {
        let record = "???.###";
        let hints = vec![1, 1, 3];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_count_possibilities_2() {
        let record = ".??..??...?##.";
        let hints = vec![1, 1, 3];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 4);
    }
    
    #[test]
    fn test_count_possibilities_3() {
        let record = "?###????????";
        let hints = vec![3, 2, 1];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 10);
    }

    #[test]
    #[ignore]
    fn test_count_possibilities_4() {
        let record = "#.#";
        let hints = vec![2];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 0);
    }

    #[test]
    #[ignore]
    fn test_count_possibilities_5() {
        let record = "##??";
        let hints = vec![2, 1];
        let ans = count_possibilities(&hints, &record, 1);
        assert_eq!(ans, 0);
    }

    #[test]
    #[ignore]
    fn test_count_possibilities_6() {
        let record = "#.###";
        let hints = vec![3];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 0);
    }

    #[test]
    #[ignore]
    fn test_count_possibilities_7() {
        let record = "??###";
        let hints = vec![3];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 1);
    }

    #[test]
    #[ignore]
    fn test_count_possibilities_8() {
        let record = ".?###";
        let hints = vec![3];
        let ans = count_possibilities(&hints, &record, 0);
        assert_eq!(ans, 1);
    }

    #[test]
    fn test_solve_day12_p1() {
        let input1 = input_generator(TEST_INPUT);
        let ans = solve_part1(&input1);
        assert_eq!(ans, 21);
    }

    #[test]
    #[ignore]
    fn test_solve_day12_p1_memoized() {
        let input1 = input_generator(TEST_INPUT);
        let ans = solve_part1(&input1);
        assert_eq!(ans, 21);
    }

    #[test]
    fn test_solve_day12_p2() {
        let input2 = input_generator(TEST_INPUT);
        let ans = solve_part2(&input2);
        assert_eq!(ans, 525152);
    }
}
