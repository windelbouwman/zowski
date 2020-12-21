use zowski::Regex;

fn main() {
    let r = Regex::symbol('G') + Regex::symbol('K');
    println!("r = {}, r_G = {}", r, r.derivative('G'));

    let r = Regex::symbol_range('A', 'C').kleene() | (Regex::symbol('G') + Regex::symbol('K'));
    // + (symbol('X') & symbol('Z'));
    println!("Regex: {}", r);

    println!("derivative_A (r) = {}", r.derivative('A'));
    println!("derivative_G (r) = {}", r.derivative('G'));
    println!("derivative_K (r) = {}", r.derivative('K'));
    // scan(r, "AAA");

    let ev = zowski::ExpressionVector::new(vec![("TEST".to_owned(), r)]);
    let _dfa = zowski::compile(ev);
}
