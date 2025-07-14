use std::fs::File;
use std::io::BufReader;
use obj::Obj;

pub fn load_obj(path: &str) -> Obj {
	let input = BufReader::new(File::open(path).unwrap());
	let model: Obj = obj::load_obj(input).unwrap();
	println!("Loaded model: {:?}", model.name);

	model
}
