use crate::charts::*;
#[test]
fn create_scatter_chart() {
	let test_chart = ScatterChart::new("test_chart.svg", (1280, 720));
	assert_eq!(test_chart.file_path, "test_chart.svg");
	assert_eq!(test_chart.title, "");
	assert_eq!(test_chart.dimensions, (1280, 720));
	assert_eq!(test_chart.x_label, "");
	assert_eq!(test_chart.y_label, "");
	assert_eq!(test_chart.padding, (0, 0, 0, 0));
}

#[test]
fn modify_scatter_chart() {
	let mut test_chart = ScatterChart::new("test_chart.svg", (1280, 720));
	test_chart.file_path = "test_chart2.svg".to_string();
	test_chart.title = "Test Chart".to_string();
	test_chart.dimensions = (1920, 1080);
	test_chart.x_label = "X Values".to_string();
	test_chart.y_label = "Y Values".to_string();
	test_chart.padding = (10, 5, 10, 5);
	assert_eq!(test_chart.file_path, "test_chart2.svg");
	assert_eq!(test_chart.title, "Test Chart");
	assert_eq!(test_chart.dimensions, (1920, 1080));
	assert_eq!(test_chart.x_label, "X Values");
	assert_eq!(test_chart.y_label, "Y Values");
	assert_eq!(test_chart.padding, (10, 5, 10, 5));
}
