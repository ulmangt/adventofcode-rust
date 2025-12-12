use good_lp::{Expression, ProblemVariables, Solution, SolverModel, solvers::microlp, variable};

use crate::advent2025::puzzle10::part1::{InputDataError, Machine, read_input_data};

use std::error::Error;

// can calculate the maximum number of times each button can be pressed (the min of the joltage requirements for each slot it affects)
// generally want to press buttons that affect the maximum number of slots (until this backs us into a corner)
// can calculate the number of buttons that can increment a given slot (the number of buttons that won't overflow another slot by pressing it--this will be all the buttons until at least one slot is at its limit)
// we can't make a move which reduces the number of buttons that can increment a slot to 0
// as long as we don't do that, we should push the button that affects the maximum number of slots
//
// click the button which reduces the largest number of the highest slots? does that have the possibility of backing itself into corner?

pub fn solve() -> Result<u32, Box<dyn Error>> {


    let machines: Vec<Machine> = read_input_data()?
        .lines()
        .map(Machine::parse)
        .collect::<Result<Vec<Machine>,InputDataError>>()?;

    Ok(machines.iter().enumerate().map(|(index,machine)|{
        println!("line {}",index+1);
        get_fewest_presses_joltage(machine)
    }).map(|v|v.unwrap()).sum())
}


fn get_fewest_presses_joltage(machine: &Machine) -> Result<u32, Box<dyn Error>> {

    // 1. Create a variable store
    let mut vars = ProblemVariables::new();

    let mut variables = vec![];

    // add a variable for the number of times each button_wiring is pressed
    for _ in machine.button_wiring.iter() {
        variables.push( vars.add(variable().min(0).integer() ) );
    }

    // construct objective function which sums the button clicks
    let mut objective_expr: Expression = 0.into();
    for var in variables.iter() {
        objective_expr = objective_expr + *var;
    }

    println!("objective {:?}",objective_expr);

    let mut solution = vars
        .minimise(&objective_expr)
        .using(microlp::microlp);

    for (requirement_index, requirement) in machine.joltage_requirements.iter().enumerate() {

        let mut constraint: Expression = 0.into();
        for (index,wiring) in machine.button_wiring.iter().enumerate() {
            if wiring.contains(&(requirement_index as u32)) {
                constraint = constraint + variables[index];
            }
        }

        println!("constaint {:?} = {}",constraint, *requirement);

        solution = solution.with(constraint.eq(*requirement));
    }

    let solution = solution.solve()?;

    
    for var in variables.iter() {
        println!("variable: {}",solution.eval(var))
    }
    

    let result = solution.eval(&objective_expr) as u32;

    println!("result: {}",result);

    let rounded_result = variables
        .iter()
        .map(|v| solution.eval(v).round().max(0.0))
        .sum::<f64>() as u32;

    Ok(rounded_result)
}