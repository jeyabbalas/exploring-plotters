use csv::Reader;
use fast_float;
use std::error::Error;
use std::path::Path;
use std::process;


fn read_housing_price_sqft_living(filepath: &Path) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let mut price_sqft_living: Vec<(f64, f64)> = Vec::new();
    let mut rdr = Reader::from_path(filepath)?;
    for result in rdr.records() {
        let record = result?;
        let price: f64 = fast_float::parse(&record[2]).unwrap();
        let sqft_living: f64 = fast_float::parse(&record[2]).unwrap();
        price_sqft_living.push((price, sqft_living))
    }
    Ok(price_sqft_living)
}

fn main() {
    // 1. Data Preparation
    let path = Path::new("./data/kc_house_data.csv");
    let price_sqft_living = read_housing_price_sqft_living(path).unwrap();
    for (price, sqft_living) in price_sqft_living {
        println!("{price}, {sqft_living}");
    }

    // 2. 
}
