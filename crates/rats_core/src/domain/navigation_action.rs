#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    Previous,
    Next,
    Left,
    Right,
    PageUp,
    PageDown,
    Top,
    Bottom,
    Help,
}
