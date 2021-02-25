  
// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language


pub fn run() {
	let name = "James";
	let mut age = 24;
	println!("My name is {} and i am {}", name, age);
	age = 25;
	println!("My name is {} and i am {}", name, age);

	// Define Constant
	const ID: i32 =001;
	println!("ID: {}", ID);

	// Assign Multiple Variables
	let (my_name, my_age ) = ("James", 24);
	println!("{} is {}", my_name, my_age );
}