use anyhow::{anyhow, Context, Result};
use llvm_ir::Module;


pub fn compile(llvm_ir: &str) -> Result<String> {
    Module::from_ir_str(llvm_ir)
        .map_err(|e| anyhow!(e))
        .context("Failed to parse LLVM-IR")?;
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

