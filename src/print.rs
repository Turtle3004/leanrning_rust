pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "brad", "mass");

    // Positional arguements
    println!(
        "{0} is from {1} and {0} like to {2}",
        "brad", "mass", "code"
    );

    // Named arguements
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
