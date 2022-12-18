use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use regex::Regex;
use lazy_static::lazy_static;
use petgraph::algo::dijkstra;
use petgraph::graph::DiGraph;

use crate::etc::id_assigner::IDAssigner;
use crate::{Solution, SolutionPair};
use std::cmp::max;
use std::fs::read_to_string;

////////////////////////////////////////////////////////////////////////////////

type ValveGraph = DiGraph<u32, ()>;

struct ValveInfo {
    flow_rate: u32,
    connections: Vec<u32>,
}

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day16.txt").unwrap();
    let mut id_assigner = IDAssigner::new();

    let valves_data: FxHashMap<u32, ValveInfo> = input.lines().map(|line| parse_line(line, &mut id_assigner)).collect();
    let nonzero_valves = valves_data.iter().filter(|(_, v)| v.flow_rate > 0).map(|(k, _)| *k).collect_vec();
    
    // Construct a graph out of the valves, to compute the path length matrix
    let edges = get_edges(&valves_data);
    let graph = ValveGraph::from_edges(&edges);
    let min_paths = compute_min_paths(&graph);
   
    let sol1 = find_solution(30, &min_paths, &valves_data, &nonzero_valves, u64::MAX);
    let mut sol2 = 0;

    for bitmask in 0..2_u64.pow(nonzero_valves.len() as u32 - 1) {
        let human_score = find_solution(26, &min_paths, &valves_data, &nonzero_valves, bitmask);
        let elephant_score = find_solution(26, &min_paths, &valves_data, &nonzero_valves, !bitmask);
        sol2 = max(sol2, human_score + elephant_score);
    }

    (Solution::from(sol1), Solution::from(sol2))
}

fn find_solution(
    time: u32, 
    path_lengths: &FxHashMap<(u32, u32), u32>, 
    valves_data: &FxHashMap<u32, ValveInfo>, 
    possible_valves: &[u32],
    bitmask: u64,
) -> u32 {
    find_optimal((0, 0, time), &mut FxHashSet::default(), path_lengths, valves_data, possible_valves, bitmask)
}

fn find_optimal(
    (current_position, acc_pressure, time_remaining): (u32, u32, u32),
    opened: &mut FxHashSet<u32>,
    path_lengths: &FxHashMap<(u32, u32), u32>,
    valves_data: &FxHashMap<u32, ValveInfo>,
    possible_valves: &[u32],
    bitmask: u64
) -> u32 {
    // Stop if we have no time left or no valves left to open
    if time_remaining == 0 || opened.len() == possible_valves.len() {
        return acc_pressure;
    }
    
    let mut best_score = acc_pressure;

    // Try the remaining valves
    for (i, &dest) in possible_valves.iter().enumerate() {
        // Skip this valve if it has already been visited, or if we would have
        // no time left to open it, of if it's not in the bitmask of allowed valves
        let travel_time = path_lengths[&(current_position, dest)];
        let mask = 1 << i;
        if bitmask & mask == 0 || opened.contains(&dest) || (travel_time + 1) >= time_remaining {
            continue;
        }

        // Go there, update the remaining time and compute how much pressure we got
        let new_time = time_remaining - travel_time - 1;
        opened.insert(dest);
        let added_score = valves_data[&dest].flow_rate * new_time;

        // Recursively search for the optimal solution from here
        let score = find_optimal((dest, acc_pressure + added_score, new_time), opened, path_lengths, valves_data, possible_valves, bitmask);
        best_score = max(score, best_score);

        // Remove the visited valve after the recursive call
        opened.remove(&dest);
    }

    best_score
}

fn compute_min_paths(graph: &ValveGraph) -> FxHashMap<(u32, u32), u32> {
    let mut paths = FxHashMap::default();

    for origin in graph.node_indices() {
        for (dest, length) in dijkstra(graph, origin, None, |_| 1) {
            let key = (origin.index() as u32, dest.index() as u32);
            paths.insert(key, length as u32);
        }
    }

    paths
}

fn get_edges(valves_data: &FxHashMap<u32, ValveInfo>) -> Vec<(u32, u32)> {
    valves_data.iter().flat_map(|(id, data)| {
        data.connections.iter().map(|conn| (*id, *conn))
    }).collect_vec()
}

fn parse_line(line: &str, id_assigner: &mut IDAssigner<String>) -> (u32, ValveInfo) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"Valve (.*) has flow rate=(\d*); tunnels? leads? to valves? (.*)").unwrap();
    }

    let groups = RE.captures(line).unwrap();
    let valve_name = groups.get(1).unwrap().as_str();
    let flow_rate = groups.get(2).unwrap().as_str().parse().unwrap();
    let conn_names = groups.get(3).unwrap().as_str();

    let valve_id = id_assigner.get_id(valve_name.to_owned());
    let connections = conn_names.split(", ").map(|spl| id_assigner.get_id(spl.to_string())).collect();

    let valve_info = ValveInfo { flow_rate, connections };
    (valve_id, valve_info)
}
