use super::*;

#[test]
fn test_read_and_write(){
    let mut cpu : CPU = new();
    let test_data : [u8; 2] = [0xde, 0xad];
    cpu.mount_mem(0x1000, &test_data);
    let test_byte1 = cpu.read_mem(0x1000);
    let test_byte2 = cpu.read_mem(0x1001);
    assert_eq!(test_byte1.clone(), 0xde);
    assert_eq!(test_byte2.clone(), 0xad);
}

#[test]
fn test_initialized_memory(){
    let cpu : CPU = new();
    let test_byte1 = cpu.read_mem(0);
    let test_byte2 = cpu.read_mem((rs6502::MAX_MEM - 1) as u16);
    assert_eq!(test_byte1.clone(), 0);
    assert_eq!(test_byte2.clone(), 0);
}