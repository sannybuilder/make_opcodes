extern crate argh;
extern crate sanny_builder_core as sb;
use argh::FromArgs;
use sb::dictionary::dictionary_num_by_str::DictNumByStr;
use sb::dictionary::ffi::*;
use sb::namespaces::namespaces::Namespaces;
use std::ffi::CString;

#[derive(FromArgs)]
///
struct Options {
    /// input text file to read
    #[argh(positional)]
    input: String,

    /// path to output file
    #[argh(option)]
    output: Option<String>,

    /// path to classes.db
    #[argh(option)]
    classes: Option<String>,

    /// path to keywords.txt
    #[argh(option)]
    keywords: Option<String>,
}
fn main() {
    let options: Options = argh::from_env();

    match std::fs::read(options.input) {
        Ok(buf) => {
            let file_content = String::from_utf8_lossy(&buf);
            let mut uniq_opcodes: Vec<String> = vec![];
            let mut output = vec![];
            let mut keywords = DictNumByStr::new(
                Duplicates::Replace,
                CaseFormat::LowerCase,
                String::from(";"),
                String::from("=,"),
                true,
                true,
            );
            let mut classes = Namespaces::new();

            let has_keywords = keywords
                .load_file(options.keywords.unwrap_or(String::new()).as_str())
                .is_some();
            let has_classes = classes
                .load_classes(options.classes.unwrap_or(String::new()).as_str())
                .is_some();

            for line in file_content.lines() {
                let line = line.trim();

                // opcode
                if let Some(':') = line.chars().nth(4) {
                    if let Some(opcode) = line.get(0..4) {
                        let opcode = String::from(opcode);
                        if !uniq_opcodes.contains(&opcode) {
                            uniq_opcodes.push(opcode);
                            output.push(line);
                            continue;
                        }
                    }
                }

                if !has_classes && !has_keywords {
                    continue;
                }

                if let Some(first_word) = line
                    .split(|c: char| c.is_ascii_whitespace() || c == '(' || c == ')' || c == ',')
                    .map(|s| s.to_ascii_lowercase())
                    .next()
                {
                    // keyword
                    if has_keywords {
                        if let Some(key) = CString::new(first_word.clone()).ok() {
                            if keywords.map.contains_key(&key) {
                                if !uniq_opcodes.contains(&first_word) {
                                    uniq_opcodes.push(first_word);
                                    output.push(line);
                                    continue;
                                }
                            }
                        }
                    }

                    // classes
                    if has_classes {
                        if first_word.contains('.') {
                            let mut class = first_word.split(|c| c == '.');

                            if let Some(class_name) = class.next() {
                                if let Some(member_name) = class.next() {
                                    // bug: see https://github.com/sannybuilder/dev/issues/92
                                    if classes
                                        .get_opcode_index_by_name(class_name, member_name)
                                        .is_some()
                                    {
                                        if !uniq_opcodes.contains(&first_word) {
                                            uniq_opcodes.push(first_word);
                                            output.push(line);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            output.sort();

            if let Some(output_file) = options.output {
                std::fs::write(output_file, output.join("\n")).unwrap();
            } else {
                for line in output {
                    println!("{}", line);
                }
            }
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(2);
        }
    }
}
