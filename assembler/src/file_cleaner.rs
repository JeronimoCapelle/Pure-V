pub fn clean_file(file_contents: String) -> String {
    let mut cleaned_file_contents_pass_1 = String::new();

    for line in file_contents.split('\n') {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with('#') {
            continue;
        }

        if line.starts_with("//") {
            continue;
        }

        let line = match line.split_once('#') {
            Some(a) => a.0,
            None => line,
        };

        let line = match line.split_once("//") {
            Some(a) => a.0,
            None => line,
        };

        let mut line = line.trim();

        if !line.ends_with(':') && line.contains(':') {
            line = match line.split_once(':') {
                None => panic!("impossible"),
                Some(a) => {
                    cleaned_file_contents_pass_1.push_str(a.0.trim());
                    cleaned_file_contents_pass_1.push(':');
                    cleaned_file_contents_pass_1.push('\n');
                    a.1.trim()
                }
            };
        }

        cleaned_file_contents_pass_1.push_str(line);
        cleaned_file_contents_pass_1.push('\n');
    }

    cleaned_file_contents_pass_1 = cleaned_file_contents_pass_1
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    let mut cleaned_file_contents_pass_2 = String::new();

    for line in cleaned_file_contents_pass_1.split('\n') {
        if line.contains(':') {
            cleaned_file_contents_pass_2.push_str(line);
            cleaned_file_contents_pass_2.push('\n');
            continue;
        }

        let (inst, args) = match line.split_once(char::is_whitespace) {
            None => panic!("No spaces in inst"),
            Some(a) => (a.0.trim(), a.1.trim()),
        };

        cleaned_file_contents_pass_2.push_str(inst);
        cleaned_file_contents_pass_2.push(' ');

        //---

        let mut arg_buffer = String::new();

        for arg in args.split(',') {
            arg_buffer.push_str(arg.trim());
            arg_buffer.push(',');
        }

        arg_buffer = arg_buffer.strip_suffix(',').unwrap().to_string();

        //---

        let mut arg_buffer_2 = String::new();

        for arg in arg_buffer.split('(') {
            arg_buffer_2.push_str(arg.trim());
            arg_buffer_2.push('(');
        }

        arg_buffer_2 = arg_buffer_2.strip_suffix('(').unwrap().to_string();

        //---

        let mut arg_buffer_3 = String::new();

        for arg in arg_buffer_2.split(')') {
            arg_buffer_3.push_str(arg.trim());
            arg_buffer_3.push(')');
        }

        arg_buffer_3 = arg_buffer_3.strip_suffix(')').unwrap().to_string();

        cleaned_file_contents_pass_2.push_str(&arg_buffer_3);
        //---

        cleaned_file_contents_pass_2.push('\n');
    }

    cleaned_file_contents_pass_2 = cleaned_file_contents_pass_2
        .strip_suffix('\n')
        .unwrap()
        .to_string();

    cleaned_file_contents_pass_2
}
