#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Fsm {
    LoadingScreen,
    LoadingAssets,
    Setup,
    Running,
}
