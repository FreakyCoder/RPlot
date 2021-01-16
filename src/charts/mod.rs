/// **charts** module consists of the **Chart** struct and its implementation.
/// # Example
/// ```
/// use rplot::charts::Chart;
///
/// let mut test_chart = Chart::new("test_chart.svg", (1280, 720));
/// assert_eq!(test_chart.file_path, "test_chart.svg");
/// assert_eq!(test_chart.title, "");
/// assert_eq!(test_chart.dimensions, (1280, 720));
/// assert_eq!(test_chart.x_label, "");
/// assert_eq!(test_chart.y_label, "");
/// assert_eq!(test_chart.padding, (0, 0, 0, 0));
/// test_chart.file_path = "test_chart2.svg".to_string();
/// test_chart.title = "Test Chart 2".to_string();
/// test_chart.dimensions = (1920, 1080);
/// test_chart.x_label = "X Values 2".to_string();
/// test_chart.y_label = "Y Values 2".to_string();
/// test_chart.padding = (10, 5, 10, 5);
/// assert_eq!(test_chart.file_path, "test_chart2.svg");
/// assert_eq!(test_chart.title, "Test Chart 2");
/// assert_eq!(test_chart.dimensions, (1920, 1080));
/// assert_eq!(test_chart.x_label, "X Values 2");
/// assert_eq!(test_chart.y_label, "Y Values 2");
/// assert_eq!(test_chart.padding, (10, 5, 10, 5));
/// ```
pub mod chart;
pub use chart::Chart;

/// **svg** module consists of the Export trait for exporting a chart as *SVG*.
pub mod export;
pub use export::Export;
