use bitvec::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn sbox_encode(x: u8) -> u8 {
    assert!(x < 0xF);
    const SUBSTITUTION: [u8; 16] = [0xC, 0x5, 0x6, 0xB, 0x9, 0x0, 0xA, 0xD, 0x3, 0xE, 0xF, 0x8, 0x4, 0x7, 0x1, 0x2];
    SUBSTITUTION[x as usize]
}

fn s_box_layer(state: &mut BitVec::<u64, Msb0>) {

}

fn p_layer(state: &mut BitVec::<u16, Msb0>) {

}

fn generate_round_keys() -> Vec<BitVec::<u64, Msb0>> {

}

fn add_round_key(state: &mut BitVec::<u64, Msb0>, round_key: BitVec<u64, Msb0>) {
    let result = BitVec<u64, Msb0>();

     state.copy_from_bitslice(&state);
    for i in 0..64 {
        state.set(i, state.get(i).xor(round_key[i]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
