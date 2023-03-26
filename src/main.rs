
mod parser;
mod test;

fn main() {
    let json_str = "
    {
        \"123\": {\"key1\": [\"456\", \"chortle\"]},
        \"789\": \"101112\",
        \"131415\": \"161718\"
    }
        ";
    let mut js_parser = parser::JsonParser { raw_json: json_str.to_string(), json_vec: vec![], len: 0, pos: 0 };
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

