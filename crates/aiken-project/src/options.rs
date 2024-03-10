use aiken_lang::ast::Tracing;

pub struct Options {
    pub code_gen_mode: CodeGenMode,
    pub tracing: Tracing,
}

pub enum CodeGenMode {
    Test {
        match_tests: Option<Vec<String>>,
        verbose: bool,
        exact_match: bool,
        seed: u32,
        property_max_success: usize,
    },
    Build(bool),
    NoOp,
}
