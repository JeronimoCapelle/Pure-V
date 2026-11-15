//! Parsing and label-resolution stage (`&[Token] -> Vec<Instruction>`).

use alloc::{collections::BTreeMap, vec::Vec};

use purev_isa::{Instruction, Register};

use crate::{
    error::{ParseError, Span},
    lexer::{Token, TokenKind},
    mnemonic::{BranchOp, IOp, LoadOp, Mnemonic, ROp, StoreOp, SysOp, UOp},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImmValue<'a> {
    Literal(i128),
    Symbol(&'a str),
}

struct TokenSlice<'a, 't> {
    tokens: &'t [Token<'a>],
    pos: usize,
}

impl<'a, 't> TokenSlice<'a, 't> {
    const fn new(tokens: &'t [Token<'a>]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&'t Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn peek_n(&self, offset: usize) -> Option<&'t Token<'a>> {
        self.tokens.get(self.pos + offset)
    }

    fn next(&mut self) -> Option<&'t Token<'a>> {
        let tok = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(tok)
    }

    fn next_required(&mut self) -> Result<&'t Token<'a>, ParseError> {
        self.next().ok_or(ParseError::UnexpectedEof)
    }

    fn expect_reg(&mut self) -> Result<Register, ParseError> {
        let tok = self.next_required()?;
        match tok.kind {
            TokenKind::Reg(reg) => Ok(reg),
            _ => Err(ParseError::ExpectedRegister(tok.span)),
        }
    }

    fn expect_comma(&mut self) -> Result<(), ParseError> {
        let tok = self.next_required()?;
        match tok.kind {
            TokenKind::Comma => Ok(()),
            _ => Err(ParseError::ExpectedComma(tok.span)),
        }
    }

    fn expect_lparen(&mut self) -> Result<(), ParseError> {
        let tok = self.next_required()?;
        match tok.kind {
            TokenKind::LParen => Ok(()),
            _ => Err(ParseError::ExpectedLParen(tok.span)),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), ParseError> {
        let tok = self.next_required()?;
        match tok.kind {
            TokenKind::RParen => Ok(()),
            _ => Err(ParseError::ExpectedRParen(tok.span)),
        }
    }

    fn expect_imm_value(&mut self) -> Result<(ImmValue<'a>, Span), ParseError> {
        let tok = self.next_required()?;
        match tok.kind {
            TokenKind::Imm(val) => Ok((ImmValue::Literal(val), tok.span)),
            TokenKind::Identifier(sym) => Ok((ImmValue::Symbol(sym), tok.span)),
            _ => Err(ParseError::ExpectedImmOrLabel(tok.span)),
        }
    }
}

/// Parses a slice of tokens into resolved [`Instruction`] nodes.
pub fn parse(tokens: &[Token<'_>]) -> Result<Vec<Instruction>, ParseError> {
    let labels = collect_labels(tokens)?;
    let mut stream = TokenSlice::new(tokens);
    let mut instructions = Vec::new();
    let mut pc_bytes: usize = 0;

    while let Some(tok) = stream.next() {
        match tok.kind {
            TokenKind::Newline => {},
            TokenKind::Identifier(_) if matches!(stream.peek(), Some(t) if t.kind == TokenKind::Colon) =>
            {
                let _ = stream.next();
            },
            TokenKind::Mnemonic(mne) => {
                let instr = parse_instruction(&mut stream, mne, pc_bytes, &labels)?;
                instructions.push(instr);
                pc_bytes += 4;

                if let Some(next) = stream.peek() {
                    if next.kind == TokenKind::Newline {
                        let _ = stream.next();
                    } else {
                        return Err(ParseError::ExpectedStatementEnd(next.span));
                    }
                }
            },
            _ => return Err(ParseError::ExpectedStatementEnd(tok.span)),
        }
    }

    Ok(instructions)
}

fn collect_labels<'a>(tokens: &[Token<'a>]) -> Result<BTreeMap<&'a str, usize>, ParseError> {
    let mut labels = BTreeMap::new();
    let mut idx = 0;
    let mut pc_bytes: usize = 0;

    while let Some(tok) = tokens.get(idx) {
        match tok.kind {
            TokenKind::Identifier(name)
                if tokens
                    .get(idx + 1)
                    .is_some_and(|next| next.kind == TokenKind::Colon) =>
            {
                if labels.insert(name, pc_bytes).is_some() {
                    return Err(ParseError::DuplicateLabel(tok.span));
                }
                idx += 2;
            },
            TokenKind::Mnemonic(_) => {
                pc_bytes += 4;
                idx += 1;
                while let Some(next) = tokens.get(idx) {
                    if next.kind == TokenKind::Newline {
                        break;
                    }
                    idx += 1;
                }
            },
            _ => idx += 1,
        }
    }

    Ok(labels)
}

