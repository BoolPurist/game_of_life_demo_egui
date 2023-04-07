use eframe::epaint::Color32;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LifeCell {
    Alive,
    Dead,
}
pub struct GridDrawSettings {
    pub dead_cell_color: Color32,
    pub alive_cell_color: Color32,
    pub cell_size: usize,
    pub stroke_color: Color32,
}
impl Default for GridDrawSettings {
    fn default() -> Self {
        Self {
            dead_cell_color: Color32::BLACK,
            alive_cell_color: Color32::WHITE,
            cell_size: 10,
            stroke_color: Color32::GRAY,
        }
    }
}

pub struct NeededArea {
    width: f32,
    height: f32,
}

impl NeededArea {
    pub fn new(width: f32, height: f32) -> Self {
        assert!(width > 0., "width be greater than 0 !");
        assert!(height > 0., "height be greater than 0 !");
        Self { width, height }
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}
