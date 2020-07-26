fn main() {
	let day_count = ["first", "second", "third", "fourth", "fifth", "sizth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
	let day_gift = ["partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five gold rings, badam-pam-pam", "Six geese a laying", "Seven swans a swimming", "Eight maids a milking", "Nine ladies dancing", "Ten lords a leaping", "Eleven pipers piping", "Twelve drummers drumming"];

	for day in 0..12 {
    	println!("On the {} day of Christmas", day_count[day]);
    	println!("My true love gave to me");
    	if day == 0 {
    		print!("A {}", day_gift[0]);
    	} else {
    		for i in (1..day+1).rev() {
    			println!("{}", day_gift[i]);
    		}
    		print!("And a {}", day_gift[0]);
    	}
    	println!("\n");
	}
}