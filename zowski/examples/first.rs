use zowski::Regex;

fn main() {
    let digit = Regex::symbol_range('0', '9');
    let number = digit.one_or_more();
    let ev = zowski::ExpressionVector::new(vec![("NUM".to_owned(), number)]);
    let dfa = zowski::compile(ev);
    zowski::write_dot(dfa).unwrap();
}
