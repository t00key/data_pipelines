use csv::Writer;
use geocoding::{Forward, Openstreetmap, Point};
use std::fs::File;
use wi_dairy_pipeline::{extract, Plant};

fn transform() -> Vec<Plant> {
    //read in dairy_plant.csv file
    let file = File::open("dairy_plant.csv").unwrap();
    // read in file and covert each row to a plant struct which is stored in a plant vector
    let mut rdr = csv::Reader::from_reader(file);
    let mut plants = Vec::new();
    // skip first row as it is header
    for result in rdr.records() {
        let record = result.unwrap();
        let plant = Plant {
            licenseno: record[0].to_string(),
            wiplantno: record[1].to_string(),
            businessname: record[2].to_string(),
            dba: record[3].to_string(),
            businessphone: record[4].to_string(),
            streetaddress: record[5].to_string(),
            city: record[6].to_string(),
            statezip: record[7].to_string(),
            county: record[8].to_string(),
            municipality: record[9].to_string(),
            gradebprocessing1: record[10].to_string(),
            gradeapermitauthorization: record[11].to_string(),
            generalprocessing: record[12].to_string(),
            specificprocessing: record[13].to_string(),
            cheesemanufactured: record[14].to_string(),
            variancestatus: record[15].to_string(),
            variancedate: record[16].to_string(),
            latitude: -1.0,
            longitude: -1.0,
        };
        plants.push(plant);
    }
    //keep track of how many non matches are found
    let mut non_matches = 0;
    // for each plant in plants vector, geoencode the address and store the lat and long in the plant struct
    for p in &mut plants {
        let search_terms = p.search_terms();
        //geoencode the address by trying multple address formats until match is found, or use defaul tof 0,0
        //todo this is ugly, but it works - need to refactor to loop over search_terms
        let points = geo_encode_addr(&search_terms[0]).unwrap_or_else(|| {
            geo_encode_addr(&search_terms[1]).unwrap_or_else(|| {
                geo_encode_addr(&search_terms[2]).unwrap_or(vec![Point::new(0.0, 0.0)])
            })
        });
        // update struct with lat/long
        let point = points[0];
        let lat = point.y();
        let long = point.x();
        println!("location={}, lat/long={lat},{long}", p.streetaddress);
        //check if lat and long are 0.0, if so increment non_matches
        if lat == 0.0 && long == 0.0 {
            non_matches += 1;
        }

        //update struct for current iteration with lat/long
        p.latitude = lat;
        p.longitude = long;
        //sleep 1 second to not violate rate limit
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("total non_matches={}/{}", non_matches, plants.len());
    plants
}

fn geo_encode_addr(address: &str) -> Option<Vec<Point>> {
    let osm = Openstreetmap::new();
    // call osm.forward(address) and return option only if the vector in result is not empty, else return None
    let result = osm.forward(address).ok().filter(|r| !r.is_empty());
    result
}

fn write(plants: Vec<Plant>) {
    // create new file text file with .csv extention
    let file = std::fs::File::create("geo_dairy_plant.csv").expect("Unable to create file");
    // write to file
    let mut wtr = Writer::from_writer(file);
    // write each plant to file
    for plant in &plants {
        wtr.serialize(plant).expect("Unable to write record");
    }
    wtr.flush().expect("Unable to flush");
}


fn load(){
    println!("load to postgres DB...");
    //todo - implement!
    println!("complete");
}


fn main() {
    // call extract function and check if error or ok returned
    let target = "https://mydatcp.wi.gov/documents/dfrs/Public_Dairy_Plant_License_Holders.csv";
    extract(target, "dairy_plant.csv");
    // call transform function
    let plants = transform();
    write(plants);

    //todo
}
