fn main() {
    print_values();
    bobby();
    let y = add(5, 10);

    let _x: i32 = {
        let y: i32 = 1;
        let z: i32 = 2;
        y * z * 3
    };
    println!("Expression: {:?}. Add: {}", _x, y);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn human(name: &str, age: u32, height: f32) {
    println!("Name: {} - Age: {} - Height: {}.", name, age, height);
}

fn bobby() {
    human("Bobby", 21, 190.0);
}

fn print_values() {
    print_literals();
    print_lists();
    print_strings();
}

fn print_literals() {
    let x: i32 = -5;
    let y: u64 = 5;
    let pi: f64 = 3.14;
    let is_true: bool = true;

    println!("Hello, world! {}.{}.{}.{}!", x, y, pi, is_true);
}

fn print_lists() {
    let numbers: [i32; 3] = [1, 2, 3];
    let fruits: [&str; 3] = ["apple", "apple", "pear"];
    let tuple: (i32, i32, String, bool) = (1, 2, "string".to_string(), true);
    let slice: &[i32] = &[1,3,4,5];

    println!("Hello, world! {:?} - {:?} - {:?} - {:?}", numbers, fruits, tuple, slice);
}

fn print_strings() {
    let str: String = String::from("string!!!");
    let str_sliced: &str = &str[0..5];

    println!("Hello, world! {} - {}", str, str_sliced);
}
