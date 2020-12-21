use zowski::Regex;


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

    let ev = ExpressionVector::new(vec![("TEST".to_owned(), r)]);
    let _dfa = compile(ev);
}


fn first_prog() {
    let digit = Regex::symbol_range('0', '9');
    let number = digit.one_or_more();
    let ev = ExpressionVector::new(vec![("NUM".to_owned(), number)]);
    let dfa = compile(ev);
    write_dot(dfa).unwrap();
}

fn simple_example() {
    /*
    Example 1:

    identifier = [_A-Za-z]([_A-Za-z0-9])*
    number = [0-9]+
    operator = [-+*=/]
    other = .

    */
    let digit = Regex::from("[0-9]");
    let letter = Regex::from("[A-Za-z_]");
    let identifier = letter.clone() + (letter | digit.clone()).kleene();
    let number = digit.one_or_more();
    let operator = Regex::symbol('-')
        | Regex::symbol('+')
        | Regex::symbol('*')
        | Regex::symbol('/')
        | Regex::symbol('=');
    let ev = ExpressionVector::new(vec![
        ("ID".to_owned(), identifier),
        ("NUM".to_owned(), number),
        ("OP".to_owned(), operator),
    ]);
    let dfa = compile(ev);
    write_dot(dfa).unwrap();
}

fn make_lex() {
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
    let ev = ExpressionVector::new(ev);
    let dfa = compile(ev);
    let test_text = "67432 2323  bla   mo";
    println!("Scanning: {}", test_text);
    let basename = "scanner";
    write_c_code(&dfa, basename).unwrap();
    let tokens = scan(dfa, test_text);
    println!("Tokens: {:?}", tokens);
    // write_dot(dfa).unwrap();
}
