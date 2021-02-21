use crate::script_engine::ScriptEngine;
use crate::world::block::{BlockRegistry, BlockDataInner};
use mlua::{LuaSerdeExt};

const TERRAIN_TABLE: &'static str = "terrain";

/// Adds the block api
///
/// # Arguments
///
/// * `engine` - A scripting engine instance
/// * `block_registry` - A block registry
pub fn add_block_api(engine: &ScriptEngine, registry: &BlockRegistry) {
    let reg = registry.clone();
    let table = engine.add_table(TERRAIN_TABLE).unwrap();
    let _ = engine.add_method_mut(table, "addBlockType", move |lua, block_data: mlua::Value| -> mlua::Result<()> {
        let data: BlockDataInner = lua.from_value(block_data).unwrap();
        reg.register_data(data.into());
        Ok(())
    }).unwrap();
}