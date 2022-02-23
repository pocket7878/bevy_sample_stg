#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    InGame,
    GameOver,
    Ending,
}
