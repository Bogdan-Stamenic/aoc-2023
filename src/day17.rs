use petgraph::{graphmap::DiGraphMap, algo::dijkstra};

#[allow(dead_code)]
#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Start,
}

#[derive(Clone,Copy,Debug,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct HeatlossNode {
    coords: (usize,usize),
    dir: Direction,
    travelled: u32,
}

pub fn input_generator_p1(input: &str) -> (DiGraphMap<HeatlossNode,u32>, (usize, usize)) {
    let grid = input.lines()
        .map(|line| {
            line.chars()
                .map(|ch: char| ch.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
    .collect::<Vec<Vec<u32>>>();
    let outer_max = grid.len();
    let inner_max = grid[0].len();
    let mut out = DiGraphMap::<HeatlossNode,u32>::with_capacity(16*input.len(), 3*16*input.len());
    for i in 0..outer_max {
        for j in 0..inner_max {
            connect_edges_p1(&mut out, &grid, i, j);
        }
    }
    (out, (outer_max - 1, inner_max - 1))
}

fn connect_edges_p1(graph: &mut DiGraphMap<HeatlossNode,u32>, grid: &Vec<Vec<u32>>, i: usize, j: usize) {
    if (i == 0) && (j == 0) {
        /* special start node */
        let strt = HeatlossNode {coords: (0,0), dir: Direction::Start, travelled: 0};
        let below_strt = HeatlossNode {coords: (1,0), dir: Direction::Down, travelled:  1};
        let right_of_strt = HeatlossNode {coords: (0,1), dir: Direction::Right, travelled:  1};
        graph.add_edge(strt, below_strt, grid[1][0]);
        graph.add_edge(strt, right_of_strt, grid[0][1]);
    } else {
        /* all other nodes */
        for direction in vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down].iter() {
            for dist_travelled in 1..=3 {
                /* curent node */
                let foo = HeatlossNode {coords: (i,j), dir: *direction, travelled: dist_travelled};
                /* Don't go out of bounds && No 180° turns && Don't travel more than 3 tiles in one
                 * direction*/
                if (i < grid.len() - 1) && (*direction != Direction::Up) && passes_three_tile_check(&foo, Direction::Down){
                    let new_travelled = calc_dist_travelled(direction, Direction::Down, dist_travelled);
                    let bar = HeatlossNode {coords: (i + 1,j), dir: Direction::Down, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i + 1][j]);
                }
                if (i > 0) && (*direction != Direction::Down) && passes_three_tile_check(&foo, Direction::Up) {
                    let new_travelled = calc_dist_travelled(direction, Direction::Up, dist_travelled);
                    let bar = HeatlossNode {coords: (i - 1,j), dir: Direction::Up, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i - 1][j]);
                }
                if (j < grid[0].len() - 1) && (*direction != Direction::Left) && passes_three_tile_check(&foo, Direction::Right){
                    let new_travelled = calc_dist_travelled(direction, Direction::Right, dist_travelled);
                    let bar = HeatlossNode {coords: (i,j + 1), dir: Direction::Right, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i][j + 1]);
                }
                if (j > 0) && (*direction != Direction::Right) && passes_three_tile_check(&foo, Direction::Left) {
                    let new_travelled = calc_dist_travelled(direction, Direction::Left, dist_travelled);
                    let bar = HeatlossNode {coords: (i,j - 1), dir: Direction::Left, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i][j - 1]);
                }
            }
        }
    }
}

/* Don't travel more than 3 tiles in one direction */
#[inline]
fn passes_three_tile_check(node: &HeatlossNode, direction: Direction) -> bool {
    if (node.dir == direction) && (node.travelled == 3) {
        return false;
    }
    true
}

/* Reset distance when changing directions */
#[inline]
fn calc_dist_travelled(curr_direction: &Direction, next_direction: Direction, dist_travelled: u32) -> u32 {
    if *curr_direction == next_direction {
        return dist_travelled + 1;
    }
    1
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (graph, last_node) = input_generator_p1(input);
    let strt = HeatlossNode {coords: (0,0), dir: Direction::Start, travelled: 0};
    let ans = dijkstra(&graph, strt, None, |edge_ref| *edge_ref.2);
    ans.into_iter()
        .filter(|el| el.0.coords == last_node)
        .map(|el| el.1)
        .min().unwrap()
}

