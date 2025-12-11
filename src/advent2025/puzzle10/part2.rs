use std::collections::{HashMap, VecDeque};

use good_lp::{Solution, SolverModel, constraint, solvers::microlp, variables};

use crate::advent2025::puzzle10::part1::{Machine};

use std::error::Error;

// can calculate the maximum number of times each button can be pressed (the min of the joltage requirements for each slot it affects)
// generally want to press buttons that affect the maximum number of slots (until this backs us into a corner)
// can calculate the number of buttons that can increment a given slot (the number of buttons that won't overflow another slot by pressing it--this will be all the buttons until at least one slot is at its limit)
// we can't make a move which reduces the number of buttons that can increment a slot to 0
// as long as we don't do that, we should push the button that affects the maximum number of slots
//
// click the button which reduces the largest number of the highest slots? does that have the possibility of backing itself into corner?

pub fn solve() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
            x >= 0;
            y >= 0;
    }

    let objective = 5 * x + 3 * y;

    let solution = vars
        .maximise(&objective)
        .using(microlp::microlp)
        .with(constraint!(x + 2*y <= 14))
        .with(constraint!(3*x - y >= 0))
        .with(constraint!(x - y <= 2))
        .solve()
        .unwrap();

    println!("x = {}", solution.value(x));
    println!("y = {}", solution.value(y));
    println!("objective = {}", solution.eval(&objective));
    Ok(())
}

/*
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
*/

impl Machine {
    fn get_fewest_presses_joltage(&self) -> Option<u32> {
        
        // map storing the minimum number of clicks found so far to get to any button state
        let mut min_press_map: HashMap<Vec<u32>,u32> = HashMap::new();
        let mut queue: VecDeque<(&Vec<u32>,Vec<u32>,u32)> = VecDeque::new();
        self.button_wiring.iter().for_each(|button|queue.push_front((&button,vec![0;self.indicator_lights_goal.len()],0)));

        while !queue.is_empty() {
            let (buttons,joltages,presses) = queue.pop_back().unwrap();
            let joltages = press_buttons( buttons, &joltages );

            if joltages.eq(&self.joltage_requirements) {
                return Some(presses+1);
            }

            let min_presses = min_press_map.get(&joltages);
            
            // if there is an entry in the min_presses map, and it is smaller than ours
            // then we are already exploring a path that got to this state with fewer presses
            if let Some(min_presses) = min_presses {
                if *min_presses <= presses+1 {
                    continue;
                }
                else {
                }
            }

            min_press_map.insert(joltages.clone(),presses+1);

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