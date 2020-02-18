use serde_json::{Result, Value};
use sha3::{Digest, Keccak256};
use std::any::Any;

pub fn wrap(input: &str, output: &str) {
	let mut hasher = Keccak256::default();
	hasher.input(input);
	let out = hasher.result();
	println!("{:x}", out);

	println!(
		"command ran was wrap {} {}",
		input,
		output
	);
	match untyped_example() {
		Err(e) => println!("{:?}", e),
		_ => ()
	}
}


fn untyped_example() -> Result<()> {
	// Some JSON input data as a &str. Maybe this comes from the user.
	let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

//	let mut hasher = Keccak256::default();
	// Parse the string of data into serde_json::Value.
	let v: Value = serde_json::from_str(data)?;
	println!("{}", v.is_object());
	let map = v.as_object().unwrap();
//	let map = v.type_id()?;
	for (key, value) in map {
		println!("{} / {}", key, value);
	}
//		let mut hasher = Keccak256::default();
//		hasher.input(y.get("name"));
//		let out = hasher.result();
//		out;
//	});

	// Access parts of the data by indexing with square brackets.
	println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//	println!("Please call {} at the number {}", x["name"], x["phones"][0]);

	Ok(())
}
