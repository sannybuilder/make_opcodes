fn main() {
    if let Some(input_file) = std::env::args().nth(1) {
        match std::fs::read(input_file) {
            Ok(buf) => {
                let file_content = String::from_utf8_lossy(&buf);
                let mut uniq_opcodes = vec![];
                let mut output = vec![];

                for line in file_content.lines() {
                    let line = line.trim();
                    if let Some(':') = line.chars().nth(4) {
                        let opcode = line.get(0..4).unwrap();
                        if !uniq_opcodes.contains(&opcode) {
                            uniq_opcodes.push(opcode);
                            output.push(line);
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
