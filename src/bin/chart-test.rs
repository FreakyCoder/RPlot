use rplot::charts::Chart;
use rplot::charts::Export;

fn main() {
	let mut test_chart = Chart::new("src/tests/images/test_chart.svg", (1280, 720));
	test_chart.title = "Test Chart".to_string();
	test_chart.padding.0 = 10;
	test_chart.export_svg().expect("Could not export chart!");
}
