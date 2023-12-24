use std::collections::HashSet;
use petgraph::{prelude::{DiGraphMap,UnGraphMap}, algo::simple_paths::all_simple_paths};

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct HikingTrailP1 {
    path: DiGraphMap<(usize,usize), ()>,
    start: (usize,usize),
    end: (usize, usize),
}

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct HikingTrailP2 {
    path: UnGraphMap<(usize,usize), u64>,
    start: (usize,usize),
    end: (usize, usize),
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line: &str| {
            line.bytes()
                .collect::<Vec<u8>>()
        })
    .collect::<Vec<Vec<u8>>>()
}

fn graph_for_p1(input: &Vec<Vec<u8>>) -> HikingTrailP1 {
    let mut hiking_trail = DiGraphMap::<(usize,usize), ()>::new();
    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            match val {
                b'.' => {
                connect_down(&mut hiking_trail, &input, i, j);
                connect_up(&mut hiking_trail, &input, i, j);
                connect_right(&mut hiking_trail, &input, i, j);
                connect_left(&mut hiking_trail, &input, i, j);
                },
                b'v' => connect_down(&mut hiking_trail, &input, i, j),
                b'^' => connect_up(&mut hiking_trail, &input, i, j),
                b'>' => connect_right(&mut hiking_trail, &input, i, j),
                b'<' => connect_left(&mut hiking_trail, &input, i, j),
                b'#' => {/* do nothing */},
                _ => unreachable!("Unexpected character was encountered"),
            };
        }
    }
    let start_node = input[0].iter()
        .enumerate()
        .filter(|(_,x)| **x == b'.')
        .map(|(idx,_)| (0usize, idx))
        .next().unwrap();
    let end_node = input[input.len() - 1].iter()
        .enumerate()
        .filter(|(_,x)| **x == b'.')
        .map(|(idx,_)| (input.len() - 1, idx))
        .next().unwrap();
    HikingTrailP1 { path: hiking_trail, start: start_node, end: end_node }
}

fn connect_left(hiking_trail: &mut DiGraphMap::<(usize,usize), ()>,
    trail: &Vec<Vec<u8>>, i: usize, j: usize) {
    if j > 0 {
        if trail[i][j-1] == b'<' || trail[i][j-1] == b'.' {
            hiking_trail.add_edge((i,j), (i,j-1), ());
        }
    }
}

fn connect_right(hiking_trail: &mut DiGraphMap::<(usize,usize), ()>,
    trail: &Vec<Vec<u8>>, i: usize, j: usize) {
    let inner_max = trail[0].len() - 1;
    if j < inner_max {
        if trail[i][j+1] == b'>' || trail[i][j+1] == b'.' {
            hiking_trail.add_edge((i,j), (i,j+1), ());
        }
    }
}

fn connect_down(hiking_trail: &mut DiGraphMap::<(usize,usize), ()>,
    trail: &Vec<Vec<u8>>, i: usize, j: usize) {
    let outer_max = trail.len() - 1;
    if i < outer_max {
        if trail[i+1][j] == b'v' || trail[i+1][j] == b'.' {
            hiking_trail.add_edge((i,j), (i+1,j), ());
        }
    }
}

fn connect_up(hiking_trail: &mut DiGraphMap::<(usize,usize), ()>,
    trail: &Vec<Vec<u8>>, i: usize, j: usize) {
    if i > 0 {
        if trail[i-1][j] == b'^' || trail[i-1][j] == b'.' {
            hiking_trail.add_edge((i,j), (i-1,j), ());
        }
    }
}

#[aoc(day23,part1)]
pub fn solve_day23_p1(input: &Vec<Vec<u8>>) -> usize {
    let hiking_trail_chars = graph_for_p1(input);
    let HikingTrailP1 { path: hiking_trail, start: start_node, end: end_node } = hiking_trail_chars;
    let ways = all_simple_paths(&hiking_trail, start_node, end_node, 1, None).collect::<Vec<Vec<_>>>();
    ways.into_iter()
        .map(|path| path.len() - 1)
        .max().unwrap()
}

fn graph_for_p2(input: &Vec<Vec<u8>>) -> HikingTrailP2 {
    let mut hiking_trail = UnGraphMap::<(usize,usize), u64>::new();
    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            match val {
                b'.' => connect_path(&mut hiking_trail, &input, i, j),
                b'v' => connect_path(&mut hiking_trail, &input, i, j),
                b'^' => connect_path(&mut hiking_trail, &input, i, j),
                b'>' => connect_path(&mut hiking_trail, &input, i, j),
                b'<' => connect_path(&mut hiking_trail, &input, i, j),
                b'#' => {/* do nothing */},
                _ => unreachable!("Unexpected character was encountered"),
            };
        }
    }
    let start_node = input[0].iter()
        .enumerate()
        .filter(|(_,x)| **x == b'.')
        .map(|(idx,_)| (0usize, idx))
        .next().unwrap();
    let end_node = input[input.len() - 1].iter()
        .enumerate()
        .filter(|(_,x)| **x == b'.')
        .map(|(idx,_)| (input.len() - 1, idx))
        .next().unwrap();
    /* Reduce nodes to make solution more computationally feasible */
    let out_graph = prune_graph_for_p2(hiking_trail);
    HikingTrailP2 { path: out_graph, start: start_node, end: end_node }
    //HikingTrailP2 { path: hiking_trail, start: start_node, end: end_node }
}

