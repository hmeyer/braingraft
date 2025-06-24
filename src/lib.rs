use anyhow::{Context, Result, anyhow};
use llvm_ir::Module;

mod alloca;
mod block;
mod call;
mod function;
mod name;
use function::compile_function;

struct CompiledFunction {
    name: String,
    assembly: String,
}

pub fn compile(llvm_ir: &str) -> Result<String> {
    let module = Module::from_ir_str(llvm_ir)
        .map_err(|e| anyhow!(e))
        .context("Failed to parse LLVM-IR")?;

    let mut compiled_output = String::new();
    for function in &module.functions {
        let f = CompiledFunction {
            name: function.name.clone(),
            assembly: compile_function(function)?,
        };
        compiled_output.push_str(&format!(
            "Function: {}\nAssembly:\n{}\n",
            f.name, f.assembly
        ));
    }

    Ok(compiled_output)
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
