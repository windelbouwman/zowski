/// Read token spec from file.
///
/// Simple file format, consisting of lines of:
/// ID: PATTERN
pub fn read_spec(filename: &str) -> std::io::Result<Vec<(String, String)>> {
    let f = std::fs::File::open(filename)?;

    let mut specs = vec![];

    let reader = std::io::BufReader::new(f);
    use std::io::BufRead;
    for line in reader.lines() {
        let line = line?;
        // println!("Line: {}", line);
        if line.contains(':') {
            let mut parts = line.split(':');
            let id: String = parts.next().unwrap().trim().to_owned();
            let pattern: String = parts.next().unwrap().trim().to_owned();
            // println!("ID = '{}' PATTERN = '{}'", id, pattern);
            specs.push((id, pattern));
        }
    }

    Ok(specs)
}
