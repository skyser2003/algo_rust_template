// Copy this file to main.rs and edit

use std::io::stdin;

#[derive(Debug, Default)]
struct InputLine {
    inputs: Vec<String>,
}

impl InputLine {
    fn read_line(&mut self) {
        let mut s = String::new();

        stdin().read_line(&mut s).unwrap();
        let inputs = s.trim().split(" ");

        let mut vec = Vec::<String>::new();

        for s in inputs {
            vec.push(s.to_string());
        }

        self.inputs = vec;
    }

    fn get_int(&self, index: usize) -> i32 {
        self.inputs[index].parse().unwrap()
    }

    fn get_i64(&self, index: usize) -> i64 {
        self.inputs[index].parse().unwrap()
    }

    fn get_i128(&self, index: usize) -> i128 {
        self.inputs[index].parse().unwrap()
    }

    fn get_str(&self, index: usize) -> &str {
        &self.inputs[index]
    }
}

fn main() {
    let mut line: InputLine = Default::default();
    line.read_line();

    let n = line.get_int(0);

    println!("{}", n);
}
