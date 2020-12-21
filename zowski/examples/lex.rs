use zowski::Regex;

fn main() {
    // Example of API usage for lexer generator
    let token_spec = vec![
        ("ID", "[A-Za-z][A-Za-z]*"),
        ("NUMBER", "[0-9][0-9]*"),
        ("SPACE", "[ ]+"),
        // TODO: ("COMMENT", "/\\*!(.*\\*/.*)\\*/"),
    ];
    let mut ev = vec![];
    for (name, re) in token_spec {
        let expr = Regex::from(re);
        ev.push((name.to_owned(), expr));
    }
    let ev = zowski::ExpressionVector::new(ev);
    let dfa = zowski::compile(ev);
    let test_text = "67432 2323  bla   mo";
    println!("Scanning: {}", test_text);
    // let basename = "scanner";
    // zowski::write_c_code(&dfa, basename).unwrap();
    let tokens = zowski::scan(dfa, test_text);
    println!("Tokens: {:?}", tokens);
    // write_dot(dfa).unwrap();
}
