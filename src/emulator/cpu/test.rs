#[cfg(test)]
use super::*;

#[test]
fn test_sub_xy() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 19;
    cpu.registers[1] = 7;
    cpu.sub_xy(0, 1);
    assert_eq!(cpu.registers[0], 12);
    assert_eq!(cpu.registers[0xF], 0);

    cpu.registers[0] = 0;
    cpu.registers[1] = 1;
    cpu.sub_xy(0, 1);
    assert_eq!(cpu.registers[0], 255);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn test_add_xy() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 6;
    cpu.add_xy(0, 1);
    assert_eq!(cpu.registers[0], 11);
    assert_eq!(cpu.registers[0xF], 0);

    cpu.registers[0] = 255;
    cpu.registers[1] = 255;
    cpu.add_xy(0, 1);
    assert_eq!(cpu.registers[0], 254);
    assert_eq!(cpu.registers[0xF], 1);
}
