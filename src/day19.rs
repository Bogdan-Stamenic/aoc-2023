use nom::{
    Parser,
    IResult,
    branch::alt,
    combinator::all_consuming,
    bytes::complete::{tag, take_while1},
    multi::{separated_list1, many_till, many1}, sequence::{separated_pair, preceded, tuple},
};

enum PartProperty {
    X,
    M,
    A,
    S
}

struct MachinePart{
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

enum WorkflowTest {
    LessThan(i32),
    GreaterThan(i32),
}

struct WorkflowRule {
    test: WorkflowTest,
    jmp: String,
}

struct Workflow {
    name: String,
    rules: Vec<(PartProperty, WorkflowRule)>,
    default: String
}

//#[aoc_generator(day19)]
//pub fn input_generator(input: &str) -> Almanac {
//    match all_consuming(separated_pair(parse_intervals, tag("\n"), parse_machine_parts))
//        .map(|(sds, maps)| Almanac{seeds: sds, almanac_maps: maps})
//        .parse(input)
//    {
//        Ok((_, val)) => val,
//        Err(e) => panic!("{}", e),
//    }
//}

//#[aoc(day19, part1)]
//pub fn solve_part1(input: &Almanac) -> i64 {
//}


//#[aoc(day19, part2)]
//pub fn solve_part2(input: &Almanac) -> i64 {
//}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

//    #[test]
//    fn test_day5_parser() {
//    }
//
//    #[test]
//    fn test_interval_splitting() {
//        let ans1 = AlmanacMap::split_interval_for_mapping((2,42), (16,25));
//        let ans2 = AlmanacMap::split_interval_for_mapping((10,20), (18,25));
//        let ans3 = AlmanacMap::split_interval_for_mapping((46,57), (56,93));
//        assert_eq!(ans1, SplitStatus::Split(((16,25), VecDeque::from([(2,16),(25,42)]))));
//        assert_eq!(ans2, SplitStatus::Split(((18,20), VecDeque::from([(10,18)]))));
//        assert_eq!(ans3, SplitStatus::Split(((56,57), VecDeque::from([(46,56)]))));
//    }
}
