use eframe::epaint::Color32;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LifeCell {
    Alive,
    Dead,
}
#[derive(Clone)]
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