pub fn input_generator_p2(input: &str) -> (DiGraphMap<HeatlossNode,u32>, (usize, usize)) {
    let grid = input.lines()
        .map(|line| {
            line.chars()
                .map(|ch: char| ch.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
    .collect::<Vec<Vec<u32>>>();
    let outer_max = grid.len();
    let inner_max = grid[0].len();
    let mut out = DiGraphMap::<HeatlossNode,u32>::with_capacity(40*input.len(), 18*input.len());
    for i in 0..outer_max {
        for j in 0..inner_max {
            connect_edges_p2(&mut out, &grid, i, j);
        }
    }
    (out, (outer_max - 1, inner_max - 1))
}

fn connect_edges_p2(graph: &mut DiGraphMap<HeatlossNode,u32>, grid: &Vec<Vec<u32>>, i: usize, j: usize) {
    if (i == 0) && (j == 0) {
        /* special start node */
        let strt = HeatlossNode {coords: (0,0), dir: Direction::Start, travelled: 0};
        let below_strt = HeatlossNode {coords: (1,0), dir: Direction::Down, travelled:  1};
        let right_of_strt = HeatlossNode {coords: (0,1), dir: Direction::Right, travelled:  1};
        graph.add_edge(strt, below_strt, grid[1][0]);
        graph.add_edge(strt, right_of_strt, grid[0][1]);
    } else {
        /* all other nodes */
        for direction in vec![Direction::Left, Direction::Right, Direction::Up, Direction::Down].iter() {
            for dist_travelled in 1..=10 {
                /* curent node */
                let foo = HeatlossNode {coords: (i,j), dir: *direction, travelled: dist_travelled};
                /* Don't go out of bounds && No 180° turns && Don't travel more than 3 tiles in one
                 * direction*/
                if (i < grid.len() - 1) && (*direction != Direction::Up) && p2_dist_check(&foo, Direction::Down){
                    let new_travelled = calc_dist_travelled(direction, Direction::Down, dist_travelled);
                    let bar = HeatlossNode {coords: (i + 1,j), dir: Direction::Down, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i + 1][j]);
                }
                if (i > 0) && (*direction != Direction::Down) && p2_dist_check(&foo, Direction::Up) {
                    let new_travelled = calc_dist_travelled(direction, Direction::Up, dist_travelled);
                    let bar = HeatlossNode {coords: (i - 1,j), dir: Direction::Up, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i - 1][j]);
                }
                if (j < grid[0].len() - 1) && (*direction != Direction::Left) && p2_dist_check(&foo, Direction::Right){
                    let new_travelled = calc_dist_travelled(direction, Direction::Right, dist_travelled);
                    let bar = HeatlossNode {coords: (i,j + 1), dir: Direction::Right, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i][j + 1]);
                }
                if (j > 0) && (*direction != Direction::Right) && p2_dist_check(&foo, Direction::Left) {
                    let new_travelled = calc_dist_travelled(direction, Direction::Left, dist_travelled);
                    let bar = HeatlossNode {coords: (i,j - 1), dir: Direction::Left, travelled: new_travelled};
                    graph.add_edge(foo, bar, grid[i][j - 1]);
                }
            }
        }
    }
}

/* Travel between 4 and 10 tiles */
#[inline]
fn p2_dist_check(node: &HeatlossNode, direction: Direction) -> bool {
    if (node.dir != direction) && (node.travelled < 4) {
        return false;
    }
    if (node.dir == direction) && (node.travelled == 10) {
        return false;
    }
    true
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let input = input_generator_p2(input);
    let (input, last_node) = input;
    let strt = HeatlossNode {coords: (0,0), dir: Direction::Start, travelled: 0};
    let ans = dijkstra(&input, strt, None, |edge_ref| *edge_ref.2);
    println!("{}", ans.is_empty());
    ans.into_iter()
        .filter(|el| el.0.coords == last_node)
        .map(|el| el.1)
        .min().expect("End is unreachable")
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
//13 * 13

    #[test]
    fn day17_p1_1() {
        let ans = solve_part1(TEST_INPUT);
        assert_eq!(ans, 102);
    }

    #[test]
    fn day17_p1_2() {
        const INPUT: &str ="11999
91199
99119
99911";
        let ans = solve_part1(INPUT);
        assert_eq!(ans, 7);
    }

    /* Can go 3 tiles in one direction, but no further */
    #[test]
    fn day17_p1_3() {
        const INPUT: &str ="10000
99110
99110
99010
99990";
//1....
//9911.
//9911.
//99.1.
//9999.
        let ans = solve_part1(INPUT);
        assert_eq!(ans, 1);
    }

    /* Can snake back and forth */
    #[test]
    fn day17_p1_4() {
        const INPUT: &str ="000099
999009
999909
000009
019999
099009
000000";
        let ans = solve_part1(INPUT);
        assert_eq!(ans, 1);
    }

    #[test]
    fn day17_p2_1() {
        let ans = solve_part2(TEST_INPUT);
        assert_eq!(ans, 94);
    }
}
