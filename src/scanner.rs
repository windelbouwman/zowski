use crate::dfa::Dfa;
use crate::expression::CharSet;

#[derive(Debug)]
pub struct Token {
    typ: String,
    text: String,
}

/// Scan the given text for tokens
pub fn scan(prog: Dfa, text: &str) -> Result<Vec<Token>, String> {
    let (transitions, accepting, error_state) = (prog.transitions, prog.accepting, prog.error_state);
    let mut state: usize = 0;
    let mut tokens = vec![];

    let mut tok_begin = 0;
    let mut tok_end: Option<(usize, String)> = None;
    let mut index = 0;
    let chars: Vec<char> = text.chars().collect();

    while let Some(c) = chars.get(index) {
        // Process a single character at a time.

        state = next_state(&transitions, state, *c);

        index += 1;

        if accepting.contains_key(&state) {
            // Okay, this can be some token.
            let typ: String = accepting.get(&state).unwrap().first().unwrap().to_owned();
            tok_end = Some((index, typ));
        }

        if state == error_state {
            if let Some((tok_end, typ)) = tok_end {
                let text: String = chars[tok_begin..tok_end].into_iter().collect();
                let token = Token { typ, text };

                tokens.push(token);
                tok_begin = tok_end;
                index = tok_end;
            } else {
                return Err("Oh noes!".to_owned());
            }

            state = 0;
            tok_end = None;
        }
    }

    // Add last token:
    if let Some((tok_end, typ)) = tok_end {
        let text: String = chars[tok_begin..tok_end].into_iter().collect();
        let token = Token { typ, text };
        tokens.push(token);
    }

    Ok(tokens)
}

fn next_state(transitions: &[(usize, CharSet, usize)], state: usize, c: char) -> usize {
    for (from_state, cc, to_state) in transitions {
        if state == *from_state {
            if cc.contains(c) {
                return *to_state;
            }
        }
    }

    unreachable!("Serious trouble here. This must not happen.");
}
