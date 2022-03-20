# Exploring the Plotters Library in Rust

Exploring the [Plotters library](https://docs.rs/plotters/latest/plotters/) for data visualization in Rust.

## Steps to creating a plot using Plotters

1. Prepare the data to plot.
2. Setup a drawing area. You can draw in this area using [built-in elements](https://docs.rs/plotters/latest/plotters/#elements) or user [composed elements](https://docs.rs/plotters/latest/plotters/#composable-elements). Examples: bitmap, SVG, HTML5 canvs, and Cairo/GTK.
3. Create a [Chart Context](https://docs.rs/plotters/latest/plotters/#chart-context). They are data objects built on top of the drawing area. For example: label areas, a mesh, cartesian coordinates, data series, etc.
4. Draw a [data series](https://docs.rs/plotters/latest/plotters/series/index.html).