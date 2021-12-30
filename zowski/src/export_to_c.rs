use crate::dfa::Dfa;
use tera::Tera;

/// Generate C-code for a state machine.
pub fn write_c_code(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    write_c_header(dfa, basename)?;
    write_c_source(dfa, basename)?;

    Ok(())
}

fn create_tera_context(dfa: &Dfa) -> tera::Context {
    let (token_types, all_transitions, accepting_states, error_state) = (
        &dfa.token_types,
        &dfa.transitions,
        &dfa.accepting,
        &dfa.error_state,
    );

    // Create list of state, transition pairs:
    let transitions2: Vec<State> = all_transitions
        .iter()
        .map(|(s, t)| {
            let mut t2: Vec<StateTransition> = vec![];
            for (r, target_state) in t {
                for y in &r.ranges {
                    t2.push(StateTransition {
                        begin: y.begin,
                        end: y.end,
                        next_state: *target_state,
                    });
                }
            }
            State {
                num: *s,
                transitions: t2,
            }
        })
        .collect();

    let mut context = tera::Context::new();
    context.insert("error_state", error_state);
    context.insert("token_types", token_types);
    context.insert("accepting_states", accepting_states);
    context.insert("all_transitions", &transitions2);
    context
}

fn write_c_header(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    let filename = format!("{}.h", basename);
    let mut f = std::fs::File::create(filename)?;
    use std::io::Write;

    let template_text = std::include_str!("templates/c/header.txt");
    let context = create_tera_context(dfa);
    let generated_src = Tera::default().render_str(template_text, &context).unwrap();

    write!(f, "{}", generated_src)?;

    Ok(())
}

#[derive(serde::Serialize)]
struct StateTransition {
    begin: char,
    end: char,
    next_state: usize,
}

#[derive(serde::Serialize)]
struct State {
    num: usize,
    transitions: Vec<StateTransition>,
}

fn write_c_source(dfa: &Dfa, basename: &str) -> std::io::Result<()> {
    let template_text = std::include_str!("templates/c/source.txt");
    let context = create_tera_context(dfa);
    let generated_src = Tera::default().render_str(template_text, &context).unwrap();

    let filename = format!("{}.c", basename);
    let mut f = std::fs::File::create(filename)?;
    use std::io::Write;

    write!(f, "{}", generated_src)?;

    Ok(())
}
