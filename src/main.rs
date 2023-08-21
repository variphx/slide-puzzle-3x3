use std::collections::{HashMap, HashSet, VecDeque};

use slide_puzzle_3x3::*;

fn main() {
    let mut frontier = VecDeque::new();
    let mut explored_set = HashSet::new();
    let mut parents_map = HashMap::new();

    let initial_puzzle = {
        let mut initial_puzzle = Vec::with_capacity(PUZZLE_SIZE);
        let mut lines = std::io::stdin().lines();
        for _ in 0..PUZZLE_SIZE {
            let row: Vec<u8> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u8>().unwrap() - 1)
                .collect();
            initial_puzzle.push(row);
        }

        let empty_tile = {
            let empty_tile_buffer: Vec<u8> = lines
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            assert_eq!(empty_tile_buffer.len(), 2);

            Coordination::new(empty_tile_buffer[0], empty_tile_buffer[1])
        };

        Puzzle::new(initial_puzzle, empty_tile)
    };

    if initial_puzzle.is_correct() {
        return println!("Inputted puzzle is already correct");
    }

    frontier.push_back(initial_puzzle.clone());

    let mut answer = None;

    while !frontier.is_empty() {
        let puzzle = frontier.pop_front().unwrap();
        if puzzle.is_correct() {
            answer = Some(puzzle);
            break;
        }
        let _ = explored_set.insert(puzzle.clone());

        for action in puzzle.possible_actions() {
            let new_state = puzzle.transitional_state_with_action(action);
            if !explored_set.contains(&new_state) {
                frontier.push_back(new_state.clone());
                let _ = parents_map.insert(new_state, action);
            }
        }
    }

    if let Some(answer) = answer {
        let mut solution_holder = Vec::new();
        let mut puzzle = answer.clone();

        while let Some(&swipe) = parents_map.get(&puzzle) {
            solution_holder.push(swipe);
            puzzle = puzzle.transitional_state_with_action(swipe.opposite());
        }

        for swipe in solution_holder.into_iter().rev() {
            println!("Swipe {:?}", swipe);
            puzzle = puzzle.transitional_state_with_action(swipe);
            println!("{}", puzzle);
        }
    } else {
        println!("No solution!");
    }
}
