#[cfg(test)]
mod hello;
#[cfg(test)]
mod hello_fn;

#[allow(dead_code)]
fn read_source(file: &str) -> String {
    std::fs::read_to_string(file).expect("file not found")
}
