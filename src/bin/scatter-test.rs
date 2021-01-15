use rplot::charts::ScatterChart;

fn main() {
	let mut test_chart = ScatterChart::new("test_chart.svg", (1280, 720));
	test_chart.title = "Test Chart".to_string();
	println!("{}", test_chart.title);
}
