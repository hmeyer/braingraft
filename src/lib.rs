use anyhow::{anyhow, Context, Result};
use either::Either;
use llvm_ir::{Constant, Function, Instruction, Module, Operand, Terminator};
use std::collections::HashSet;


pub fn compile(llvm_ir: &str) -> Result<String> {
    let module = Module::from_ir_str(llvm_ir)
        .map_err(|e| anyhow!(e))
        .context("Failed to parse LLVM-IR")?;

    let mut compiled_output = String::new();
    for function in &module.functions {
        compiled_output.push_str(&compile_function(function)?);
    }

    Ok(compiled_output)
}

fn compile_function(function: &Function) -> Result<String> {
    let mut function_assembly = String::new();
    function_assembly.push_str(&format!("{}:\n", function.name));

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

fn compile_block(
    function: &Function,
    block_name: &llvm_ir::Name,
    output: &mut String,
    visited: &mut HashSet<llvm_ir::Name>,
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
                match &call.function {
                    Either::Right(Operand::ConstantOperand(cref)) => {
                        match cref.as_ref() {
                            Constant::GlobalReference { name, .. } => {
                                output.push_str(&format!("  call @{}\n", name));
                            }
                            operand => {
                                let formatted = format!("{:?}", operand);
                                let type_name = formatted.split('(').next().unwrap_or("").trim();
                                println!("Unhandled constant operand type: {}", type_name);
                            }
                        }
                    }
                    operand => {
                        let formatted = format!("{:?}", operand);
                        let type_name = formatted.split('(').next().unwrap_or("").trim();
                        println!("Unhandled call function type: {}", type_name);
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_ir() {
        let result = compile("this is not valid llvm ir");
        assert!(result.is_err());
    }
}

