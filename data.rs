use ink::codegen::{EmitEvent, Env};
use ink_lang as ink;
use ink_prelude::format;
use ink_prelude::string::String;

use crate::{
    blbc::{Blbc, StructCreated},
    model::data::{Inner, Outer},
};

pub fn create_inner(ctx: &mut Blbc, inner: Inner, event_id: Option<String>) -> Result<(), String> {
    let id = inner.id.clone();
    ctx.res_inner_map.insert(id.clone(), inner);

    if let Some(event_id) = event_id {
        ctx.env().emit_event(StructCreated {
            event_id,
            struct_id: id.clone(),
        });
    }

    return Ok(());
}

pub fn create_outer(ctx: &mut Blbc, outer: Outer, event_id: Option<String>) -> Result<(), String> {
    let id = outer.id.clone();
    ctx.res_outer_map.insert(id.clone(), outer);

    if let Some(event_id) = event_id {
        ctx.env().emit_event(StructCreated {
            event_id,
            struct_id: id.clone(),
        });
    }

    return Ok(());
}
