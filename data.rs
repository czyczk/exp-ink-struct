use ink::codegen::{EmitEvent, Env};
use ink_lang as ink;
use ink_prelude::string::String;

use crate::{
    model::data::{Inner, Outer},
    structcontract::{StructContract, StructCreated},
};

pub fn create_inner(
    ctx: &mut StructContract,
    inner: Inner,
    event_id: Option<String>,
) -> Result<(), String> {
    let id = inner.id.clone();
    ctx.res_inner_map.insert(id.clone(), &inner);

    if let Some(event_id) = event_id {
        ctx.env().emit_event(StructCreated {
            event_id,
            struct_id: id.clone(),
        });
    }

    return Ok(());
}

pub fn create_outer(
    ctx: &mut StructContract,
    outer: Outer,
    event_id: Option<String>,
) -> Result<(), String> {
    let id = outer.id.clone();
    ctx.res_outer_map.insert(id.clone(), &outer);

    if let Some(event_id) = event_id {
        ctx.env().emit_event(StructCreated {
            event_id,
            struct_id: id.clone(),
        });
    }

    return Ok(());
}