#[allow(dead_code)]
fn prune_graph_for_p2(graph: UnGraphMap::<(usize,usize), u64>) -> UnGraphMap::<(usize,usize),u64> {
    let mut out_graph = UnGraphMap::<(usize,usize), u64>::new();
    let junctions = graph.nodes()
        .filter(|n| graph.neighbors(*n).count() != 2)
        .collect::<HashSet<(usize,usize)>>();
    for node in junctions.iter() {
        /* traverse neighbours until adjacent junction is encountered */
        let mut visited = HashSet::from([*node]);
        for next_node in graph.neighbors(*node) {
            let mut dist = 1;
            let mut curr_node = next_node;
            loop {
                /* Connect if both nodes are junctions */
                if junctions.contains(&curr_node) {
                    out_graph.add_edge(curr_node, *node, dist);
                    break;
                }
                visited.insert(curr_node);
                dist += 1;
                let new_curr_node = graph.neighbors(curr_node)
                    .filter(|x| {
                        !visited.contains(x)
                    }).next().unwrap();
                curr_node = new_curr_node;
            }
        }
    }
    out_graph
}

fn connect_path(hiking_trail: &mut UnGraphMap::<(usize,usize), u64>,
    trail: &Vec<Vec<u8>>, i: usize, j: usize) {
    let outer_max = trail.len() - 1;
    let inner_max = trail[0].len() - 1;
    if j > 0 {
        if trail[i][j-1] != b'#' {
            hiking_trail.add_edge((i,j), (i,j-1), 1);
        }
    }
    if j < inner_max {
        if trail[i][j+1] != b'#' {
            hiking_trail.add_edge((i,j), (i,j+1), 1);
        }
    }
    if i < outer_max {
        if trail[i+1][j] != b'#' {
            hiking_trail.add_edge((i,j), (i+1,j), 1);
        }
    }
    if i > 0 {
        if trail[i-1][j] != b'#' {
            hiking_trail.add_edge((i,j), (i-1,j), 1);
        }
    }
}

#[aoc(day23,part2)]
pub fn solve_day23_p2(input: &Vec<Vec<u8>>) -> u64 {
    let hiking_trail_chars = graph_for_p2(input);
    let HikingTrailP2 { path: hiking_trail, start: start_node, end: end_node } = hiking_trail_chars;
    let ways = all_simple_paths(&hiking_trail, start_node, end_node, 1, None).collect::<Vec<Vec<_>>>();
    ways.into_iter()
        .map(|path_vec| {
            path_vec.windows(2)
                .map(|wsl| {
                    let [p1,p2] = wsl else {unreachable!()};
                    hiking_trail.edge_weight(*p1,*p2).unwrap()
                })
                .sum()
        })
    .max().expect("no simple paths found")
    //ways.into_iter()
    //    .map(|path| path.len() - 1)
    //    .max().expect("no simple paths found")
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str =
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

// Helped me solve p2
//"#.#####################
//#.......#########...###
//#######.#########.#.###
//###.....#..*..###.#.###
//###.#####.#.#.###.#.###
//###*....#.#.#.....#...#
//###.###.#.#.#########.#
//###...#.#.#.......#...#
//#####.#.#.#######.#.###
//#.....#.#.#.......#...#
//#.#####.#.#.#########.#
//#.#...#...#...###....*#
//#.#.#.#######.###.###.#
//#...#*..#....*..#.###.#
//#####.#.#.###.#.#.###.#
//#.....#...#...#.#.#...#
//#.#########.###.#.#.###
//#...###...#...#...#.###
//###.###.#.###.#####.###
//#...#...#.#..*..#..*###
//#.###.###.#.###.#.#.###
//#.....###...###...#...#
//#####################.#";
//junctions = { (0,1), (3,11), (5,3), (11,21), (13,5), (13,13), (19,13), (19,19), (22,21)}
    #[test]
    fn day23_input_p1() {
        let input = input_generator(TEST_INPUT);
        let foo = graph_for_p1(&input);
        let HikingTrailP1 { path: hiking_trail, start: start_node, end: end_node } = foo;
        assert_eq!(start_node, (0,1));
        assert_eq!(end_node, (22,21));
        assert_eq!(hiking_trail.node_count(), 213);
    }

    #[test]
    fn day23_input_p2() {
        let input = input_generator(TEST_INPUT);
        let foo = graph_for_p2(&input);
        let HikingTrailP2 { path: hiking_trail, start: start_node, end: end_node } = foo;
        assert_eq!(start_node, (0,1));
        assert_eq!(end_node, (22,21));
        assert_eq!(hiking_trail.node_count(), 9);
    }

    #[test]
    fn day23_solve_p1() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_day23_p1(&input);
        assert_eq!(ans, 94);
    }

    #[test]
    fn day23_solve_p2() {
        let input = input_generator(TEST_INPUT);
        let ans = solve_day23_p2(&input);
        assert_eq!(ans, 154);
    }
}
