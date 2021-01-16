/// Trait with function for exporting a chart as *SVG*(and in the future other backends).
pub trait Export {
	/// Function for exporting a chart as *SVG*.
	fn export_svg(&self) -> std::io::Result<()>;
}
