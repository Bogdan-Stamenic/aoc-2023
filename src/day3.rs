use ndarray::*;
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{tag,take},
    character::complete::one_of,
    combinator::{all_consuming, value, not},
    multi::many1, sequence::{terminated, tuple},
};

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum SchematicEntry {
    Number(char),
    Symbol(char),
    Dot,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Array2<SchematicEntry> {
    let parse_vec = match all_consuming(many1(parse_schematic_line))
        .parse(input)
    {
        Ok((_, val)) => val,
        Err(e) => panic!("{}", e),
    };
    let shape = (parse_vec.len(), parse_vec[0].dim());
    let flat = parse_vec.iter().flatten().cloned().collect();
    let input = Array2::from_shape_vec(shape, flat).expect("Dimensions didn't line up");
    input
}

fn parse_schematic_line(input: &str) -> IResult<&str, Array1<SchematicEntry>> {
    alt((
            terminated(many1(parse_schematic_entry), tag("\n")),
            many1(parse_schematic_entry)
            ))
        .map(|vec| Array1::from_shape_vec(vec.len(), vec).unwrap())
        .parse(input)
}

fn parse_schematic_entry(input: &str) -> IResult<&str, SchematicEntry> {
    alt((
            parse_schematic_num,
            value(SchematicEntry::Dot, tag(".")),
            parse_schematic_symbol,
    ))
        .parse(input)
}

fn parse_schematic_num(input: &str) -> IResult<&str, SchematicEntry> {
    one_of("0123456789")
        .map(|c| SchematicEntry::Number(c))
        .parse(input)
}

fn parse_schematic_symbol(input: &str) -> IResult<&str, SchematicEntry> {
    tuple((not(tag("\n")), take(1usize)))
        .map(|(_,c): (_,&str)| SchematicEntry::Symbol(c.chars().next().unwrap()))
        .parse(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Array2<SchematicEntry>) -> u64 {
    let mut ans = 0;
    let mut fsaw_symbol = false;
    let mut char_stack = Vec::<char>::new();
    let mut char_stack_row_num = 0;//track where we're collecting num chars
    let (outer_max, inner_max) = input.dim();
    for j in 0..outer_max {
        for i in 0..inner_max {
            match input[[j,i]] {
                SchematicEntry::Number(num_char) => {
                    if char_stack.is_empty() {//started reading num chars
                        char_stack_row_num = j;
                    }
                    if char_stack_row_num != j {//prevent errors at line breaks
                        if fsaw_symbol {
                            ans += consume_char_stack_to_u64(&mut char_stack);
                        } else {
                            char_stack.clear();
                        }
                        char_stack_row_num = j;
                        fsaw_symbol = false;
                    }
                    char_stack.push(num_char);
                    if !fsaw_symbol {
                        fsaw_symbol |= check_above_and_below_for_symbol(&input, (j,i));
                        if (i > 0) && (char_stack.len() == 1) {
                            fsaw_symbol |= check_above_and_below_for_symbol(&input, (j,i - 1));
                        }
                    }
                },
                _ => {
                    if char_stack.len() > 0 {//after parsing nums
                        if char_stack_row_num == j {//prevent errors at line breaks
                            fsaw_symbol |= check_above_and_below_for_symbol(&input, (j,i));
                        } 
                        if fsaw_symbol {
                            let inc = consume_char_stack_to_u64(&mut char_stack);
                            ans += inc;
                            fsaw_symbol = false;
                        } else {
                            char_stack.clear();
                        }
                    }
                },
            }
        }
    }
    ans
}

#[inline]
fn consume_char_stack_to_u64(char_stack: &mut Vec<char>) -> u64 {
    let my_str = char_stack.iter().collect::<String>();
    char_stack.clear();
    my_str.parse::<u64>().expect("Non-numeric char in stack")
}

fn check_above_and_below_for_symbol(input: &Array2<SchematicEntry>, (x,y): (usize,usize)) -> bool {
    let start = if x == 0 {
        0
    } else {
        (x - 1).clamp(0, input.dim().0)
    };
    let end = (x + 2).clamp(0, input.dim().0);
    input.slice(s![start..end, y])
        .iter()
        .any(|el| {
            match el {
                SchematicEntry::Symbol(_) => true,
                _ => false,
            }
        })
}

//#[aoc(day3, part2)]
//pub fn solve_part2(input: &[Vec<i64>]) -> i64 {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_parse_schematic_line() {
        const INPUT: &str = ".....+.58.
617*......";
        let ans = parse_schematic_line(INPUT);
        assert_eq!(ans, Ok((
                    "617*......",
                    array![
                    SchematicEntry::Dot,
                    SchematicEntry::Dot,
                    SchematicEntry::Dot,
                    SchematicEntry::Dot,
                    SchematicEntry::Dot,
                    SchematicEntry::Symbol('+'),
                    SchematicEntry::Dot,
                    SchematicEntry::Number('5'),
                    SchematicEntry::Number('8'),
                    SchematicEntry::Dot,
                    ]
                    )))
    }

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn day3_input_generator() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.dim(),(10,10));
        assert_eq!(input[[0,1]],SchematicEntry::Number('6'));
        assert_eq!(
            input.slice(s![1..3, 2..4]),
            array![
            [SchematicEntry::Dot,SchematicEntry::Symbol('*')],
            [SchematicEntry::Number('3'),SchematicEntry::Number('5')]
            ]
        );
    }

    #[test]
    fn day3_char_stack_to_u64() {
        let mut input = vec!['1', '2', '3'];
        let ans = consume_char_stack_to_u64(&mut input);
        assert_eq!(ans, 123);
        assert_eq!(input.is_empty(), true);
    }

    #[test]
    fn day3_check_above_and_below_for_symbol_1() {
        let input = input_generator(TEST_INPUT);
        let ans1 = check_above_and_below_for_symbol(&input, (0,1));
        let ans2 = check_above_and_below_for_symbol(&input, (1,1));
        let ans3 = check_above_and_below_for_symbol(&input, (9,1));
        let ans4 = check_above_and_below_for_symbol(&input, (0,3));
        let ans5 = check_above_and_below_for_symbol(&input, (1,3));
        let ans6 = check_above_and_below_for_symbol(&input, (2,3));
        let ans7 = check_above_and_below_for_symbol(&input, (7,5));
        assert_eq!(ans1, false);
        assert_eq!(ans2, false);
        assert_eq!(ans3, false);
        assert_eq!(ans4, true);
        assert_eq!(ans5, true);
        assert_eq!(ans6, true);
        assert_eq!(ans7, true);
    }


    #[test]
    fn day3_solve_p1_1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans, 4361);
    }

    #[test]
    fn day3_solve_p1_2() {
        const INPUT: &str = ".....
..123
+....";
        let input = input_generator(&INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans,0);
    }

    #[test]
    fn day3_solve_p1_3() {
        const INPUT: &str = ".....
..123
4+...";
        let input = input_generator(&INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans,127);
    }

    #[test]
    fn day3_solve_p1_4() {
        const INPUT: &str = "......
...123
4+....";
        let input = input_generator(&INPUT);
        let ans = solve_part1(&input);
        assert_eq!(ans,4);
    }
//    #[test]
//    fn day3_solve_p2() {
//    }
}
