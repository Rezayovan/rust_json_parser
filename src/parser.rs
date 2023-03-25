use core::fmt;
use std::fmt::Display;
use std::error::Error;
use std::iter::TakeWhile;
use std::ops::Index;
use std::str::FromStr;
use std::str::Chars;


#[derive(Debug)]
pub struct JsonObject {
    pub objects: Vec<(String, JsonEnum)>
}

#[derive(Debug)]
pub struct JsonArray {
    pub objects: Vec<JsonEnum>,
}

#[derive(Debug)]
pub struct JsonString {
    pub value: String
}

#[derive(Debug)]
pub struct JsonNumber {
    pub value: f64
}

const QUOTE_OPERATOR: char = '"';

#[derive(Debug)]
pub enum JsonEnum {
    Object(JsonObject),
    Array(JsonArray),
    String(JsonString),
    Number(JsonNumber),
    Empty
}

pub struct JsonParser {
    pub raw_json: String,
    pub json_vec: Vec<char>,
    pub pos: usize,
    pub len: usize,
}

impl JsonParser {

    pub fn from_str(&mut self) -> Result<JsonEnum, JsonError> {
        self.pos = 0;
        self.json_vec = self.raw_json.chars().collect();
        self.len = self.json_vec.len();
        return self.from_str_helper();
    }

    pub fn from_str_helper(&mut self) -> Result<JsonEnum, JsonError>{
        let mut result = Ok(JsonEnum::Empty);

        while self.pos < self.len {
            let c = self.json_vec[self.pos];
            if c == ' ' || c == '\n' {
                self.pos += 1;
                continue;
            }

            self.pos += 1;

            let result = match c {
                '"' => self.parse_string(),
                '[' => self.parse_array(),
                '{' => self.parse_object(),
                ' ' => continue,
                _ => Err(JsonError::BadCharacter(String::from(c)))
            };

            return result;
        }

        return result;
    }

    fn parse_string(&mut self) -> Result<JsonEnum, JsonError> {
        println!("parsing str");
        let mut s = "".to_string();
        while self.pos < self.len {
            let c = self.json_vec[self.pos];
            if c == '"' {
                self.pos += 1;
                return Ok(JsonEnum::String(JsonString {value: s}));
            }

            s.push(c);
            self.pos += 1;
        }

        // this should be an error since we finished string without ending quote
        return Err(JsonError::NoClosingChar("'\"' needed".to_string()));
    }

    fn parse_array(&mut self) -> Result<JsonEnum, JsonError> {
        let mut json_arr: Vec<JsonEnum> = Vec::new();
        while self.pos < self.len {
            let c = self.json_vec[self.pos];
            if c == ' ' {
                self.pos += 1;
                continue;
            }
            if c == ']' {
                break;
            }

            json_arr.push(self.from_str_helper()?);

            // need to get end or comma
            while self.pos < self.len {
                let e = self.json_vec[self.pos];
                if e == ']' || e == ',' {
                    self.pos += 1;
                    break;
                }
                if e == ' ' || e == '\n' {
                    self.pos += 1;
                    continue;
                }

                dbg!(&self.json_vec[self.pos]);
                dbg!(self.pos);
                return Err(JsonError::BadCharacter(format!("{e}")));
            }
        }

        return Ok(JsonEnum::Array(JsonArray { objects: json_arr }));
    }

    fn parse_object(&self) -> Result<JsonEnum, JsonError> {
        let mut objects: Vec<(String, JsonEnum)> = Vec::new();
        while self.pos < self.len {
            
        }
        return Ok(JsonEnum::Empty);
    }

    fn parse_number(&self) -> Result<JsonEnum, JsonError> {
        return Ok(JsonEnum::Empty);
    }
}



#[derive(Debug)]
pub enum JsonError {
    BadCharacter(String),
    NoClosingChar(String)
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            JsonError::BadCharacter(y) => write!(f, "Bad character: {}", y),
            JsonError::NoClosingChar(y) => write!(f, "No closing char: {}", y)
        };
    }
}

impl Error for JsonError {}

pub fn to_str(json: &JsonEnum) -> String {
    return match json {
        JsonEnum::Object(o) => "object undefined".to_string(),
        JsonEnum::Array(o) => "array undefined".to_string(),
        JsonEnum::Number(o) => o.value.to_string(),
        JsonEnum::Empty => "empty".to_string(),
        JsonEnum::String(o) => {
            let mut owned_string = "".to_owned();
            owned_string.push_str("\"");
            owned_string.push_str(&o.value);
            owned_string.push_str("\"");
            return owned_string;
        },
    };
}

