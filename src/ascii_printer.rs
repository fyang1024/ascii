pub fn print_a_to_capital_z() {
    let mut s = String::new();
    for x in 'Z'..='a' {
        s.push(x);
    }
    println!("{}", s.chars().rev().collect::<String>());
}

pub mod simple_printer {
    pub fn print_capital_a_to_z() {
        let mut s = String::new();
        for x in 'A'..='z' {
            s.push(x);
        }
        println!("{s}");
    }
}
