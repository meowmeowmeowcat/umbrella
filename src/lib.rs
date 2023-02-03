use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::fs;
use std::path::Path;

fn read_file(filepath: &Path) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Cannot read the file");
    contents
}

fn parse_file(contents: String) -> Vec<String> {
    let allowed_options: Vec<&str> = vec![
        "-i",
        "--index-url",
        "--extra-index-url",
        "--no-index",
        // "-c",
        // "--constraint",
        "-r",
        "--requirement",
        "-e",
        "--editable",
        "-f",
        "--find-links",
        "--no-binary",
        "--only-binary",
        "--prefer-binary",
        "--require-hashes",
        "--pre",
        "--trusted-host",
        "--use-feature"
    ];

    let mut packages: Vec<String> = Vec::new();
    for line in contents.lines() {
        if line.trim().starts_with("#") {
            // It's a comment
            continue;
        }
        else if line.is_empty() {
            continue;
        }
        else if line.trim().starts_with("-") {
            let items = line.split(" ");
            let option = items.clone().into_iter().nth(0);
            if !allowed_options.contains(&option.unwrap()) {
                println!("{} is not a valid option, skipping", option.unwrap());
                continue;
            }
            if option.unwrap() == "-r" || option.unwrap() == "--requirement" {
                let file = items.clone().into_iter().nth(1);
                parse_requirements(file, &mut packages);
                continue;
            }
        }
        packages.push(line.to_string());
    }
    packages
}

fn parse_requirements(file: Option<&str>, packages: &mut Vec<String>) {
    let file_contents = read_file(Path::new(file.unwrap()));
    let mut file_packages = parse_file(file_contents);
    packages.append(&mut file_packages);
}

#[pyfunction]
fn parse(filepath: String) -> Vec<String> {
    return parse_file(read_file(Path::new(&filepath)));
}

/// A Python module implemented in Rust.
#[pymodule]
fn umbrella(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;
    Ok(())
}
