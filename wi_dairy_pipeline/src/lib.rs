use std::fs::File;
use std::io::copy;

pub fn extract(target_url: &str, file_name: &str) {
    // get data from target url
    let response = reqwest::blocking::get(target_url);
    // match response to check if error or ok
    match response {
        Ok(response) => {
            // from response get data from body and store in body variable
            let body = response.bytes().unwrap();
            //create file to store data
            let mut file = File::create(file_name).unwrap();
            //copy data from body to file
            copy(&mut body.as_ref(), &mut file).unwrap();
            println!("Downloaded file to dairy_plant.csv");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plant {
    pub licenseno: String,
    pub wiplantno: String,
    pub businessname: String,
    pub dba: String,
    pub businessphone: String,
    pub streetaddress: String,
    pub city: String,
    pub statezip: String,
    pub county: String,
    pub municipality: String,
    pub gradebprocessing1: String,
    pub gradeapermitauthorization: String,
    pub generalprocessing: String,
    pub specificprocessing: String,
    pub cheesemanufactured: String,
    pub variancestatus: String,
    pub variancedate: String,
    pub latitude: f64,
    pub longitude: f64,
}
//implement debug
impl std::fmt::Debug for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Plant {{ licenseno: {}, wiplantno: {}, businessname: {}, dba: {}, businessphone: {}, streetaddress: {}, city: {}, statezip: {}, county: {}, municipality: {}, gradebprocessing1: {}, gradeapermitauthorization: {}, generalprocessing: {}, specificprocessing: {}, cheesemanufactured: {}, variancestatus: {}, variancedate: {}, latitude: {}, longitude: {} }}",
            self.licenseno,
            self.wiplantno,
            self.businessname,
            self.dba,
            self.businessphone,
            self.streetaddress,
            self.city,
            self.statezip,
            self.county,
            self.municipality,
            self.gradebprocessing1,
            self.gradeapermitauthorization,
            self.generalprocessing,
            self.specificprocessing,
            self.cheesemanufactured,
            self.variancestatus,
            self.variancedate,
            self.latitude,
            self.longitude
        )
    }
}

impl Plant {
    pub fn search_terms(&self) -> Vec<String> {
        let muni = self
            .municipality
            .replace("Town of ", "")
            .replace("Village of ", "")
            .replace("City of ", "");
        let county = format!("{} County", self.county);
        vec![
            format!("{}, {}, {}", self.streetaddress, self.city, self.statezip), // full street address search
            format!("{} {}, {}", self.streetaddress, self.county, self.statezip), //county search
            format!(
                "{} {}, {}, {}",
                self.streetaddress, muni, county, self.statezip
            ), //muni search
            format!(
                "{} {}, {},{}",
                self.streetaddress, muni, self.county, self.statezip
            ), //muni search - for loc like 105 E 3rd Ave, Weyauwega, WI 54983
        ]
    }
}
