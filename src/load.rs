use crate::MAX_BITS;
use crate::name::compile_name;
use anyhow::{Result, bail};
use llvm_ir::{Constant, Operand, Type, instruction::Load};

pub fn compile_load(load: &Load) -> Result<String> {
    let mut output = String::new();
    let address = match &load.address {
        Operand::LocalOperand { name, ty } => {
            if !matches!(*ty.as_ref(), Type::PointerType { .. }) {
                bail!("Load address must be a pointer type, found: {}", ty);
            }
            eprintln!("Local operand in load: {} of type {}", name, ty);
            compile_name(name)
        }
        Operand::ConstantOperand(cref) => match cref.as_ref() {
            Constant::GlobalReference { name, ty } => {
                eprintln!("Global reference in load: {} of type {}", name, ty);
                compile_name(name)
            }
            _ => {
                eprintln!("Unsupported constant operand in load: {}", cref);
                String::new()
            }
        },
        _ => bail!("Unsupported operand type in load: {}", load.address),
    };

    output.push_str(&format!(
        "{} = LINMEM[{}];",
        compile_name(&load.dest),
        address
    ));

    Ok(output)
}
