use crate::{ir, Type};

use anyhow::*;

pub struct Input {
    pub variable: u32,
    pub name: String,
    pub type_: Type,
}

impl Input {
    pub(crate) fn decode(input: ir::Input) -> Result<Self> {
        Ok(Self {
            variable: input.variable,
            name: input.name,
            type_: Type::decode(input.r#type.ok_or_else(|| anyhow!("missing type for input"))?)?,
        })
    }

    pub(crate) fn encode(&self) -> ir::Input {
        ir::Input {
            variable: self.variable,
            name: self.name.clone(),
            r#type: Some(self.type_.encode()),
        }
    }
}
