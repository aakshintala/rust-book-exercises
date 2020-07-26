fn gen_fibonacci(minus2:i32, minus1:i32, n :i32, n_max : i32) {
	print!("{} ", minus2);
	if n < n_max {
		gen_fibonacci(minus1, minus2+minus1, n+1, n_max);
	}
}

fn main() {
	println!("Hello! My only purpose is to generate the fibonacci sequence.\nHow many numbers of the sequence would you like me to generate? ");
    let mut count_till = String::new();
    std::io::stdin().read_line(&mut count_till).expect("Enter a number only please!");

    let count_till : i32 = match count_till.trim().parse() {
    	Ok(num) => num,
    	Err(_) => panic!("Invalid input! Exiting.")
    };

    gen_fibonacci(0, 1, 0, count_till);
    println!("");
}
