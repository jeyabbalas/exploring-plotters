use csv::Reader;
use fast_float;
use plotters::prelude::*;
use std::error::Error;
use std::path::Path;


fn read_housing_price_sqft_living(filepath: &Path) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let mut price_sqft_living: Vec<(f64, f64)> = Vec::new();
    let mut rdr = Reader::from_path(filepath)?;
    for result in rdr.records() {
        let record = result?;
        let price: f64 = fast_float::parse(&record[2]).unwrap();
        let sqft_living: f64 = fast_float::parse(&record[5]).unwrap();
        price_sqft_living.push((price / 1000.0, sqft_living))
    }
    Ok(price_sqft_living)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Prepare the data to plot.
    let path = Path::new("./data/kc_house_data.csv");
    let price_sqft_living = read_housing_price_sqft_living(path).unwrap();

    // 2. Setup a drawing area (layout).
    let root = BitMapBackend::new("./plots/scatter.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(60, 10, 10, 60);

    // 3. Create a Chart Context.
    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 80.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 60.0)
        .caption("House Sales in King County", ("sans-serif", 30.0))
        .build_cartesian_2d(0.0..8_000.0, 0.0..10_000.0)?;
    chart
        .configure_mesh()
        .y_desc("Price (thousand US dollars)")
        .x_desc("Interior living space (ft^2)")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;
    
    // 4. Draw a data series
    chart.draw_series(price_sqft_living.iter().map(|(price, sqft_living)| Circle::new((*price, *sqft_living), 3.0_f64, RED.mix(0.2).filled())))?;

    Ok(())
}
