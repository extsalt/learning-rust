use std::thread;
use std::time::Duration;

fn main() {
	thread::spawn(|| {
		for i in 1..100000000 {
			println!("From thread {:?}", i);
		}
	});

	for i in 1..100 {
		println!("{:?}",i);
		thread::sleep(Duration::from_millis(1000000));
	}
}
