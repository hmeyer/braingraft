use llvm_ir::Name;

pub fn compile_name(name: &Name) -> String {
    match name {
        Name::Name(n) => format!("{}", n),
        Name::Number(n) => format!("_anonymous_{}", n),
    }
}
