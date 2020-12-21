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

    let specs = zowski::read_spec(filename)?;
    let ev = spec_to_expression_vector(specs);
    let dfa = zowski::compile(ev);
    zowski::write_c_code(&dfa, basename)?;
    Ok(())
}

fn spec_to_expression_vector(specs: Vec<(String, String)>) -> zowski::ExpressionVector {
    let mut ev = vec![];
    for (name, re) in specs {
        let expr = zowski::Regex::from(re.as_str());
        ev.push((name, expr));
    }
    zowski::ExpressionVector::new(ev)
}
