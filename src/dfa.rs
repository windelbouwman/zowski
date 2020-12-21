use crate::expression::CharSet;
use crate::vector::ExpressionVector;
use std::collections::HashMap;

pub struct Dfa {
    pub token_types: Vec<String>,
    x: Vec<usize>,
    pub transitions: Vec<(usize, Vec<(CharSet, usize)>)>,
    pub accepting: HashMap<usize, Vec<String>>,
    pub error_state: usize,
}

/// Compile the given expression vector into a state machine.
///
/// The state machine contains:
/// - state transitions
/// - a list of accepting state
/// - an error state
pub fn compile(start_state: ExpressionVector) -> Dfa {
    println!("Compiling expression vector: {:?}", start_state);

    let mut transitions: Vec<(usize, Vec<(CharSet, usize)>)> = vec![];
    let mut states: HashMap<ExpressionVector, usize> = HashMap::new();
    let mut accepting: HashMap<usize, Vec<String>> = HashMap::new();
    let mut error_state = None;
    states.insert(start_state.clone(), 0);

    let token_types: Vec<String> = start_state.names();

    // vec![v.clone()];
    // println!("States: {:?}", states);
    let mut stack = vec![(0, start_state)];
    while !stack.is_empty() {
        let (state_num, state_vector) = stack.pop().unwrap();
        println!("State: {}", state_vector);

        let matches = state_vector.is_nullable();
        if !matches.is_empty() {
            accepting.insert(state_num, matches);
        }

        if state_vector.is_null() {
            error_state = Some(state_num);
        }

        let mut state_transitions = vec![];

        for char_class in state_vector.character_classes() {
            // println!("Char class: {}", char_class);
            let c = char_class.first();
            // println!("Char: |{}|", c);

            // Determine new state:
            let new_state_vector = state_vector.derivative(c);
            if !states.contains_key(&new_state_vector) {
                let new_state_num = states.len();
                states.insert(new_state_vector.clone(), new_state_num);
                stack.push((new_state_num, new_state_vector.clone()));
            };
            let new_state_num = states[&new_state_vector];

            // Add state transition:
            state_transitions.push((char_class, new_state_num));
        }
        transitions.push((state_num, state_transitions));
    }

    println!("Done & done. States: {:?}", states);

    Dfa {
        token_types,
        x: vec![],
        transitions,
        accepting,
        error_state: error_state.unwrap(),
    }
}
