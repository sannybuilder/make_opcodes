extern crate sanny_builder_core as sb;
use sb::dictionary::dictionary_num_by_str::DictNumByStr;
use sb::dictionary::ffi::*;
use sb::namespaces::namespaces::Namespaces;
use std::ffi::CString;

fn main() {
    if let Some(input_file) = std::env::args().nth(1) {
        match std::fs::read(input_file) {
            Ok(buf) => {
                let file_content = String::from_utf8_lossy(&buf);
                let mut uniq_opcodes = vec![];
                let mut output = vec![];
                let mut keywords = DictNumByStr::new(
                    Duplicates::Replace,
                    CaseFormat::NoFormat,
                    String::from(";"),
                    String::from("=,"),
                    true,
                    true,
                );
                let mut classes = Namespaces::new();

                // todo: read from command line
                let has_keywords = keywords.load_file("keywords.txt").is_some();
                let has_classes = classes.load_classes("classes.db").is_some();

                for line in file_content.lines() {
                    let line = line.trim();

                    // opcode
                    if let Some(':') = line.chars().nth(4) {
                        if let Some(opcode) = line.get(0..4) {
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
                        .split(|c: char| {
                            c.is_ascii_whitespace() || c == '(' || c == ')' || c == ','
                        })
                        .next()
                    {
                        // keyword
                        if has_keywords {
                            if let Some(key) = CString::new(first_word).ok() {
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

                for line in output {
                    println!("{}", line);
                }
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(2);
            }
        }
    } else {
        println!("Usage: make_opcodes.exe <input file>");
    }
}
