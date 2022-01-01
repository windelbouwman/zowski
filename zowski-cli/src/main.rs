fn main() {
    convert();
}

/// Read token spec from file, and generate c code.
fn convert() {
    let matches = clap::App::new("regex scanner generator")
        .version("0")
        .author("Windel Bouwman")
        .arg(clap::Arg::with_name("filename").required(true))
        .arg(clap::Arg::with_name("basename").required(true))
        .get_matches();

    let filename = matches.value_of("filename").unwrap();
    let basename = matches.value_of("basename").unwrap();

    match zowski::read_spec(filename) {
        Ok(specs) => {
            let ev = spec_to_expression_vector(specs);
            let dfa = zowski::compile(ev);
            zowski::write_c_code(&dfa, basename).unwrap();
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn spec_to_expression_vector(specs: Vec<zowski::TokenSpec>) -> zowski::ExpressionVector {
    let mut ev = vec![];
    for spec in specs {
        ev.push((spec.name, spec.pattern));
    }
    zowski::ExpressionVector::new(ev)
}
