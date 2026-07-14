use crate::structures::TrackedError;

mod code_generation;
mod convert_bytes;
mod lexical_analysis;
mod structures;
mod symbol_resolution;
mod syntax_analysis;

pub fn compile_string(input: &str) -> Result<Vec<u8>, TrackedError> {
    let tokens = lexical_analysis::tokenize(input)?;
    let (labels, stripped_tokens) = symbol_resolution::collect_symbols(&tokens)?;
    let statements = syntax_analysis::parse(&stripped_tokens, &labels)?;
    let packed = code_generation::assemble(statements);
    convert_bytes::transform(packed)
}
