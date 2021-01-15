/// Container for all properties related to the scatter chart.\
/// It stores the *chart title*, *resolution*, *x* and *y labels*, and the *padding* of the chart.
/// # Example
/// ```
/// use rplot::charts::ScatterChart;
///
/// let mut test_chart = ScatterChart::new("test_chart.svg", (1280, 720));
/// test_chart.title = "Test Chart".to_string();
/// test_chart.x_label = "X Values".to_string();
/// test_chart.y_label = "Y Values".to_string();
/// test_chart.padding = (5, 10, 5, 10);
/// assert_eq!(test_chart.file_path, "test_chart.svg");
/// assert_eq!(test_chart.title, "Test Chart");
/// assert_eq!(test_chart.x_label, "X Values");
/// assert_eq!(test_chart.y_label, "Y Values");
/// assert_eq!(test_chart.padding, (5, 10, 5, 10));
/// ```
pub struct ScatterChart {
	/// The file path where the chart image should be saved.
	pub file_path: String,
	/// The *title* of the chart.
	pub title: String,
	/// The *dimensions* of the chart.
	pub dimensions: (u32, u32),
	/// The *label* on the *x axis*.
	pub x_label: String,
	/// The *label* on the *y axis*.
	pub y_label: String,
	/// The *padding* from the borders of the image to the **Chart**.\
	/// The padding order is: *top*, *right*, *bottom*, *left*.
	pub padding: (u32, u32, u32, u32),
}

impl ScatterChart {
	/// Create a new *ScatterChart* with default values and given dimensions as (u32, u32).\
	/// # Example
	/// ```
	/// use rplot::charts::ScatterChart;
	/// let mut test_chart = ScatterChart::new("test_chart.svg", (1280, 720));
	///
	/// assert_eq!(test_chart.file_path, "test_chart.svg");
	/// assert_eq!(test_chart.title, "");
	/// assert_eq!(test_chart.dimensions, (1280, 720));
	/// assert_eq!(test_chart.x_label, "");
	/// assert_eq!(test_chart.y_label, "");
	/// assert_eq!(test_chart.padding, (0, 0, 0, 0));
	/// ```
	pub fn new(file_path: &str, dimensions: (u32, u32)) -> Self {
		Self {
			file_path: file_path.to_string(),
			title: "".to_string(),
			dimensions: dimensions,
			x_label: "".to_string(),
			y_label: "".to_string(),
			padding: (0, 0, 0, 0),
		}
	}
}
