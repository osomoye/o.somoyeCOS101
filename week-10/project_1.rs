struct Laptop {
	// brand:String,
	quanitity:u32,
	price_each:u32
}

impl Laptop {
	fn cost(&self)->u32 {
		self.quanitity * self.price_each
	}
}

fn main() {

	let pc1 = Laptop {
		// brand:String::from("HP"),
		quanitity:3,
		price_each:650_000
	};
	let pc2 = Laptop {
		// brand:String::from("IBM"),
		quanitity:3,
		price_each:755_000
	};
	let pc3 = Laptop {
		// brand:String::from("Toshiba"),
		quanitity:3,
		price_each:550_000
	};
	let pc4 = Laptop {
		// brand:String::from("Dell"),
		quanitity:3,
		price_each:850_000
	};


	println!("Price of 3 HP laptops is {} \n Price of 3 IBM laptops is {} \n 
		Price of 3 Toshiba is {} \n Price of Dell is {}", pc1.cost(), pc2.cost(), pc3.cost(), pc4.cost() );
}