fn parse_r_args(
    stream: &mut TokenSlice<'_, '_>,
) -> Result<(Register, Register, Register), ParseError> {
    let rd = stream.expect_reg()?;
    stream.expect_comma()?;
    let rs1 = stream.expect_reg()?;
    stream.expect_comma()?;
    let rs2 = stream.expect_reg()?;
    Ok((rd, rs1, rs2))
}

fn parse_i_args<'a>(
    stream: &mut TokenSlice<'a, '_>,
) -> Result<(Register, Register, ImmValue<'a>, Span), ParseError> {
    let rd = stream.expect_reg()?;
    stream.expect_comma()?;
    let rs1 = stream.expect_reg()?;
    stream.expect_comma()?;
    let (imm, span) = stream.expect_imm_value()?;
    Ok((rd, rs1, imm, span))
}

fn parse_mem_args<'a>(
    stream: &mut TokenSlice<'a, '_>,
) -> Result<(Register, ImmValue<'a>, Span, Register), ParseError> {
    let reg = stream.expect_reg()?;
    stream.expect_comma()?;

    let (offset, span) = if matches!(stream.peek(), Some(t) if t.kind == TokenKind::LParen) {
        (ImmValue::Literal(0), Span::default())
    } else {
        stream.expect_imm_value()?
    };

    stream.expect_lparen()?;
    let base = stream.expect_reg()?;
    stream.expect_rparen()?;

    Ok((reg, offset, span, base))
}

fn parse_jalr_args<'a>(
    stream: &mut TokenSlice<'a, '_>,
) -> Result<(Register, ImmValue<'a>, Span, Register), ParseError> {
    let rd = stream.expect_reg()?;
    stream.expect_comma()?;

    // Support both `jalr rd, rs1, offset` and `jalr rd, offset(rs1)` (plus
    // `jalr rd, (rs1)`).
    if matches!(stream.peek(), Some(t) if matches!(t.kind, TokenKind::Reg(_)))
        && matches!(stream.peek_n(1), Some(t) if t.kind == TokenKind::Comma)
    {
        let base = stream.expect_reg()?;
        stream.expect_comma()?;
        let (offset, span) = stream.expect_imm_value()?;
        return Ok((rd, offset, span, base));
    }

    let (offset, span) = if matches!(stream.peek(), Some(t) if t.kind == TokenKind::LParen) {
        (ImmValue::Literal(0), Span::default())
    } else {
        stream.expect_imm_value()?
    };

    stream.expect_lparen()?;
    let base = stream.expect_reg()?;
    stream.expect_rparen()?;

    Ok((rd, offset, span, base))
}

fn parse_branch_args<'a>(
    stream: &mut TokenSlice<'a, '_>,
) -> Result<(Register, Register, ImmValue<'a>, Span), ParseError> {
    let rs1 = stream.expect_reg()?;
    stream.expect_comma()?;
    let rs2 = stream.expect_reg()?;
    stream.expect_comma()?;
    let (target, span) = stream.expect_imm_value()?;
    Ok((rs1, rs2, target, span))
}

fn parse_rd_imm_args<'a>(
    stream: &mut TokenSlice<'a, '_>,
) -> Result<(Register, ImmValue<'a>, Span), ParseError> {
    let rd = stream.expect_reg()?;
    stream.expect_comma()?;
    let (imm, span) = stream.expect_imm_value()?;
    Ok((rd, imm, span))
}

fn parse_fence_bits(word: &str, span: Span) -> Result<u8, ParseError> {
    if word.is_empty() {
        return Err(ParseError::InvalidFenceOperand(span));
    }
    let mut bits: u8 = 0;
    for ch in word.chars() {
        match ch {
            'i' | 'I' => bits |= 0b1000,
            'o' | 'O' => bits |= 0b0100,
            'r' | 'R' => bits |= 0b0010,
            'w' | 'W' => bits |= 0b0001,
            _ => return Err(ParseError::InvalidFenceOperand(span)),
        }
    }
    Ok(bits)
}

