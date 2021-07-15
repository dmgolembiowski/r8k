#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(unused_imports)]

use std::collections::HashMap;
use std::borrow::Cow;
use std::char;
use std::str::FromStr;

macro_rules! bounds {
    ($start:expr, $stop:expr) => {
        [("start", $start), 
         ("end",   $stop)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>()
    }
}

macro_rules! chr {
    ($str_int:expr) => {
        char::from_u32($str_int).unwrap()
    }
}

pub fn rot_8000<'a>(text: impl Into<Cow<'a, str>>) -> Result<String, DecipherError> { 
    
    let mut outstring = String::new();
 
    let start_block: u32 = 55296;
    let rotate_num: u32  = 31702;
    
    let hiddenblocks: [HashMap<&str, u32>; 9] = [
        bounds!(0, 32),
        bounds!(127, 160),
        bounds!(5760, 5760),
        bounds!(8192, 8202),
        bounds!(8232, 8233),
        bounds!(8239, 8239),
        bounds!(8287, 8287),
        bounds!(12288, 12288),
        bounds!(55296, 57343),
    ];
    let valid_int_list = [
      ("33",    true), ("127",   false),
      ("161",   true), ("5760",  false),
      ("5761",  true), ("8192",  false),
      ("8203",  true), ("8232",  false),
      ("8234",  true), ("8239",  false),
      ("8240",  true), ("8287",  false),
      ("8288",  true), ("12288", false),
      ("12289", true), ("55296", false),
      ("57344", true)];
   {
        let mut rotlist: HashMap<char, char> = HashMap::with_capacity(63404);
        type Code = Result<char, <char as FromStr>::Err>;
        valid_int_list
            .into_iter()
            .enumerate()
            .for_each(|(i, kv_pair)| {
                match kv_pair {
                    (char_code, valid) => {
                        let code: Code;
                        if let code = chr!(char_code.parse().unwrap()) {                            
                            let rotate_num_ = 31702;
                            let rot: usize = (i + rotate_num_) % (rotate_num_ * 2);
                            let slot: char = valid_int_list[rot].0.parse().unwrap();
                            &rotlist.insert(code, slot.into());
                        } else {
                            panic!("Could not safely convert each `u32` to a UTF-8 character.");
                                                    
                        }
                    }
                        
                }
            });
        
 
        let text: Cow<'a, str> = text.into();
        
        for (i, chara) in text.chars().enumerate() {
            
            // If it is not in the collection of mappings
            // then add it directly (without rotation)

            if !(&rotlist.contains_key(&chara)) {
                &mut outstring.push(chara);
                continue;
            }

            // Otherwise, rotate it and add it to the response
            &mut outstring.push(rotlist.get(&chara).unwrap().clone());
        }
        
        return Ok(outstring)
    }

}

#[derive(Debug)]
pub struct DecipherError(String);

#[cfg(test)]
mod tests {
    use crate::rot_8000;
    #[test]
    fn test_borrowed_str() {
        assert_eq!(rot_8000("F").unwrap(), String::from("籏"));
    }
}
