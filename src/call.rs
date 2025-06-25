use crate::MAX_BITS;
use crate::name::compile_name;
use anyhow::Result;
use either::Either;
use llvm_ir::{
    Constant, Operand,
    function::ParameterAttribute,
    instruction::{Call, InlineAssembly},
    operand,
};

fn compile_argument(operand: &Operand, attrs: &[ParameterAttribute]) -> String {
    if !attrs.is_empty() {
        eprintln!("Parameter attributes are not supported: {:?}", attrs);
    }
    match operand {
        Operand::LocalOperand { name, ty } => compile_name(name),
        Operand::ConstantOperand(cref) => match cref.as_ref() {
            Constant::GlobalReference { name, ty } => compile_name(name),
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
                eprintln!("Unsupported constant operand: {}", cref);
                String::new()
            }
        },
        Operand::MetadataOperand => {
            eprintln!("Metadata operands are not supported in calls.");
            String::new()
        }
    }
}

pub fn compile_call(call: &Call) -> Result<String> {
    let mut output = String::new();
    let (name, signature) = match &call.function {
        Either::Left(InlineAssembly { ty }) => {
            eprintln!("Inline assembly is not supported yet: {}", ty.as_ref());
            return Ok(output);
        }
        Either::Right(operand) => match operand {
            Operand::LocalOperand { name, ty } => (name, ty.as_ref()),
            Operand::ConstantOperand(cref) => match cref.as_ref() {
                Constant::GlobalReference { name, ty } => (name, ty.as_ref()),
                c => {
                    eprintln!("Unhandled constant operand type: {}", c);
                    return Ok(output);
                }
            },
            Operand::MetadataOperand => {
                eprintln!("Metadata operands are not supported in calls.");
                return Ok(output);
            }
        },
    };
    let name = compile_name(name);
    let arguments = call
        .arguments
        .iter()
        .map(|arg| compile_argument(&arg.0, &arg.1))
        .collect::<Vec<_>>()
        .join(", ");

    output.push_str(&format!("{}({});\n", name, arguments));

    Ok(output)
}
