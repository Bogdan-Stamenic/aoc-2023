use core::panic;
use std::collections::{HashSet, VecDeque};
use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::all_consuming,
    bytes::complete::{tag, take_while1},
    multi::{separated_list0, separated_list1}, sequence::{separated_pair, preceded},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct ScratchCard {
    card_num: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<ScratchCard> {
    match all_consuming(separated_list0(tag("\n"), parse_scratch_card))(input) {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    }
}

fn parse_scratch_card(input: &str) -> IResult<&str, ScratchCard> {
    separated_pair(parse_card_num, tag(": "), parse_scratch_nums)
        .map(|(id, (winning_nums, my_nums))| {
            ScratchCard {card_num: id, winning_numbers: winning_nums, my_numbers: my_nums}
        })
    .parse(input)
}

fn parse_card_num(input: &str) -> IResult<&str, u32> {
    let (str,_) = tag("Card ")(input)?;
    alt((parse_num, preceded(take_while1(char::is_whitespace), parse_num)))(str)
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    let (out,num) = take_while1(char::is_numeric)(input)?;
    let num = num.parse::<u32>().unwrap();
    Ok((out,num))
}

fn parse_scratch_nums(input: &str) -> IResult<&str, (HashSet<u32>, HashSet<u32>)> {
    separated_pair(parse_nums_to_hast_set, tag(" | "), parse_nums_to_hast_set)
        .parse(input)
}

fn parse_nums_to_hast_set(input: &str) -> IResult<&str, HashSet<u32>> {
    separated_list1(
        take_while1(char::is_whitespace),
        alt((parse_num, preceded(take_while1(char::is_whitespace), parse_num)))
        )
        .map(|el| el.into_iter().collect::<HashSet<u32>>())
        .parse(input)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[ScratchCard]) -> u32 {
    input.iter()
        .map(|el| el.my_numbers.intersection(&el.winning_numbers).count())
        .map(|x| if x == 0 {0} else {1u32 << (x - 1)})
        .sum()
}

#[derive(Debug,Default)]
struct ScratchCardSpawnTracker {
    queue: VecDeque<u32>,
}

#[allow(dead_code)]
impl ScratchCardSpawnTracker {
   fn pop_front(&mut self) -> Option<u32> {
       self.queue.pop_front()
   } 

   fn push_back(&mut self, val: u32) {
       self.queue.push_back(val)
   }

   /* magic_fn(2,2) for self.queue = [1,1,1]
    * => self.queue = [3,3,1]
    *
    * magic_fn(4,1) for self.queue = []
    * => self.queue = [1,1,1,1]
    *
    * magic_fn(4,2) for self.queue = [5,2]
    * => self.queue = [7,4,2,2]
    * */
   fn magic_fn(&mut self, affected: usize, num_cards: u32) {
       for idx in 0..affected {
           if idx < self.queue.len() {
               self.queue[idx] += num_cards;
           } else {
               self.queue.push_back(num_cards);
           }
       }
   }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[ScratchCard]) -> u32 {
    let mut queue = ScratchCardSpawnTracker::default();
    input.iter()
        .map(|el| el.my_numbers.intersection(&el.winning_numbers).count())
        .map(|affected_cards| {
            let num_cards = match queue.pop_front() {
                Some(copies) => copies + 1,
                None => 1,//just the original
            };
            queue.magic_fn(affected_cards, num_cards);
            num_cards
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day4_parse_scratch_numbers() {
        const INPUT: &str = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (str, (win, my)) = parse_scratch_nums(INPUT).unwrap();
        assert_eq!(str, "");
        assert_eq!(win.contains(&86), true);
        assert_eq!(my.contains(&86), true);
    }

    #[test]
    fn day4_parse_scratch_card() {
        const INPUT: &str = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (str, card) = parse_scratch_card(INPUT).unwrap();
        assert_eq!(str, "");
        assert_eq!(card.card_num, 3);
        assert_eq!(card.winning_numbers.contains(&59), true);
        assert_eq!(card.my_numbers.contains(&82), true);
    }

    const TEST_INPUT: &str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    #[test]
    fn day4_parser() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.len(), 6)
    }

    #[test]
    fn day4_solve_p1_1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 13)
    }

    #[test]
    fn day4_solve_p1_2() {
        const INPUT: &str =
"Card   1: 13  5 40 15 21 61 74 55 32 56 | 21 57 74 56  7 84 37 47 75 66 68  8 55 22 53 61 40 13 15 41 32 46 95 65  5
Card   2: 92 97 39 23 25 40 33 70 55 77 | 25 70 23 91 45 60 34 56 82  6  9 62 24  3 67 99 18 58  1 26 50 37 32 14 85";
        let input = input_generator(INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 516);
    }

    #[test]
    fn day4_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part2(&input);
        assert_eq!(ans, 30)
    }
}

