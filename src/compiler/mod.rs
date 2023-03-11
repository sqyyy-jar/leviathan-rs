pub mod collecting;

pub struct CompileTask {
    pub state: State,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            state: State::LayoutCollecting {},
        }
    }
}

pub enum State {
    LayoutCollecting {},
    IntermediaryGeneration {},
    DependencyFiltering {},
    BinaryAssembling {},
}