fn parse_fence_args(stream: &mut TokenSlice<'_, '_>) -> Result<(u8, u8), ParseError> {
    if stream.peek().is_none() || matches!(stream.peek(), Some(t) if t.kind == TokenKind::Newline) {
        return Ok((0xF, 0xF));
    }

    let paren = matches!(stream.peek(), Some(t) if t.kind == TokenKind::LParen);
    if paren {
        stream.expect_lparen()?;
    }

    let pred_tok = stream.next_required()?;
    let pred = match pred_tok.kind {
        TokenKind::Identifier(word) => parse_fence_bits(word, pred_tok.span)?,
        _ => return Err(ParseError::InvalidFenceOperand(pred_tok.span)),
    };

    stream.expect_comma()?;

    let succ_tok = stream.next_required()?;
    let succ = match succ_tok.kind {
        TokenKind::Identifier(word) => parse_fence_bits(word, succ_tok.span)?,
        _ => return Err(ParseError::InvalidFenceOperand(succ_tok.span)),
    };

    if paren {
        stream.expect_rparen()?;
    }

    Ok((pred, succ))
}

fn resolve_literal_i12(imm: ImmValue<'_>, span: Span) -> Result<i64, ParseError> {
    let ImmValue::Literal(val) = imm else {
        return Err(ParseError::UndefinedLabel(span));
    };
    if !(-2048..=2047).contains(&val) {
        return Err(ParseError::ImmediateOutOfRange(span));
    }
    i64::try_from(val).map_err(|_| ParseError::ImmediateOutOfRange(span))
}

fn resolve_shamt(imm: ImmValue<'_>, max: i128, span: Span) -> Result<u8, ParseError> {
    let ImmValue::Literal(val) = imm else {
        return Err(ParseError::UndefinedLabel(span));
    };
    if !(0..=max).contains(&val) {
        return Err(ParseError::ImmediateOutOfRange(span));
    }
    u8::try_from(val).map_err(|_| ParseError::ImmediateOutOfRange(span))
}

fn resolve_u20(imm: ImmValue<'_>, span: Span) -> Result<i64, ParseError> {
    let ImmValue::Literal(val) = imm else {
        return Err(ParseError::UndefinedLabel(span));
    };
    if !(-524_288..=1_048_575).contains(&val) {
        return Err(ParseError::ImmediateOutOfRange(span));
    }
    #[allow(
        clippy::cast_possible_truncation,
        reason = "range-checked to 20 bits (-524_288..=1_048_575)"
    )]
    let imm20 = (val as i32) & 0x000F_FFFF;
    Ok(i64::from(imm20 << 12))
}

fn resolve_pc_rel(
    imm: ImmValue<'_>,
    span: Span,
    pc_bytes: usize,
    labels: &BTreeMap<&str, usize>,
    min: i128,
    max: i128,
) -> Result<i64, ParseError> {
    let offset = match imm {
        ImmValue::Literal(val) => val,
        ImmValue::Symbol(name) => {
            let target = labels
                .get(name)
                .copied()
                .ok_or(ParseError::UndefinedLabel(span))?;
            let target_i =
                i128::try_from(target).map_err(|_| ParseError::ImmediateOutOfRange(span))?;
            let pc_i =
                i128::try_from(pc_bytes).map_err(|_| ParseError::ImmediateOutOfRange(span))?;
            target_i - pc_i
        },
    };

    if offset % 2 != 0 || !(min..=max).contains(&offset) {
        return Err(ParseError::ImmediateOutOfRange(span));
    }

    i64::try_from(offset).map_err(|_| ParseError::ImmediateOutOfRange(span))
}

