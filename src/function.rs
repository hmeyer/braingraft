use anyhow::Result;
use llvm_ir::Function;
use std::collections::HashSet;

use crate::block::compile_block;

pub fn compile_function(function: &Function) -> Result<String> {
    if function.basic_blocks.is_empty() {
        return Ok(String::new());
    }

    let mut visited_blocks = HashSet::new();

    let block = compile_block(
        function,
        &function.basic_blocks[0].name,
        &mut visited_blocks,
    )?
    .into_iter()
    .map(|s| format!("    {}", s))
    .collect::<Vec<_>>()
    .join("\n");

    Ok(block)
}
