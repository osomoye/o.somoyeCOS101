fn main() {
	let toshiba:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;

	//sum
	let total = (toshiba * 2.0) + mac + (hp * 3.0) + (dell * 3.0) + acer;
	println!("The sum is {}", total);

	// Average cost
	let avg = (toshiba + mac + hp + dell + acer)/5.0;
	println!("The average cost of items is {}", avg);
}
