use anyhow::Result;
use either::Either;
use llvm_ir::{instruction::Call, Constant, Operand};

pub fn compile_call(call: &Call) -> Result<String> {
    let mut output = String::new();
    match &call.function {
        Either::Right(Operand::ConstantOperand(cref)) => match cref.as_ref() {
            Constant::GlobalReference { name, .. } => {
                output.push_str(&format!("  call @{}\n", name));
            }
            operand => {
                let formatted = format!("{:?}", operand);
                let type_name = formatted.split('(').next().unwrap_or("").trim();
                eprintln!("Unhandled constant operand type: {}", type_name);
            }
        },
        operand => {
            let formatted = format!("{:?}", operand);
            let type_name = formatted.split('(').next().unwrap_or("").trim();
            eprintln!("Unhandled call function type: {}", type_name);
        }
    }
    Ok(output)
}
