fn parse_labels(file_contents: &str) -> Vec<String> {
    let mut labels = Vec::<String>::new();

    for (index, line) in file_contents.split('\n').enumerate() {
        let line = line.trim();

        if line.starts_with('.') {
            labels.insert(index, line.strip_prefix('.').unwrap().to_string());
        }
    }

    labels
}
