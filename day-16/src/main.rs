use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rayon::prelude::*;

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    tunnels: Vec<&'a str>,
}

fn parse_line(line: &str) -> Option<Valve> {
    let line = line.strip_prefix("Valve ")?;
    let name = &line[..2];
    let line = &line[2..].strip_prefix(" has flow rate=")?;
    let (flow_rate, line) = line.split_once("; tunnel")?;
    let (_, line) = line.split_once("valve")?;
    let (_, line) = line.split_once(' ')?;
    let tunnels: Vec<_> = line.split(", ").collect();

    Some(Valve {
        name,
        flow_rate: flow_rate.parse().ok()?,
        tunnels,
    })
}

fn parse_input(input: &str) -> Vec<Valve> {
    input.lines().filter_map(parse_line).collect()
}

fn key<'a>(left: &'a str, right: &'a str) -> (&'a str, &'a str) {
    if left < right {
        (left, right)
    } else {
        (right, left)
    }
}

fn floyd<'a>(valves: &Vec<Valve<'a>>) -> HashMap<(&'a str, &'a str), u32> {
    let mut graph: HashMap<(&str, &str), u32> = HashMap::new();

    for i in valves {
        for j in &i.tunnels {
            graph.insert(key(i.name, j), 1);
        }
    }

    for Valve { name: k, .. } in valves {
        for Valve { name: i, .. } in valves {
            for Valve { name: j, .. } in valves {
                let min = u32::min(
                    *graph.get(&key(i, j)).unwrap_or(&u32::MAX),
                    graph
                        .get(&key(i, k))
                        .unwrap_or(&u32::MAX)
                        .saturating_add(*graph.get(&key(k, j)).unwrap_or(&u32::MAX)),
                );

                graph.insert(key(i, j), min);
            }
        }
    }

    graph
}

/// Compute the solution to part 1
fn part_1(input: &str) -> String {
    let valves = parse_input(input);

    let graph = floyd(&valves);

    fn traverse(
        mut time: u32,
        pos: &str,
        valves: &Vec<Valve>,
        opened: &HashSet<&str>,
        graph: &HashMap<(&str, &str), u32>,
    ) -> u32 {
        if time == 0 {
            return 0;
        }

        let valve = valves.iter().find(|v| v.name == pos).unwrap();
        let mut flow = 0;
        let mut opened = opened.clone();

        if valve.flow_rate > 0 {
            time -= 1;
            flow = valve.flow_rate * time;
            opened.insert(pos);
        }

        let sub_flow = valves
            .iter()
            .filter(|v| v.flow_rate > 0 && !opened.contains(v.name))
            .filter(|v| graph[&key(pos, v.name)] < time)
            .map(|v| {
                traverse(
                    time - graph[&key(pos, v.name)],
                    v.name,
                    valves,
                    &opened,
                    graph,
                )
            })
            .max()
            .unwrap_or(0);

        flow + sub_flow
    }

    let res = traverse(30, "AA", &valves, &HashSet::new(), &graph);

    res.to_string()
}

/// Compute the solution to part 2
fn part_2(input: &str) -> String {
    let valves = parse_input(input);

    let graph = floyd(&valves);

    fn traverse(
        mut time: u32,
        mut me: (&str, u32),
        mut el: (&str, u32),
        valves: &Vec<Valve>,
        opened: &HashSet<&str>,
        graph: &HashMap<(&str, &str), u32>,
    ) -> u32 {
        if time == 0 {
            return 0;
        }

        if opened.len() == valves.iter().filter(|v| v.flow_rate > 0).count() {
            return 0;
        }

        time -= 1;

        let valve_me = valves.iter().find(|v| v.name == me.0).unwrap();
        let valve_el = valves.iter().find(|v| v.name == el.0).unwrap();

        me.1 -= 1;
        el.1 -= 1;

        let mut flow = 0;
        let mut opened = opened.clone();

        if valve_me.flow_rate > 0 && !opened.contains(&me.0) && me.1 == 0 {
            flow += valve_me.flow_rate * time;
            me.1 += 1;
            opened.insert(me.0);
        }

        if valve_el.flow_rate > 0 && !opened.contains(&el.0) && el.1 == 0 {
            flow += valve_el.flow_rate * time;
            el.1 += 1;
            opened.insert(el.0);
        }

        let unopened_valves: Vec<_> = valves
            .iter()
            .filter(|v| !opened.contains(v.name))
            .filter(|v| v.flow_rate > 0)
            .filter(|v| v.name != me.0 && v.name != el.0)
            .map(|v| v.name)
            .collect();

        let subflow = match (me.1, el.1) {
            (0, 0) => unopened_valves
                .iter()
                .enumerate()
                .flat_map(|(idx, v1)| unopened_valves[idx + 1..].iter().map(move |v2| (v1, v2)))
                .collect::<Vec<_>>()
                .par_iter()
                .map(|(me_n, el_n)| {
                    traverse(
                        time,
                        (me_n, graph[&key(me.0, me_n)]),
                        (el_n, graph[&key(el.0, el_n)]),
                        valves,
                        &opened,
                        graph,
                    )
                })
                .max()
                .unwrap_or(0),
            (0, _) => unopened_valves
                .par_iter()
                .map(|v| traverse(time, (v, graph[&key(me.0, v)]), el, valves, &opened, graph))
                .max()
                .unwrap_or(0),
            (_, 0) => unopened_valves
                .par_iter()
                .map(|v| traverse(time, me, (v, graph[&key(el.0, v)]), valves, &opened, graph))
                .max()
                .unwrap_or(0),
            _ => traverse(time, me, el, valves, &opened, graph),
        };

        flow + subflow
    }

    let res = traverse(26, ("AA", 1), ("AA", 1), &valves, &HashSet::new(), &graph);

    res.to_string()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Advent of Code 2022 - Day 16");

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
