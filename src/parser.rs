use core::fmt;
use std::error::Error;


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
        self.parse_whitespace();
        return self.from_str_helper();
    }

    pub fn from_str_helper(&mut self) -> Result<JsonEnum, JsonError>{
        let result: Result<JsonEnum, JsonError> = Ok(JsonEnum::Empty);

        let c = self.json_vec[self.pos];

        let result = match c {
            '"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            _ => Err(JsonError::BadCharacter(format!("from_str_helper {c}")))
        };
        self.parse_whitespace();

        return result;
    }

    fn parse_string(&mut self) -> Result<JsonEnum, JsonError> {
        let mut s = "".to_string();
        self.pos += 1; // opening quotation
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
        return Err(JsonError::NoClosingChar(format!("parse_string, '\"' needed. string is {s}")));
    }

    fn parse_array(&mut self) -> Result<JsonEnum, JsonError> {
        let mut json_arr: Vec<JsonEnum> = Vec::new();
        
        self.pos += 1; // opening bracket
        while self.pos < self.len {
            self.parse_whitespace();
            let c = self.json_vec[self.pos];
            if c == ']' {
                break;
            }
            json_arr.push(self.from_str_helper()?);

            // need to get end or comma
            self.parse_whitespace();
            match self.json_vec[self.pos] {
                ']' => {
                    self.pos += 1;
                    break;
                },
                ',' => {
                    self.pos += 1;
                    continue;
                },
                 x => return Err(JsonError::BadCharacter(format!("parse_array: {x}")))
            }
        }
        return Ok(JsonEnum::Array(JsonArray { objects: json_arr }));
    }

    fn parse_object(&mut self) -> Result<JsonEnum, JsonError> {
        let mut object_vec: Vec<(String, JsonEnum)> = Vec::new();
        
        while self.pos < self.len {
            self.pos += 1;
            self.parse_whitespace();
            let key = match self.parse_string()? {
                JsonEnum::String(x) => x,
                _ => return Err(JsonError::BadCharacter(format!("parse_object: shouldnt get here")))
            };
            let res = match self.json_vec[self.pos] {
                ':' => {
                    self.pos += 1;
                    self.parse_whitespace();
                    self.from_str_helper()?
                },
                x => return Err(JsonError::BadCharacter(format!("parse_object, expected ':', got {x}")))
            };
            
            object_vec.push((key.value, res));

            // need to get end or comma
            dbg!(&object_vec);
            match self.json_vec[self.pos] {
                '}' => {
                    self.pos += 1;
                    break;
                },
                ',' => {
                    self.pos += 1;
                    continue;
                },
                 x => return Err(JsonError::BadCharacter(format!("parse_object: {x}")))
            }
        }
        return Ok(JsonEnum::Object(JsonObject { objects: object_vec }));
    }

    fn parse_number(&self) -> Result<JsonEnum, JsonError> {
        return Ok(JsonEnum::Empty);
    }

    fn parse_whitespace(&mut self) {
        let mut c = self.json_vec[self.pos];
        while self.pos < self.len - 1 && (c == ' ' || c == '\n') {
            self.pos += 1;
            c = self.json_vec[self.pos];
        }
    }
}



#[derive(Debug)]
pub enum JsonError {
    BadCharacter(String),
    NoClosingChar(String),
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

