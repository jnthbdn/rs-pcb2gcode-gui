pub trait Tool {
    fn name(&self) -> String;
    fn offset(&self, depth_cut: f32) -> f32;
    fn shaft_diameter(&self) -> f32;
    fn single_pass_depth(&self) -> f32;
    fn spindle_speed(&self) -> f32;
    fn vertical_speed(&self) -> f32;
    fn horizontal_speed(&self) -> f32;

    // fn get_base_tool()
}

// pub trait ToolBuilder {
//     type Type: Sized + Default;

//     fn build() -> Self::Type {
//         Self::Type::default()
//     }

//     fn name(s: Self::Type, name: String) -> Self::Type {
//         s.base_tool.name = name;
//         s
//     }
// }
