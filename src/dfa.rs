use crate::expression::CharSet;
use crate::vector::ExpressionVector;
use std::collections::HashMap;

pub type Dfa = (Vec<usize>, Vec<(usize, CharSet, usize)>, Vec<usize>);

/// Compile the given expression vector into a state machine
pub fn compile(start_state: ExpressionVector) -> Dfa {
    let mut transitions: Vec<(usize, CharSet, usize)> = vec![];
    let mut states: HashMap<ExpressionVector, usize> = HashMap::new();
    let mut accepting = vec![];
    states.insert(start_state.clone(), 0);
    // vec![v.clone()];
    // println!("States: {:?}", states);
    let mut stack = vec![(0, start_state)];
    while !stack.is_empty() {
        let (state_num, state_vector) = stack.pop().unwrap();
        println!("State: {}", state_vector);

        if state_vector.is_nullable() {
            accepting.push(state_num);
        }

        for char_class in state_vector.character_classes() {
            println!("Char class: {}", char_class);
            let c = char_class.first();
            println!("Char: |{}|", c);

            // Determine new state:
            let new_state_vector = state_vector.derivative(c);
            if !states.contains_key(&new_state_vector) {
                let new_state_num = states.len();
                states.insert(new_state_vector.clone(), new_state_num);
                stack.push((new_state_num, new_state_vector.clone()));
            };
            let new_state_num = states[&new_state_vector];

            // Add state transition:
            transitions.push((state_num, char_class, new_state_num));
        }
    }

    println!("Done & done. States: {:?}", states);

    (vec![], transitions, accepting)
}
