use crate::parser::Node;

use self::{
    collecting::CollectedModule,
    error::{Error, Result},
};

pub mod collecting;
pub mod error;

pub struct CompileTask {
    pub state: State,
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            state: State::LayoutCollecting {
                modules: Vec::with_capacity(0),
            },
        }
    }
}

impl CompileTask {
    pub fn include(&mut self, BareModule { name, root }: BareModule) -> Result<()> {
        let State::LayoutCollecting { modules } = &mut self.state else {
            return Err(Error::InvalidOperation);
        };
        Ok(())
    }
}

pub enum State {
    LayoutCollecting { modules: Vec<CollectedModule> },
    IntermediaryGeneration {},
    DependencyFiltering {},
    BinaryAssembling {},
}

pub struct BareModule {
    pub name: String,
    pub root: Node,
}
