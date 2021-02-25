pub fn run() {
	// Print to console 
	println!(
		"Hello from the print.rs file"
	);

	// Basic Fomatting
	println!(
		"{} is a {}", "James", "Blockchain Developer"
	);

	// Positional Arguments
	println!(
		"{0} is a {1} and {0} loves to {2}",
	    "James", "Blockchain Developer", "Innovate"
	);

	// Named Arguments
	println!("{name} is fearlessly learning {activity}", 
		name = "James",
		activity = "programming"
	);

	// Placeholder Traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );

	// Placeholder for debug trait
	println!("{:?}",(12, true, "CipherProxy") );

	// Basic Math
	println!("10 + 10 = {}", 10 + 10 );
}