use regex::Regex;

pub fn get_key_from_json(target: String) -> Vec<String> {
	let find_key = Regex::new(r#""(?P<key>.+)":"#).unwrap();
	let mut keys: Vec<String> = vec![];
	for caps in find_key.captures_iter(&target) {
		keys.push(caps[0].replace(r#"""#, "").replace(":", "").to_string());
		// 中身の確認用
		// println!("{}", &caps[0].replace(r#"""#, "").replace(":", ""));
	}
	keys
}