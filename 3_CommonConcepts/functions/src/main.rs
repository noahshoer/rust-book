fn five() -> i32 {
    5
}

fn main() {
    print_labeled_measurement(five(), 'h');

    // Not having a semicolon makes x+1 an expr
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}