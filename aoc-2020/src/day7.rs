use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::Read};
use petgraph::{graph::NodeIndex, Direction, Graph};
use regex::Regex;

const START: &str = "shiny gold";
const PART: &str = "part2";

struct WeightedNode {
  count: u32,
}

pub fn solve(mut input: File) -> u32 {
  let mut input_str = String::new();
  input
    .read_to_string(&mut input_str)
    .unwrap();

  let input: Vec<&str> = input_str
    .lines()
    .collect();

  return if PART == "part1" { part1(input) } else { part2(input) };
}

pub fn part1(input: Vec<&str>) -> u32 {
  let mut graph = Graph::<String, ()>::new();
  let mut nodes = HashMap::<String, NodeIndex>::new();

  for &rule in &input {
    let re = Regex::new(r"(.+) bags contain (.+)").unwrap();

    let Some(caps) = re.captures(rule) else {
      panic!("No match");
    };

    // println!("{:?}", caps);
    let root = get_or_insert_node(&mut graph, &mut nodes, &caps[1]);

    let re = Regex::new(r"(\d+) (.+?) bags?\.?$").unwrap();

    let children: Vec<&str> = (&caps[2])
    .split(", ")
    .collect();

    for child in children {
      if let Some(cap) = re.captures(child) {
        let child_idx = get_or_insert_node(&mut graph, &mut nodes, &cap[2]);
        graph.add_edge(root, child_idx, ());
      }
    }
  }

  // println!("Graph: {:?}\nNodes: {:?}", graph, nodes);

  // Do a breadth first search and count ALL of the nodes GOING IN to the root node aka START
  let root = *nodes.get(&START.to_string()).unwrap();
  let mut visited = HashSet::new();
  let mut queue = VecDeque::new();

  queue.push_back(root);

  while let Some(node) = queue.pop_front() {
    // println!("Visited: {:?}", graph[node]);

    for parent in graph.neighbors_directed(node, Direction::Incoming) {
      if visited.insert(parent) {
        queue.push_back(parent);
      }
    }
  }

  return visited.len().try_into().unwrap();
}

pub fn part2(input: Vec<&str>) -> u32 {
  let mut graph = Graph::<WeightedNode, ()>::new();
  let mut nodes = HashMap::<String, NodeIndex>::new();

  for &rule in &input {
    let re = Regex::new(r"(.+) bags contain (.+)").unwrap();

    let Some(caps) = re.captures(rule) else {
      panic!("No match");
    };

    // println!("{:?}", caps);
    let root = get_or_insert_weighted_node(&mut graph, &mut nodes, &caps[1], 1);

    let re = Regex::new(r"(\d+) (.+?) bags?\.?$").unwrap();

    let children: Vec<&str> = (&caps[2])
    .split(", ")
    .collect();

    for child in children {
      if let Some(cap) = re.captures(child) {
        let child_idx = get_or_insert_weighted_node(&mut graph, &mut nodes, &cap[2], (&cap[1]).parse().unwrap());
        graph.add_edge(root, child_idx, ());
      }
    }
  }

  // println!("Graph: {:?}\nNodes: {:?}", graph, nodes);

  // Do a breadth first search and count ALL of the nodes GOING OUT to the root node aka START
  let root = *nodes.get(&START.to_string()).unwrap();

  // let mut visited = HashSet::new();
  // let mut queue = VecDeque::new();
  // let mut depth = HashMap::<NodeIndex, u32>::new();

  // queue.push_back(root);
  // depth.insert(root, 0);

  // while let Some(node) = queue.pop_front() {
  //   // println!("Visited: {} {}", graph[node].name, graph[node].count);

  //   for child in graph.neighbors_directed(node, Direction::Outgoing) {
  //     if visited.insert(child) {
  //       queue.push_back(child);
  //       depth.insert(child, *depth.get(&node).unwrap() + 1);
  //     }
  //   }
  // }

  // println!("{:?}", nodes);
  // println!("{:?}", depth);

  return rec_bfs(&graph, &mut nodes, root) - 1;
}

fn rec_bfs(graph: &Graph<WeightedNode, ()>, nodes: &mut HashMap<String, NodeIndex>, curr: NodeIndex) -> u32 {
  let curr_node = &graph[curr];

  let mut res: u32 = 0;
  for child in graph.neighbors_directed(curr, Direction::Outgoing) {
    res += rec_bfs(graph, nodes, child);
  }

  curr_node.count * (1 + res)
}

fn get_or_insert_node(graph: &mut Graph<String, ()>, nodes: &mut HashMap<String, NodeIndex>, new: &str) -> NodeIndex {
  if let Some(&idx) = nodes.get(new) {
    idx
  } else {
    let idx = graph.add_node(new.to_string());
    nodes.insert(new.to_string(), idx);
    idx
  }
}

fn get_or_insert_weighted_node(graph: &mut Graph<WeightedNode, ()>, nodes: &mut HashMap<String, NodeIndex>, new: &str, count: u32) -> NodeIndex {
  if let Some(&idx) = nodes.get(new) {
    idx
  } else {
    let idx = graph.add_node(WeightedNode { count });
    nodes.insert(new.to_string(), idx);
    idx
  }
}