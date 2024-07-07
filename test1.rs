fn test1() {
    let test_string = "Test String";
    let test_float = 1.0;
    let test_boolean = true;

    println!("test {}", test_string);
    println!("test {}", test_float);
    println!("test {}", test_boolean);
}

fn variables() {
    let salary: f64 = 10_000.0;

    println!("Salary: {}", salary);

    let mut mutable_value: i32 = 10;
    println!("Mutable value: {}", mutable_value);

    mutable_value = 20;
    println!("Mutable value: {}", mutable_value);
}

fn constants() {
    const CONSTANT: i32 = 10;

    println!("Constant: {}", CONSTANT);
}

fn stringss() {
    let name: &str = "Kurl";

    println!("Name: {}", name);

    let another_name: &'static str = "Kurl 2";

    println!("Name: {}", another_name);
}

fn main() {
    println!("Hello world!");

    test1();
    variables();
    constants();
    stringss();
}
