fn main() {
    #[cfg(not(debug_assertions))]
    built::write_built_file().unwrap()
}
