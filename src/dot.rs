use crate::dfa::Dfa;

/// Export as dot file
pub fn write_dot(dfa: Dfa) -> std::io::Result<()> {
    let filename = "machine.dot";
    let mut f = std::fs::File::create(filename)?;
    let (all_transitions, accepting) = (dfa.transitions, dfa.accepting);
    use std::io::Write;
    writeln!(f, "digraph state_machine {{")?;
    for (from_state, transitions) in all_transitions {
        for (char_set, to_state) in transitions {
            let label = format!("{}", char_set);
            writeln!(f, "  {} -> {} [label=\"{}\"];", from_state, to_state, label)?;
        }
    }

    for s in accepting.keys() {
        writeln!(f, "  {}[peripheries=2];", s)?;
    }
    writeln!(f, "}}")?;
    Ok(())
}
