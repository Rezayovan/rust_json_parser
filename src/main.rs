
mod parser;
mod test;

fn main() {
    println!("Hello, world!");
    let json_str = "[\"one\"   ,\"2\"   ]".to_string();
    let mut js_parser = parser::JsonParser { raw_json: json_str, json_vec: vec![], len: 0, pos: 0 };
    let res = js_parser.from_str();
    //let to_string = parser::to_str(&res);
    dbg!("{}", res);
    //println!("{}", to_string);

    //let somestr = "asfas asdfsa fas aa  \"";
    //let mut iter = somestr.chars();
    //let sl: String = iter.take_while(|&x| x != '"').collect();
    
    //println!("{}", sl);
    //

}

