#[warn(missing_docs)]
/// *charts* module consists of structs and their implementations for the different chart types.
/// # Example
/// ```
/// use rplot::charts::Chart;
///
/// let mut test_chart = Chart::new("test_chart.svg", (1280, 720));
/// test_chart.title = "Test Chart".to_string();
/// test_chart.x_label = "X Values".to_string();
/// test_chart.y_label = "Y Values".to_string();
/// test_chart.padding = (10, 5, 10, 5);
/// assert_eq!(test_chart.file_path, "test_chart.svg");
/// assert_eq!(test_chart.title, "Test Chart");
/// assert_eq!(test_chart.dimensions, (1280, 720));
/// assert_eq!(test_chart.x_label, "X Values");
/// assert_eq!(test_chart.y_label, "Y Values");
/// assert_eq!(test_chart.padding, (10, 5, 10, 5));
/// ```
pub mod charts;

// Reference test module
#[cfg(test)]
mod tests;
