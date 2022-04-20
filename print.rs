pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    //Basic Formatting
    println!("{} is from {}", "Adrian", "Tx");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
    "Adrian", "Tx", "code" );

    //Named Arguments
    println!("{name} likes to play {activity}", 
    name = "Adrian", 
    activity = "Soccer");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} octal: {:0}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10+10 = {}", 10+10);
}