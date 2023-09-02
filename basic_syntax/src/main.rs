// Constant values don't have fixed addresses in memory.
const CON: u32 = 100_000;
// Static values have fixed addresses in memory.
static STA: u32 = 50_000;

static mut UNSF: u32 = 20_000;

fn scope() {
    // This variable is only accessible inside this function.
    let scope_var: u32 = 10_000;
    println!(
        "scope_var = {}, size = {};",
        scope_var,
        std::mem::size_of_val(&scope_var)
    );
}

fn more_scope() {
    // The variable is only accessible inside this function.
    // but if I create another scope here
    // I can access the variable from the outer scope.
    let mut a: u32 = 10_000;

    {
        println!(
            "inner shadow a = {}, size = {};",
            a,
            std::mem::size_of_val(&a)
        );
        // I can even modify the variable from the outer scope.
        a = 20_000;
        // but a variable inside this scope will not be accessible outside.
    }
    println!(
        "outer shadow a = {}, size = {};",
        a,
        std::mem::size_of_val(&a)
    );
}

fn shadowing() {
    // Shadowing is a process of re-declaring a variable.
    // The new variable will shadow the old one.
    // The old variable will be inaccessible.
    let shadow_var: u32 = 10_000;
    println!(
        "shadow_var = {}, size = {};",
        shadow_var,
        std::mem::size_of_val(&shadow_var)
    );

    {
        let shadow_var: u32 = 20_000;
        println!(
            "shadow_var = {}, size = {};",
            shadow_var,
            std::mem::size_of_val(&shadow_var)
        );
    }

    {
        let shadow_var: u32 = 30_000;
        println!(
            "shadow_var = {}, size = {};",
            shadow_var,
            std::mem::size_of_val(&shadow_var)
        );
    }
    println!(
        "shadow_var = {}, size = {};",
        shadow_var,
        std::mem::size_of_val(&shadow_var)
    );
}

fn sum(a: u32, b: u32) -> u32 {
    let result = a + b;
    println!("Sum: {} + {} = {}", a, b, result);
    // In this case the result variable will be returned.
    // This can be done by not using semicolon.
    result
}

fn loops() {
    let mut multiple: u16 = 10;
    let mut counter: u16 = 0;

    while counter < 10 {
        counter += 1;
        println!("{} x {} = {}", multiple, counter, multiple * counter);
    }

    counter = 0;
    multiple = 20;

    loop {
        counter += 1;
        println!("{} x {} = {}", multiple, counter, multiple * counter);
        if counter >= 10 {
            break;
        }
    }

    multiple = 30;
    for counter in 1..=10 {
        println!("{} x {} = {}", multiple, counter, multiple * counter);
    }
}

fn primitive_types() {
    // variable always have fixed addresses in memory.
    // and also immutable by default.
    let var: u8 = 255;
    println!("var = {}, size = {};", var, std::mem::size_of_val(&var));

    let float: f32 = std::f32::consts::PI;
    println!(
        "float = {}, size = {};",
        float,
        std::mem::size_of_val(&float)
    );

    // Using mut keyword to make variable mutable.
    let mut boolean: bool = true;
    println!(
        "boolean = {}, size = {};",
        boolean,
        std::mem::size_of_val(&boolean)
    );
    // Assigning new value to mutable variable.
    boolean = false;
    println!(
        "boolean = {}, size = {};",
        boolean,
        std::mem::size_of_val(&boolean)
    );

    let char: char = 'V';
    println!("char = {}, size = {};", char, std::mem::size_of_val(&char));
}

fn borrowing(string: &mut String) {
    string.push_str(" is a Rustacean.");
    println!("borrowed string = {}", string);
}

fn ownership() {
    // String is a growable, heap-allocated data structure.
    // After the variable goes out of scope, the memory will be freed.
    let mut a_string = String::from("Vinicius");
    println!("a_string = {}", a_string);
    borrowing(&mut a_string);
}

fn main() {
    primitive_types();

    println!("CON = {}, size = {};", CON, std::mem::size_of_val(&CON));

    println!("STA = {}, size = {};", STA, std::mem::size_of_val(&STA));

    unsafe {
        // Always try to avoid using unsafe block.
        println!("UNSF = {}, size = {};", UNSF, std::mem::size_of_val(&UNSF));
        UNSF = 30_000;
        println!("UNSF = {}, size = {};", UNSF, std::mem::size_of_val(&UNSF));
    }

    scope();

    more_scope();

    shadowing();

    sum(10, 20);

    let age: u8 = 17;
    let underage_authorization: bool = true;

    if age >= 18 || age >= 16 && underage_authorization {
        println!("You are an adult.");
    } else {
        println!("You are happy.");
    }

    loops();

    let language = "Rust";
    match language {
        "Rust" => println!("You are a Rustacean."),
        "Python" => println!("You are a Pythonista."),
        "JavaScript" => println!("You are a JavaScripter."),
        _ => println!("You are a human."),
    }

    ownership();
}
