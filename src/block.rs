use crate::alloca::compile_alloca;
use crate::call::compile_call;
use crate::load::compile_load;
use anyhow::{Result, anyhow};
use llvm_ir::{Function, Instruction, Name, Terminator};
use std::collections::HashSet;

pub fn compile_block(
    function: &Function,
    block_name: &Name,
    output: &mut String,
    visited: &mut HashSet<Name>,
) -> Result<()> {
    if !visited.insert(block_name.clone()) {
        return Ok(());
    }

    let block = function
        .basic_blocks
        .iter()
        .find(|b| &b.name == block_name)
        .ok_or_else(|| anyhow!("Failed to find block: {}", block_name))?;

    for instruction in &block.instrs {
        match instruction {
            Instruction::Call(call) => {
                output.push_str(&compile_call(call)?);
            }
            Instruction::Alloca(a) => {
                output.push_str(&compile_alloca(a)?);
            }
            Instruction::Load(load) => {
                output.push_str(&compile_load(load)?);
            }

            instr => {
                let formatted = format!("{:?}", instr);
                let type_name = formatted.split('(').next().unwrap_or("").trim();
                eprintln!("Unhandled instruction type: {}", type_name);
            }
        }
    }

    match &block.term {
        Terminator::Ret(_ret) => {
            output.push_str("  ret\n");
        }
        Terminator::Br(br) => {
            compile_block(function, &br.dest, output, visited)?;
        }
        Terminator::CondBr(cond_br) => {
            eprintln!("Unhandled condition: {:?}", cond_br.condition);
            compile_block(function, &cond_br.true_dest, output, visited)?;
            compile_block(function, &cond_br.false_dest, output, visited)?;
        }
        term => {
            let formatted = format!("{:?}", term);
            let type_name = formatted.split('(').next().unwrap_or("").trim();
            eprintln!("Unhandled terminator type: {}", type_name);
        }
    }

    Ok(())
}
