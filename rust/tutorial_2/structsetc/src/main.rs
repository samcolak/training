#![allow(dead_code)]
#![allow(clippy::unnecessary_unwrap)]
#![allow(clippy::let_and_return)]

use chrono::{DateTime, Datelike, Local, Utc};




#[derive(Debug, Clone)]
struct PersonRecord {
    name: String,
    year: i32,
    location: String
}


impl PersonRecord {

    fn age(&self) -> u8 {
        let _t = Local::now();
        (_t.year() - self.year) as u8
    }

}


fn main() {
    
    let mut _peoplein: Vec<PersonRecord> = Vec::new();

    // i want to see the ppl in the list of (ppl.json) read into this structure... (peoplein)

    let _sam = PersonRecord { name: "Sam".to_string(), year: 1973, location: "Koeln".to_string() };
    let _varad = PersonRecord { name: "Varad".to_string(), year: 1996, location: "Augsburg".to_string() };

    let _age = _varad.age();
    let _sams_age = PersonRecord::age(&_sam);
    let _varads_age = PersonRecord::age(&_varad);

    let mut _varad_copy = _varad.clone();
    _varad_copy.location = "Muenchen".to_string();
    _varad_copy.year = 1992;

    println!("Hello - {}, your age is {}, located in {}", _varad.name, _varad.age(), _varad.location);
    println!("Hello - {}, your age is {}, located in {}", _varad_copy.name, _varad_copy.age(), _varad_copy.location);

}
