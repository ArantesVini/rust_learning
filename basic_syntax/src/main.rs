// Constant values don't have fixed addresses in memory.
const CON: u32 = 100_000;
// Static values have fixed addresses in memory.
static STA: u32 = 50_000;

static mut UNSF: u32 = 20_000;

fn main() {
    println!("CON = {}, size = {};", CON, std::mem::size_of_val(&CON));

    println!("STA = {}, size = {};", STA, std::mem::size_of_val(&STA));

    unsafe {
        // Always try to avoid using unsafe block.
        println!("UNSF = {}, size = {};", UNSF, std::mem::size_of_val(&UNSF));
        UNSF = 30_000;
        println!("UNSF = {}, size = {};", UNSF, std::mem::size_of_val(&UNSF));
    }

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
