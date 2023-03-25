

#[cfg(test)]
mod tests {

use crate::parser::{JsonEnum, JsonNumber, from_str};

    #[test]
    fn test_from_str_single_number() {
        let exp = JsonEnum::Number(JsonNumber {value: 3.0});
        let res = from_str("3");
        if let JsonEnum::Number(val) = res {
            assert_eq!(val.value, 3.0);
        } 
        else {
            assert!(false);
        }

    }

}