fn parse_instruction<'a>(
    stream: &mut TokenSlice<'a, '_>,
    mne: Mnemonic,
    pc_bytes: usize,
    labels: &BTreeMap<&'a str, usize>,
) -> Result<Instruction, ParseError> {
    let instr = match mne {
        Mnemonic::R(op) => {
            let (rd, rs1, rs2) = parse_r_args(stream)?;
            match op {
                ROp::Add => Instruction::Add { rd, rs1, rs2 },
                ROp::Sub => Instruction::Sub { rd, rs1, rs2 },
                ROp::Sll => Instruction::Sll { rd, rs1, rs2 },
                ROp::Slt => Instruction::Slt { rd, rs1, rs2 },
                ROp::Sltu => Instruction::Sltu { rd, rs1, rs2 },
                ROp::Xor => Instruction::Xor { rd, rs1, rs2 },
                ROp::Srl => Instruction::Srl { rd, rs1, rs2 },
                ROp::Sra => Instruction::Sra { rd, rs1, rs2 },
                ROp::Or => Instruction::Or { rd, rs1, rs2 },
                ROp::And => Instruction::And { rd, rs1, rs2 },
                ROp::Addw => Instruction::Addw { rd, rs1, rs2 },
                ROp::Subw => Instruction::Subw { rd, rs1, rs2 },
                ROp::Sllw => Instruction::Sllw { rd, rs1, rs2 },
                ROp::Srlw => Instruction::Srlw { rd, rs1, rs2 },
                ROp::Sraw => Instruction::Sraw { rd, rs1, rs2 },
            }
        },
        Mnemonic::I(op) => {
            let (rd, rs1, imm, span) = parse_i_args(stream)?;
            match op {
                IOp::Addi => Instruction::Addi {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Slti => Instruction::Slti {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Sltiu => Instruction::Sltiu {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Xori => Instruction::Xori {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Ori => Instruction::Ori {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Andi => Instruction::Andi {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Slli => Instruction::Slli {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 63, span)?,
                },
                IOp::Srli => Instruction::Srli {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 63, span)?,
                },
                IOp::Srai => Instruction::Srai {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 63, span)?,
                },
                IOp::Addiw => Instruction::Addiw {
                    rd,
                    rs1,
                    imm: resolve_literal_i12(imm, span)?,
                },
                IOp::Slliw => Instruction::Slliw {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 31, span)?,
                },
                IOp::Srliw => Instruction::Srliw {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 31, span)?,
                },
                IOp::Sraiw => Instruction::Sraiw {
                    rd,
                    rs1,
                    shamt: resolve_shamt(imm, 31, span)?,
                },
            }
        },
        Mnemonic::Load(op) => {
            let (rd, offset, span, rs1) = parse_mem_args(stream)?;
            let imm = resolve_literal_i12(offset, span)?;
            match op {
                LoadOp::Lb => Instruction::Lb { rd, rs1, imm },
                LoadOp::Lh => Instruction::Lh { rd, rs1, imm },
                LoadOp::Lw => Instruction::Lw { rd, rs1, imm },
                LoadOp::Ld => Instruction::Ld { rd, rs1, imm },
                LoadOp::Lbu => Instruction::Lbu { rd, rs1, imm },
                LoadOp::Lhu => Instruction::Lhu { rd, rs1, imm },
                LoadOp::Lwu => Instruction::Lwu { rd, rs1, imm },
            }
        },
        Mnemonic::Store(op) => {
            let (rs2, offset, span, rs1) = parse_mem_args(stream)?;
            let imm = resolve_literal_i12(offset, span)?;
            match op {
                StoreOp::Sb => Instruction::Sb { rs1, rs2, imm },
                StoreOp::Sh => Instruction::Sh { rs1, rs2, imm },
                StoreOp::Sw => Instruction::Sw { rs1, rs2, imm },
                StoreOp::Sd => Instruction::Sd { rs1, rs2, imm },
            }
        },
        Mnemonic::Branch(op) => {
            let (rs1, rs2, target, span) = parse_branch_args(stream)?;
            let imm = resolve_pc_rel(target, span, pc_bytes, labels, -4096, 4094)?;
            match op {
                BranchOp::Beq => Instruction::Beq { rs1, rs2, imm },
                BranchOp::Bne => Instruction::Bne { rs1, rs2, imm },
                BranchOp::Blt => Instruction::Blt { rs1, rs2, imm },
                BranchOp::Bge => Instruction::Bge { rs1, rs2, imm },
                BranchOp::Bltu => Instruction::Bltu { rs1, rs2, imm },
                BranchOp::Bgeu => Instruction::Bgeu { rs1, rs2, imm },
            }
        },
        Mnemonic::U(op) => {
            let (rd, imm_val, span) = parse_rd_imm_args(stream)?;
            let imm = resolve_u20(imm_val, span)?;
            match op {
                UOp::Lui => Instruction::Lui { rd, imm },
                UOp::Auipc => Instruction::Auipc { rd, imm },
            }
        },
        Mnemonic::Jal => {
            let (rd, target, span) = parse_rd_imm_args(stream)?;
            let imm = resolve_pc_rel(target, span, pc_bytes, labels, -1_048_576, 1_048_574)?;
            Instruction::Jal { rd, imm }
        },
        Mnemonic::Jalr => {
            let (rd, offset, span, rs1) = parse_jalr_args(stream)?;
            let imm = resolve_literal_i12(offset, span)?;
            Instruction::Jalr { rd, rs1, imm }
        },
        Mnemonic::Fence => {
            let (pred, succ) = parse_fence_args(stream)?;
            Instruction::Fence { pred, succ }
        },
        Mnemonic::Sys(op) => match op {
            SysOp::Ecall => Instruction::Ecall,
            SysOp::Ebreak => Instruction::Ebreak,
        },
    };

    Ok(instr)
}
