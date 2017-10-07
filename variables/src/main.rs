fn main() {
	// re-assignment of immutable variable, will cause error[E0384]
	//let x = 5;
	//println!("The value of x is: {}", x);
	//x = 6;
	//println!("The value of x is: {}", x);

	// example of correct mutable usage
	let mut x = 5;
    	println!("The value of x is: {}", x);
    	x = 6;
    	println!("The value of x is: {}", x);
}
