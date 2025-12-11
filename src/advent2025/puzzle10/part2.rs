use std::collections::VecDeque;

use crate::advent2025::puzzle10::part1::{InputDataError, Machine, read_input_data};

pub fn solve() -> Result<u32,InputDataError> {

    let machines: Vec<Machine> = read_input_data()?
        .lines()
        .map(Machine::parse)
        .collect::<Result<Vec<Machine>,InputDataError>>()?;

    Ok(
        machines
            .iter()
            .map(Machine::get_fewest_presses_joltage)
            .map(|v| {
                let min_presses = v.unwrap_or(0);
                println!("{min_presses}");
                min_presses
            })
            .sum()
    )
}

impl Machine {
    fn get_fewest_presses_joltage(&self) -> Option<u32> {
        
        let mut queue: VecDeque<(&Vec<u32>,Vec<u32>,u32)> = VecDeque::new();
        self.button_wiring.iter().for_each(|button|queue.push_front((&button,vec![0;self.indicator_lights_goal.len()],0)));

        while !queue.is_empty() {
            let (buttons,joltages,presses) = queue.pop_back().unwrap();
            let joltages = press_buttons( buttons, &joltages );

            if joltages.eq(&self.joltage_requirements) {
                return Some(presses+1);
            }

            // since counters cannot decrease, only continue down this branch if no counters are greater than the requirement
            if !joltages
                .iter()
                .zip(&self.joltage_requirements)
                .any(|(jolt,requirement)| *jolt > *requirement) {

                self.button_wiring.iter().for_each(|button|queue.push_front((&button,joltages.clone(),presses+1)));
            }
        }

        return None;
    }
}

fn press_buttons( buttons: &Vec<u32>, joltages: &Vec<u32> ) -> Vec<u32> {
    let mut joltages: Vec<u32> = joltages.clone();
    for button in buttons.iter() {
        joltages.get_mut(*button as usize).map(|value|*value += 1);
    }
    joltages
}