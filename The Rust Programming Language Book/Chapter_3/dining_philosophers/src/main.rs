use std::thread; // "use" brings names into scope
				 // use the thread module
use std::sync::{Mutex, Arc};

struct Philosopher {
	name: String,
	left: usize,
	right: usize,
}

struct Table {
	forks: Vec<Mutex<()>>,
}

/*
	impl block lets us define things on Philosopher struct
*/
impl Philosopher {
	// define an associated function called "new"
	fn new(name: &str, left: usize, right: usize) -> Philosopher {
		Philosopher {
			name: name.to_string(), // copy the string
			left: left,
			right: right,
		}
	}

	fn eat(&self, table: &Table) {
		// methods take an explicit "self" parameter,
		// that's why "eat" is a method, but "new" is an
		// associated function

		// Vars prefiexed with "_" to tell Rust not to warn us about
		// unused vars.
		let _left = table.forks[self.left].lock().unwrap();
		let _right = table.forks[self.right].lock().unwrap();

		println!("{} is eating", self.name);

		thread::sleep_ms(1000);

		println!("{} is done eating!", self.name);
	}
}

fn main() {
	let table = Arc::new(Table { forks: vec![
		Mutex::new( () ),
		Mutex::new( () ),
		Mutex::new( () ),
		Mutex::new( () ),
		Mutex::new( () ),
		]});

	let philosophers = vec![
		// let p1 = Philosopher { name: "Philosopher 1".to_string };
		Philosopher::new("Philosopher 1", 0, 1),
		Philosopher::new("Philosopher 2", 1, 2),
		Philosopher::new("Philosopher 3", 2, 3),
		Philosopher::new("Philosopher 4", 3, 4),
		Philosopher::new("Philosopher 5", 0, 4),
	];

	// .into_iter() creates an iterator that takes ownership of each philosopher
	// .map() takes a closure as an arg and calls that closure on each element in turn
	let handles: Vec<_> = philosophers.into_iter().map(|p| {
		let table = table.clone(); // bumps up referrence count

		thread::spawn(move || {
			p.eat(&table);
		}) // no ";", thead::spawn is an expression
	}).collect();

	for h in handles {
		h.join().unwrap();
	}
}

/*
let p1 = Philosopher { name: "Philosopher 1".to_string } | 
let p1 = Philosopher::new("Philosopher 1")
*/