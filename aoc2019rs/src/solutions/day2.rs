use anyhow::{bail, Result};

use crate::utils::input;
use crate::intcode::IntcodeMachine;

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(2, b',')?;
    
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run_test(&program, noun, verb);
            if result == 19690720 {
                return Ok(format!("{}", 100 * noun + verb));
            }
            
        }
    }

    bail!("Couldn't find inputs with output 19690720")
}

fn run_test(program: &[i64], noun: i64, verb: i64) -> i64 {
    let mut machine = IntcodeMachine::new_console_machine(&program);
    machine.write_memory(1, noun);
    machine.write_memory(2, verb);
    machine.run();

    machine.read_memory_position(0)
}

// Part 1: 7594646
// Part 2: 3376

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1() {
        let input = day2_input();
        assert_eq!(run_day2_test(&input, 12, 2), 7594646);
    }

    #[test]
    fn day2_part2() {
        let input = day2_input();
        for noun in 0..=99 {
            for verb in 0..=99 {
                let result = run_day2_test(&input, noun, verb);
                if result == 19690720 {
                    return assert_eq!(100 * noun + verb, 3376);
                }
                
            }
        }

        assert!(false)
    }

    fn day2_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(2, b',').unwrap()
    }

    fn run_day2_test(program: &[i64], noun: i64, verb: i64) -> i64 {
        let mut machine = IntcodeMachine::new_console_machine(program);
        machine.write_memory(1, noun);
        machine.write_memory(2, verb);
        machine.run();
    
        machine.read_memory_position(0)
    }
}
