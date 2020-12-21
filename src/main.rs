mod dfa;
mod dot;
mod export_to_c;
mod expression;
mod parse;
mod range;
mod rangeset;
mod scanner;
mod spec;
mod vector;

use dfa::compile;
use dot::write_dot;
use export_to_c::write_c_code;
use expression::Regex;
use scanner::scan;
use spec::read_spec;
use vector::ExpressionVector;

fn main() {
    convert().unwrap();
}

/// Read token spec from file, and generate c code.
fn convert() -> std::io::Result<()> {
    let matches = clap::App::new("regex scanner generator")
        .version("0")
        .author("Windel Bouwman")
        .arg(clap::Arg::with_name("filename").required(true))
        .arg(clap::Arg::with_name("basename").required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let basename = matches.value_of("basename").unwrap();

    let specs = read_spec(filename)?;
    let ev = spec_to_expression_vector(specs);
    let dfa = compile(ev);
    write_c_code(&dfa, basename)?;
    Ok(())
}

fn spec_to_expression_vector(specs: Vec<(String, String)>) -> ExpressionVector {
    let mut ev = vec![];
    for (name, re) in specs {
        let expr = Regex::from(re.as_str());
        ev.push((name, expr));
    }
    ExpressionVector::new(ev)
}
