use crate::alloca::compile_alloca;
use anyhow::{anyhow, Result};
use either::Either;
use llvm_ir::{Constant, Function, Instruction, Name, Operand, Terminator};
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
            Instruction::Call(call) => match &call.function {
                Either::Right(Operand::ConstantOperand(cref)) => match cref.as_ref() {
                    Constant::GlobalReference { name, .. } => {
                        output.push_str(&format!("  call @{}\n", name));
                    }
                    operand => {
                        let formatted = format!("{:?}", operand);
                        let type_name = formatted.split('(').next().unwrap_or("").trim();
                        println!("Unhandled constant operand type: {}", type_name);
                    }
                },
                operand => {
                    let formatted = format!("{:?}", operand);
                    let type_name = formatted.split('(').next().unwrap_or("").trim();
                    println!("Unhandled call function type: {}", type_name);
                }
            },
            Instruction::Alloca(a) => {
                let alloca_output = compile_alloca(a)?;
                output.push_str(&format!("  {}", alloca_output));
            }

            instr => {
                let formatted = format!("{:?}", instr);
                let type_name = formatted.split('(').next().unwrap_or("").trim();
                println!("Unhandled instruction type: {}", type_name);
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
            println!("Unhandled condition: {:?}", cond_br.condition);
            compile_block(function, &cond_br.true_dest, output, visited)?;
            compile_block(function, &cond_br.false_dest, output, visited)?;
        }
        term => {
            let formatted = format!("{:?}", term);
            let type_name = formatted.split('(').next().unwrap_or("").trim();
            println!("Unhandled terminator type: {}", type_name);
        }
    }

    Ok(())
}
