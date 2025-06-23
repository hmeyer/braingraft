use anyhow::{bail, Result};
use llvm_ir::{instruction::Alloca, Constant, Name, Operand, Type};

fn compile_name(name: &Name) -> String {
    match name {
        Name::Name(n) => format!("var_{}", n),
        Name::Number(n) => format!("anonymous_var_{}", n),
    }
}

pub fn compile_alloca(alloca: &Alloca) -> Result<String> {
    let mut output = String::new();
    let maybe_array_size = match alloca.allocated_type.as_ref() {
        Type::IntegerType { bits, .. } => {
            if *bits > 32 {
                bail!(
                    "Alloca with integer type larger than 32 bits is not supported: {}",
                    bits
                );
            }
            None
        }
        Type::PointerType { addr_space: _ } => None,
        Type::ArrayType {
            element_type,
            num_elements,
        } => {
            if let Type::IntegerType { bits, .. } = element_type.as_ref() {
                if *bits > 32 {
                    bail!(
                        "Alloca with array element type larger than 32 bits is not supported: {}",
                        bits
                    );
                }
            } else {
                bail!(
                    "Alloca with unsupported array element type: {}",
                    element_type
                );
            }
            Some(num_elements)
        }
        _ => bail!("Alloca with unsupported type: {}", alloca.allocated_type),
    };
    let num_elements = match &alloca.num_elements {
        Operand::LocalOperand { name, ty } => {
            bail!("Alloca with local operand name: {} type: {}", name, ty)
        }
        Operand::ConstantOperand(cref) => {
            let c = cref.as_ref();
            match c {
                Constant::Int { bits, value, .. } => {
                    if *bits >= 64 {
                        *value as usize
                    } else {
                        let mask = (1 << bits) - 1;
                        (*value & mask) as usize
                    }
                }
                _ => bail!("Unsupported constant type for alloca: {}", c),
            }
        }
        Operand::MetadataOperand => bail!("Alloca with metadata operand"),
    };
    if num_elements != 1 {
        bail!(
            "Alloca with multiple elements is not supported: {}",
            num_elements
        );
    }
    output.push_str(&format!("var {}", compile_name(&alloca.dest)));
    if let Some(array_size) = maybe_array_size {
        output.push_str(&format!("[{}]", array_size));
    }
    output.push_str(";\n");
    Ok(output)
}
