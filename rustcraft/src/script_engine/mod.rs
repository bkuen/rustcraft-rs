pub mod terrain;

use std::sync::Arc;
use std::ops::Deref;
use mlua::{Lua, Table, FromLuaMulti, ToLuaMulti, Function};
use crate::resources::Resources;

/// ScriptEngine
///
/// The scripting engine loads ``Lua`` scripts
/// from the resource directory and runs the code.
/// As a result, ``block types``, ``biomes``, ``world generation``,
/// etc. can be handled without writing any code - just ``Lua``
/// scripts.
#[derive(Clone)]
pub struct ScriptEngine {
    inner: Arc<ScriptEngineInner>,
}

impl Deref for ScriptEngine {
    type Target = ScriptEngineInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ScriptEngine {
    /// Creates a new scripting engine
    pub fn new() -> Self {
        let lua = Lua::new();
        Self {
            inner: Arc::new(ScriptEngineInner {
                lua,
            })
        }
    }

    /// Adds a table to the globals
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the table
    pub fn add_table(&self, name: &str) -> mlua::Result<Table> {
        let table = self.lua.create_table()?;
        self.lua.globals().set(name, table.clone())?;
        Ok(table)
    }

    /// Adds a method to a table
    ///
    /// # Arguments
    ///
    /// * `table` - A lua table
    /// * `name` - The name of the function
    /// * `f` - A function which should be add to the table
    pub fn add_method<'lua, 'callback, A, R, F>(&'lua self, table: Table, name: &str, f: F) -> mlua::Result<Function>
    where
        'lua: 'callback,
        A: FromLuaMulti<'callback>,
        R: ToLuaMulti<'callback>,
        F: 'static + Fn(&'callback Lua, A) -> mlua::Result<R>,
    {
        let method = self.lua.create_function(f)?;
        table.set(name, method.clone())?;
        Ok(method)
    }

    /// Adds a mutable method to a table
    ///
    /// # Arguments
    ///
    /// * `table` - A lua table
    /// * `name` - The name of the function
    /// * `f` - A function which should be add to the table
    pub fn add_method_mut<'lua, 'callback, A, R, F>(&'lua self, table: Table, name: &str, f: F) -> mlua::Result<Function>
        where
            'lua: 'callback,
            A: FromLuaMulti<'callback>,
            R: ToLuaMulti<'callback>,
            F: 'static + FnMut(&'callback Lua, A) -> mlua::Result<R>,
    {
        let method = self.lua.create_function_mut(f)?;
        table.set(name, method.clone())?;
        Ok(method)
    }

    /// Runs a file from the resources
    pub fn run_file(&self, resources: &Resources, path: &str) {
        let script = resources.load_file_content(path).unwrap();
        let _ = self.lua.load(&script).exec().unwrap();
    }
}

/// ScriptEngineInner
pub struct ScriptEngineInner {
    /// A ``Lua`` instance
    lua: Lua,
}

#[test]
fn test_script_engine() {
    let resources = Resources::from_relative_exe_path(Path::new("res")).unwrap();
    let block_registry = BlockRegistry::default();
    let script_engine  = ScriptEngine::new();
    add_block_api(&script_engine, &block_registry);

    script_engine.run_file(&resources, "scripts/world/blocks.lua");

    for block in block_registry.blocks() {
        println!("BlockData {} {}", block.id(), block.name())
    }
}