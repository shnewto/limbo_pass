#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Fsm {
    Loading,
    MainMenu,
    Running,
}
