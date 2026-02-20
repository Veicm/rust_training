use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("data/lorem.txt");
    let output: HashMap<char, i32> = count_letters(path);
    let mut vec: Vec<(&char, &i32)> = output.iter().collect();

    vec.sort_by(|a, b| b.1.cmp(a.1));

    for (key, value) in vec {
        println!("{:?}: {}", key, value);
    }
}

fn count_letters(file_path: &Path) -> HashMap<char, i32> {
    let lorem: String = fs::read_to_string(file_path).expect("Should have worked");
    let chars: Vec<char> = lorem.chars().collect();

    let mut result: HashMap<char, i32> = HashMap::new();

    for char in chars {
        result.entry(char).and_modify(|v| *v += 1).or_insert(1);
    }
    return result;
}
