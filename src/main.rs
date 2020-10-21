mod dfa;
mod expression;
mod range;
mod rangeset;
mod vector;

use dfa::{compile, Dfa};
use expression::{CharSet, Regex};
use std::collections::HashMap;
use vector::ExpressionVector;

/// Scan the given text for tokens
// fn scan(prog: usize, text: &str) {
//     let state = 0;
//     for c in text.chars() {
//         // ugh...
//         state = next_state(c);
//     }
// }
fn first_usages() {
    let r = Regex::symbol('G') + Regex::symbol('K');
    println!("r = {}, r_G = {}", r, r.derivative('G'));

    let r = Regex::symbol_range('A', 'C').kleene() | (Regex::symbol('G') + Regex::symbol('K'));
    // + (symbol('X') & symbol('Z'));
    println!("Regex: {}", r);

    println!("derivative_A (r) = {}", r.derivative('A'));
    println!("derivative_G (r) = {}", r.derivative('G'));
    println!("derivative_K (r) = {}", r.derivative('K'));
    // scan(r, "AAA");

    let ev = ExpressionVector::new(vec![r]);
    println!("Compiling expression vector: {:?}", ev);
    let _dfa = compile(ev);
}

fn first_prog() {
    let digit = Regex::symbol_range('0', '9');
    let number = digit.clone() + digit.kleene();
    let ev = ExpressionVector::new(vec![number]);
    println!("Compiling expression vector: {:?}", ev);
    let dfa = compile(ev);

    write_dot(dfa).unwrap();
}

/// Export as dot file
fn write_dot(dfa: Dfa) -> std::io::Result<()> {
    let filename = "machine.dot";
    let mut f = std::fs::File::create(filename)?;
    let (_, transitions, accepting) = dfa;
    use std::io::Write;
    writeln!(f, "digraph state_machine {{")?;
    for (from_state, char_set, to_state) in transitions {
        let label = format!("{}", char_set);
        writeln!(f, "  {} -> {} [label=\"{}\"];", from_state, to_state, label)?;
    }

    for s in accepting {
        writeln!(f, "  {}[peripheries=2];", s)?;
    }
    writeln!(f, "}}")?;
    Ok(())
}

fn simple_example() {
    /*
    Example 1:

    identifier = [_A-Za-z]([_A-Za-z0-9])*
    number = [0-9]+
    operator = [-+*=/]
    other = .

    */
    let digit = Regex::symbol_range('0', '9');
    let letter = Regex::symbol_range('A', 'Z') | Regex::symbol_range('a', 'z') | Regex::symbol('_');
    let identifier = letter.clone() + (letter | digit.clone()).kleene();
    let number = digit.clone() + digit.clone().kleene();
    let operator = Regex::symbol('-')
        | Regex::symbol('+')
        | Regex::symbol('*')
        | Regex::symbol('/')
        | Regex::symbol('=');
    let ev = ExpressionVector::new(vec![identifier, number, operator]);
    println!("Compiling expression vector: {}", ev);

    let dfa = compile(ev);
    write_dot(dfa).unwrap();
}

fn main() {
    // first_usages();
    // first_prog();
    simple_example();
}
