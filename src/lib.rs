use anyhow::{anyhow, Context, Result};
use llvm_ir::Module;


pub fn compile(llvm_ir: &str) -> Result<String> {
    let module = Module::from_ir_str(llvm_ir)
        .map_err(|e| anyhow!(e))
        .context("Failed to parse LLVM-IR")?;

    for function in &module.functions {
        println!("Found function: {}", function.name);
        println!("  Return type: {:?}", function.return_type);
        if function.parameters.is_empty() {
            println!("  Parameters: None");
        } else {
            println!("  Parameters:");
            for param in &function.parameters {
                println!("    - {}: {:?}", param.name, param.ty);
            }
        }
    }

    Ok(String::new())
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

