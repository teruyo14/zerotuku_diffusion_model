use plotters::prelude::*;
use std::f64::consts::PI;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "plotters-doc-data";
    fs::create_dir_all(path)?;

    let file_path = format!("{}/0.png", path);

    let root = BitMapBackend::new(&file_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(-5f64..5f64, -0.005f64..0.4f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("y")
        .x_desc("x")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (-220..=220)
                .map(|x| x as f64 / 50.0)
                .map(|x| (x, normal(x, 0., 1.))),
            &BLUE,
        ))?
        .label("Normal Distribution")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn normal(x: f64, mu: f64, sigma: f64) -> f64 {
    let y = 1. / ((2. * PI).sqrt() * sigma) * (-(x - mu).powf(2.) / (2. * sigma.powf(2.))).exp();
    y
}
