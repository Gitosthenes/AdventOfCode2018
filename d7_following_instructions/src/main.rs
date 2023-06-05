use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error, fs, result,
};

type Result<T> = result::Result<T, Box<dyn error::Error>>;

// Config
const INPUT_FILE: &str = "input.txt";
const NUM_WORKERS: usize = 5;
const BASE_TIME: usize = 60;

fn main() -> Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let mut graph = Graph::from_input(&input);

    let order = graph.op_alignment();
    let time = graph.op_multi_threaded(&order);

    println!("Part 1: {}", order);
    println!("Part 2: {}", time);

    Ok(())
}

// `Graph` maps the parent/child relationships between steps
//
//  *first: A sorted list of steps that can be done first (i.e. no prerequisites).
//          This vector is used to initialize the queue in op_alignment
//
//  *step_to_next: Maps each step to a list of "child" steps
//                (i.e. steps where this one is a prerequisite).
//
//  *step_to_prev: Maps each step to a list of "parent" steps
//                (i.e. steps hat are a prerequisite to this one).
#[derive(Debug)]
struct Graph {
    first: Vec<String>,
    step_to_next: HashMap<String, Vec<String>>,
    step_to_prev: HashMap<String, Vec<String>>,
}

impl Graph {
    // Part 0: Create `Graph` struct from the input text
    fn from_input(input: &str) -> Self {
        static PATTERN: &str = r"Step\s(?P<parent>[A-Z]{1})\smust\sbe\sfinished\sbefore\sstep\s(?P<child>[A-Z]{1})\scan\sbegin.";
        let rgx = Regex::new(PATTERN).expect("failed to initialize regex");

        // For step/next-step relationaships
        let mut step_to_next: HashMap<String, Vec<String>> = HashMap::new();
        // For step/prev-step relationaships
        let mut step_to_prev: HashMap<String, Vec<String>> = HashMap::new();

        for line in input.lines() {
            let caps = rgx.captures(line).expect("failed to parse line");
            let (parent, child) = (&caps[1], &caps[2]);

            // Add step/next-step relationship
            step_to_next
                .entry(parent.to_string())
                .and_modify(|children| children.push(child.to_string()))
                .or_insert(vec![child.to_string()]);

            // Add step/prev-step relationship
            step_to_prev
                .entry(child.to_string())
                .and_modify(|prev_steps| prev_steps.push(parent.to_string()))
                .or_insert(vec![parent.to_string()]);

            // Initialize opposite entry for first/last steps
            step_to_next.entry(child.to_string()).or_insert(vec![]);
            step_to_prev.entry(parent.to_string()).or_insert(vec![]);
        }

        // Find first step (only step with no prerequisites)
        let mut first = step_to_prev
            .iter() // Iterate over map entries
            .filter(|e| e.1.is_empty()) // Only keep steps with no prereqs
            .map(|e| e.0) // keep just the steps; (k, v) -> k
            .cloned() // &String -> String
            .collect::<Vec<String>>(); // Consume iterator

        first.sort();

        Graph {
            first,
            step_to_next,
            step_to_prev,
        }
    }

    // Part 1: Calculate correct order of steps
    fn op_alignment(&mut self) -> String {
        // Put steps in order
        let mut queue = self.first.clone();
        let mut processed = String::new();
        let mut visited = HashSet::<String>::new();

        while !queue.is_empty() {
            // Find next step where prereq steps completed
            let mut next_step = String::new();
            for (i, step) in queue.iter().enumerate() {
                let prereqs = self.step_to_prev.get(step).expect("failed to get prereqs");
                let mut prereqs_met = true;

                for pre in prereqs {
                    if !processed.contains(pre) {
                        prereqs_met = false;
                        break;
                    }
                }

                if prereqs_met {
                    next_step = step.to_string();
                    queue.remove(i);
                    break;
                }
            }

            if visited.contains(&next_step) {
                continue;
            }

            // Get steps that next_step is prereq for
            let next_children: &mut Vec<String> = self
                .step_to_next
                .get_mut(&next_step)
                .expect("failed to get children");

            // Record next step
            processed.push_str(&next_step);
            visited.insert(next_step);
            // Append child steps to queue
            queue.append(next_children);
            // Sort queue for next iteration
            queue.sort();
        }

        processed
    }

    // Part 2: Calculate time to complete steps w/ `NUM_WORKERS`
    fn op_multi_threaded(&self, order: &str) -> usize {
        // Map each step to the time it takes to complete
        let time_per_step = "_ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .map(|(i, c)| (c.to_string(), BASE_TIME + i))
            .collect::<HashMap<String, usize>>();

        // Convert `order` string into Vec<String> so that individual steps can be removed
        let mut steps_todo = order
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        // Used for checking if a step can be started
        let mut steps_done: Vec<String> = vec![];

        // Initialize workers
        let mut labor_pool = vec![Worker::new(); NUM_WORKERS];

        // Process work
        let mut time_taken = 0;
        while !steps_todo.is_empty() {
            // Record completed work before assigning more
            for worker in labor_pool.iter_mut() {
                //if a worker is assigned a step and has finished it, mark work as done and update worker
                if worker.time_to_complete == 0 {
                    if let Some(step) = &worker.current_step {
                        steps_done.push(step.to_owned());
                        worker.current_step = None;
                    }
                }
            }

            // Assign new work
            for worker in labor_pool.iter_mut() {
                if worker.current_step.is_none() {
                    for todo in &steps_todo {
                        //check if prerequisites met
                        let prereqs = self.step_to_prev.get(todo).expect("failed to get prereqs");
                        let mut can_do = true;

                        for pre in prereqs {
                            if !steps_done.contains(pre) {
                                can_do = false;
                                break;
                            }
                        }

                        if can_do {
                            //assign
                            worker.current_step = Some(todo.to_owned());
                            worker.time_to_complete = *time_per_step
                                .get(todo)
                                .expect("failed to get completion time");

                            //update todo list
                            steps_todo.remove(
                                steps_todo
                                    .iter()
                                    .position(|step| step == todo)
                                    .expect("failed to find position of step in todo"),
                            );

                            //move to next worker
                            break;
                        }
                    }
                }
            }

            // Move time forward
            time_taken += 1;

            for worker in labor_pool.iter_mut() {
                if worker.current_step.is_some() {
                    worker.time_to_complete -= 1;
                }
            }
        }

        // Finish work in progress
        time_taken += labor_pool
            .iter()
            .map(|w| w.time_to_complete)
            .sum::<usize>();

        time_taken
    }
}

#[derive(Debug, Clone)]
struct Worker {
    current_step: Option<String>,
    time_to_complete: usize,
}

impl Worker {
    fn new() -> Self {
        Worker {
            current_step: None,
            time_to_complete: 0,
        }
    }
}
