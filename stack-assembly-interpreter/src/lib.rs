mod logic;
mod models;

use std::{error, fs};

use logic::tokenize;

pub fn run(file_paths: &[String]) -> Result<(), Box<dyn error::Error>> {
    let contents: Result<Vec<_>, _> = file_paths.iter().map(fs::read_to_string).collect();

    println!("Tokenized: {:?}", tokenize::tokenize(contents?.concat().as_str()));

    Ok(())
}

fn _assembly(_text: &str) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "not implemented"]
    #[test]
    fn translates_assembly_into_opcodes() {
        let text = "10 +65 -40 :_  ; _ == 259
_Loop :a1 HALT _Read_number_ _- _ a1 ; a1 == 260
123 ; ;; i'm comment
1234 PROGRAM_SIZE
:_Loop :_Read_number_ :_- ; _Loop == _Read_number_ == _- == 268";

        let got = _assembly(text);

        assert_eq!(got, vec![10, 65, -40, 268, -37, 268, 268, 259, 260, 123, 1234, 268]);
    }

    #[ignore = "not implemented"]
    #[test]
    fn translates_hello_world() {
        let text = "72 OUT 101 OUT 108 OUT 108 OUT 111 OUT 33 OUT 0 HALT";

        let got = _assembly(text);

        assert_eq!(got, vec![72, -44, 101, -44, 108, -44, 108, -44, 111, -44, 33, -44, 0, -37]);
    }
}
