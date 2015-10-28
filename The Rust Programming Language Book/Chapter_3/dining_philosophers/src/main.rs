use std::thread; // "use" brings names into scope
				 // use the thread module

struct Philosopher {
	name: String,
}

/*
	impl block lets us define things on Philosopher struct
*/
impl Philosopher {
	// define an associated function called "new"
	fn new(name: &str) -> Philosopher {
		Philosopher {
			name: name.to_string(), // copy the string
		}
	}

	fn eat(&self) {
		// methods take an explicit "self" parameter,
		// that's why "eat" is a method, but "new" is an
		// associated function
		println!("{} is eating", self.name);

		thread::sleep_ms(1000);

		println!("{} is done eating!", self.name);
	}
}

fn main() {
	let philosophers = vec![
		// let p1 = Philosopher { name: "Philosopher 1".to_string };
		Philosopher::new("Philosopher 1"),
		Philosopher::new("Philosopher 2"),
		Philosopher::new("Philosopher 3"),
		Philosopher::new("Philosopher 4"),
		Philosopher::new("Philosopher 5"),
	];

	let handles: Vec<_> = philosophers.into_iter().map(|p| {
		thread::spawn(move || {
			p.eat();
		})
	}).collect();

	for h in handles {
		h.join().unwrap();
	}
}

/*
let p1 = Philosopher { name: "Philosopher 1".to_string } | 
let p1 = Philosopher::new("Philosopher 1")
*/