/// *scatter* module consists of the **ScatterChart** struct and its implementation.
/// # Example
/// ```
/// use rplot::charts::ScatterChart;
///
///let mut test_chart = ScatterChart::new((1280, 720));
/// assert_eq!(test_chart.title, "");
/// assert_eq!(test_chart.dimensions, (1280, 720));
/// assert_eq!(test_chart.x_label, "");
/// assert_eq!(test_chart.y_label, "");
/// assert_eq!(test_chart.padding, (0, 0, 0, 0));
/// test_chart.title = "Test Chart 2".to_string();
/// test_chart.dimensions = (1920, 1080);
/// test_chart.x_label = "X Values 2".to_string();
/// test_chart.y_label = "Y Values 2".to_string();
/// test_chart.padding = (10, 5, 10, 5);
/// assert_eq!(test_chart.title, "Test Chart 2");
/// assert_eq!(test_chart.dimensions, (1920, 1080));
/// assert_eq!(test_chart.x_label, "X Values 2");
/// assert_eq!(test_chart.y_label, "Y Values 2");
/// assert_eq!(test_chart.padding, (10, 5, 10, 5));
/// ```
pub mod scatter;
pub use scatter::ScatterChart;
