use zowski::Regex;

fn main() {
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
    let ev = zowski::ExpressionVector::new(vec![
        ("ID".to_owned(), identifier),
        ("NUM".to_owned(), number),
        ("OP".to_owned(), operator),
    ]);
    let dfa = zowski::compile(ev);
    zowski::write_dot(dfa).unwrap();
}
