#[test]
fn crate_documentation_path_matches_filename_case() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = std::fs::read_to_string(root.join("src/lib.rs")).unwrap();
    let path = source
        .split("include_str!(\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let path = root.join("src").join(path);
    let name = path.file_name().unwrap();
    assert!(
        std::fs::read_dir(path.parent().unwrap())
            .unwrap()
            .any(|entry| entry.unwrap().file_name() == name)
    );
}
