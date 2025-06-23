use anyhow::Result;
use llvm_ir::Function;
use std::collections::HashSet;

use crate::block::compile_block;

pub fn compile_function(function: &Function) -> Result<String> {
    let mut function_assembly = String::new();

    if function.basic_blocks.is_empty() {
        return Ok(function_assembly);
    }

    let mut visited_blocks = HashSet::new();
    compile_block(
        function,
        &function.basic_blocks[0].name,
        &mut function_assembly,
        &mut visited_blocks,
    )?;

    Ok(function_assembly)
}
