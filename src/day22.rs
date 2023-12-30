use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use nom::{
    Parser,
    IResult,
    combinator::all_consuming,
    bytes::complete::{tag, take_while1},
    multi::separated_list1, sequence::{separated_pair, tuple},
};

#[derive(Clone,Debug,Default)]
struct BrickAdjNode {
    below_set: HashSet<usize>,//all bricks beneath current one
    above_set: HashSet<usize>,//all bricks above current one
}

#[allow(dead_code)]
#[derive(Clone,Debug,Default)]
pub struct FallenBricks {
    brick_locations: Vec<(usize, (u32,u32,u32),(u32,u32,u32))>,
    adjacent_bricks: Vec<BrickAdjNode>
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> FallenBricks {
    let falling_bricks = file_to_brick_vec(input);
    let mut space = HashMap::<(u32,u32,u32), usize>::new();
    let piled_bricks = drop_and_stack_all_bricks(falling_bricks, &mut space);
    let adjacent = calc_brick_adjacency(&piled_bricks, &space);
    FallenBricks {brick_locations: piled_bricks, adjacent_bricks: adjacent}
}

fn file_to_brick_vec(input: &str) -> Vec<(usize, (u32,u32,u32),(u32,u32,u32))> {
    let mut bricks_vec = match all_consuming(separated_list1(tag("\n"), parse_brick_line))
        .parse(input) {
        Ok((_,val)) => val,
        Err(e) => panic!("While parsing : {}", e),
    };
    bricks_vec.sort_unstable_by_key(|x| x.0.2);//sort by z1 coordinate
    bricks_vec.into_iter()
        .enumerate()
        .map(|(idx,(p1, p2))| (idx,p1,p2))
        .collect::<Vec<(usize, (u32,u32,u32), (u32,u32,u32))>>()
        /*Vec<( brick_id , (x1,y1,z1) , (x2,y2,z2) )>*/
}

#[inline]
fn parse_brick_line(input: &str) -> IResult<&str, ((u32,u32,u32), (u32,u32,u32))> {
    separated_pair(parse_brick_coords, tag("~"), parse_brick_coords)
        .parse(input)
}

#[inline]
fn parse_brick_coords(input: &str) -> IResult<&str, (u32,u32,u32)> {
    tuple((
            parse_num_to_u32,
            tag(","),
            parse_num_to_u32,
            tag(","),
            parse_num_to_u32,
            ))
        .map(|x| (x.0, x.2, x.4))
        .parse(input)
}

fn parse_num_to_u32(input: &str) -> IResult<&str, u32> {
    take_while1(char::is_numeric)
        .map(|x: &str| x.parse::<u32>().unwrap())
        .parse(input)
}

/* Expects bricks to be sorted in ascending z1 order */
#[inline]
fn drop_and_stack_all_bricks(
    falling_bricks: Vec<(usize,(u32,u32,u32),(u32,u32,u32))>,
    space: &mut HashMap<(u32,u32,u32),usize>)
    -> Vec<(usize,(u32,u32,u32),(u32,u32,u32))>
{
    let mut pile_of_bricks = Vec::<(usize, (u32,u32,u32), (u32,u32,u32))>::new();
    for brick in falling_bricks.into_iter() {
        let (brick_id, mut p1, mut p2) = brick;
        while p1.2 > 1 && (p1.0..=p2.0).cartesian_product(p1.1..=p2.1)
            .all(|(x,y)| !space.contains_key(&(x,y,p1.2-1))) {
                p1.2 -= 1;// z1--
                p2.2 -= 1;// z2--
        }
        let positions = (p1.0..=p2.0).cartesian_product(p1.1..=p2.1)
            .cartesian_product(p1.2..=p2.2);
        space.extend(positions.map(|((x,y),z)| ((x,y,z), brick_id)));
        pile_of_bricks.push((brick_id, p1, p2))
    }
    pile_of_bricks
}

#[inline]
fn calc_brick_adjacency(
    bricks_vec: &[(usize,(u32,u32,u32),(u32,u32,u32))],
    space: &HashMap<(u32,u32,u32), usize>)
    -> Vec<BrickAdjNode>
{
    let mut adjacent = vec![BrickAdjNode::default(); bricks_vec.len()];
    for (brick_id, (x1,y1,z1), (x2,y2,_)) in bricks_vec.iter() {
        for (x,y) in (*x1..=*x2).cartesian_product(*y1..=*y2) {
            if let Some(id_below) = space.get(&(x,y,z1-1)) {
                adjacent[*id_below].above_set.insert(*brick_id);
                adjacent[*brick_id].below_set.insert(*id_below);
            }
        }
    }
    adjacent
}

fn can_be_disintegrated_p1(adjacent_bricks: &[BrickAdjNode], brick_id: usize) -> bool {
    for bid in adjacent_bricks[brick_id].above_set.iter() {
        if adjacent_bricks[*bid].below_set.iter().all(|x| *x == brick_id) {
            /* brick_id is only supporting block -> cannot be disintegrated*/
            return false;
        }
    }
    true
}

#[aoc(day22,part1)]
pub fn solve_day22_p1(input: &FallenBricks) -> usize {
    let FallenBricks {brick_locations: bricks, adjacent_bricks: adjacent} = input;
    bricks.into_iter()
        .map(|(brick_id,_,_)| brick_id)
        .filter(|bid| can_be_disintegrated_p1(adjacent, **bid))
        .count()
}

fn disintegration_cascade_p2(adjacent_bricks: &[BrickAdjNode], falling_bricks: &mut HashSet<usize>, brick_id: usize) {
    if !falling_bricks.insert(brick_id) {
        return;
    }
    for abv in adjacent_bricks[brick_id].above_set.iter() {
        if adjacent_bricks[*abv].below_set.iter().all(|x| falling_bricks.contains(x)) {
            disintegration_cascade_p2(adjacent_bricks, falling_bricks, *abv);
        }
    }
}

#[aoc(day22,part2)]
pub fn solve_day22_p2(input: &FallenBricks) -> usize {
    let FallenBricks {brick_locations: bricks, adjacent_bricks: adjacent} = input;
    let mut falling = HashSet::<usize>::new();
    let mut cascade_lengths = vec![0usize; bricks.len()];
    for (brick_id,_,_) in bricks.iter() {
        falling.clear();
        disintegration_cascade_p2(adjacent, &mut falling, *brick_id);
        cascade_lengths[*brick_id] = falling.iter().count();
    }
    cascade_lengths.iter()
        .map(|x| x - 1)
        .sum::<usize>()
}

#[cfg(test)]
mod test{
    use super::*;
    const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn day22_input() {
        let input = input_generator(TEST_INPUT);
        assert_eq!(input.adjacent_bricks.len(), 7);
        assert_eq!(input.adjacent_bricks[0].below_set, HashSet::from([]));
        assert_eq!(input.adjacent_bricks[0].above_set, HashSet::from([1usize,2usize]));
        assert_eq!(input.adjacent_bricks[3].below_set, HashSet::from([1,2]));//D
        assert_eq!(input.adjacent_bricks[5].above_set, HashSet::from([6]));//F
        assert_eq!(input.adjacent_bricks[6].above_set, HashSet::from([]));
        assert_eq!(input.adjacent_bricks[6].below_set, HashSet::from([5]));
    }

    #[test]
    fn day22_can_be_disintegrated_p1() {
        let input = input_generator(TEST_INPUT);
        let FallenBricks {brick_locations: _, adjacent_bricks: adjacent} = input;
        assert_eq!(can_be_disintegrated_p1(&adjacent, 0), false);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 1), true);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 2), true);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 3), true);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 4), true);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 5), false);
        assert_eq!(can_be_disintegrated_p1(&adjacent, 6), true);
    }
    
    #[test]
    fn day22_solve_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_day22_p1(&input);
        assert_eq!(ans, 5);
    }

    #[test]
    fn day22_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_day22_p2(&input);
        assert_eq!(ans, 7);
    }
}
