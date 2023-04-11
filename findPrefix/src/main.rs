use std::io;

fn find_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();

    for (i, c) in strings[0].chars().enumerate() {
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    println!("Enter a set of strings (separated by commas):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let strings: Vec<String> = input
        .split(",")
        .map(|s| s.trim().to_string())
        .collect();

    let prefix = find_prefix(&strings);

    if prefix.is_empty() {
        println!("There is no common prefix.");
    } else {
        println!("The longest common prefix is: {}", prefix);
    }
}
