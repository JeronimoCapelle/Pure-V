//! End-to-end round-trip tests between `purev-asm` and `purev-core`.

use purev_asm::{assemble, lexer, parser};
use purev_core::{Machine, bus::Memory, cpu::decode::decode};
use purev_isa::{DRAM_BASE, IsaConfig, Register, Trap};

#[test]
fn assemble_decode_roundtrip_all_rv64i() {
    let source = "
        start:
            lui a0, 0x12345
            auipc a1, 16
            addi a2, a0, -42
            slti a3, a2, 100
            sltiu a4, a2, 100
            xori a5, a2, 0xFF
            ori a6, a2, 0x0F
            andi a7, a2, 0x55
            slli t0, a2, 35
            srli t1, t0, 7
            srai t2, t0, 12
            addiw t3, a2, -1
            slliw t4, t3, 31
            srliw t5, t4, 4
            sraiw t6, t4, 8
            add s0, a0, a1
            sub s1, s0, a2
            sll s2, s1, t0
            slt s3, s1, s0
            sltu s4, s1, s0
            xor s5, s1, s2
            srl s6, s5, t1
            sra s7, s5, t2
            or s8, s5, s6
            and s9, s5, s7
            addw s10, s8, s9
            subw s11, s10, s9
            sllw a0, s10, t4
            srlw a1, s10, t5
            sraw a2, s10, t6
            lb a0, -8(sp)
            lh a1, -4(sp)
            lw a2, 0(sp)
            ld a3, 8(sp)
            lbu a4, 16(sp)
            lhu a5, 24(sp)
            lwu a6, 32(sp)
            sb a0, -8(sp)
            sh a1, -4(sp)
            sw a2, 0(sp)
            sd a3, 8(sp)
            beq a0, a1, start
            bne a0, a1, end
            blt a0, a1, start
            bge a0, a1, end
            bltu a0, a1, start
            bgeu a0, a1, end
            jal ra, start
            jalr ra, 16(sp)
            fence (iw, rw)
            ecall
        end:
            ebreak
    ";

    let tokens = lexer::tokenize(source).unwrap();
    let expected = parser::parse(&tokens).unwrap();
    let words = assemble(source).unwrap();
    assert_eq!(words.len(), expected.len());

    for (word, orig) in words.into_iter().zip(expected) {
        let (decoded, len) = decode(word, IsaConfig::RV64I).unwrap();
        assert_eq!(len, 4);
        assert_eq!(decoded, orig);
    }
}

#[test]
fn machine_executes_assembled_sum_loop() {
    let source = "
        _start:
            addi a0, zero, 0
            addi t0, zero, 1
            addi t1, zero, 6
        loop:
            beq t0, t1, done
            add a0, a0, t0
            addi t0, t0, 1
            jal zero, loop
        done:
            ecall
    ";

    let words = assemble(source).unwrap();
    let mut machine = Machine::new(IsaConfig::RV64I);
    for (idx, word) in words.into_iter().enumerate() {
        let off = u64::try_from(idx * 4).unwrap();
        machine.bus.write32(DRAM_BASE + off, word).unwrap();
    }

    let trap = machine.run().unwrap_err();
    assert_eq!(trap, Trap::EnvironmentCallFromMMode);
    assert_eq!(machine.cpu.regs.read(Register::A0), 1 + 2 + 3 + 4 + 5);
}
