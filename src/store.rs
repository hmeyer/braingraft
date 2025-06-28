use crate::MAX_BITS;
use crate::name::compile_name;
use anyhow::{Result, bail};
use llvm_ir::{Constant, Operand, Type, instruction::Store};

pub fn compile_store(store: &Store) -> Result<String> {
    let mut output = String::new();
    let address = match &store.address {
        Operand::LocalOperand { name, ty } => {
            if !matches!(*ty.as_ref(), Type::PointerType { .. }) {
                bail!("Load address must be a pointer type, found: {}", ty);
            }
            eprintln!("Local operand in load: {} of type {}", name, ty);
            compile_name(name)
        }
        _ => bail!("Unsupported operand type in load: {}", store.address),
    };
    let value = match &store.value {
        Operand::LocalOperand { name, ty } => {
            compile_name(name)
        },
        Operand::ConstantOperand(cref) => match cref.as_ref() {
            Constant::GlobalReference { name, ty } => {
                eprintln!("Global reference in store value: {} of type {}", name, ty);
                compile_name(name)
            }
            Constant::Int { bits, value, .. } => {
                if *bits > MAX_BITS && *value > (1 << MAX_BITS) - 1 {
                    eprintln!(
                        "Integer constant with more than {} bits is not supported: {} ({} bits)",
                        MAX_BITS, value, bits
                    );
                }
                format!("{}", value)
            }
            _ => {
                eprintln!("Unsupported constant operand in store: {}", cref);
                String::new()
            }
        },
        Operand::MetadataOperand => {
            bail!("Metadata operands are not supported in stores.");
        }
    };

    output.push_str(&format!(
        "LINMEM[{}] = {};",
        address, value
    ));

    Ok(output)
}
