use bitvec::prelude::*;
use rand::{rngs::OsRng, RngCore};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub enum PresentKey {
    Key80(Present80bitKey),
    Key128(Present128bitKey),
}

pub struct Present80bitKey {
    pub data: BitArray<[u8; 10]>,
}

impl Present80bitKey {
    /// Generate a new 80 bit key from OS's secure RNG.
    pub fn new() -> Present80bitKey {
        let mut arr = BitArray::<[u8; 10]>::new([0; 10]);
        OsRng.fill_bytes(arr.as_raw_mut_slice());
        Present80bitKey { data: arr }
    }

    /// Generate a new 80 bit key from a bit array.
    pub fn from_bytes(bytes: [u8; 10]) -> Present80bitKey {
        Present80bitKey {
            data: BitArray::new(bytes),
        }
    }
}

impl Default for Present80bitKey {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Present128bitKey {
    pub data: BitArray<[u8; 16]>,
}

impl Present128bitKey {
    /// Generate a new 128 bit key from OS's secure RNG.
    pub fn new() -> Present128bitKey {
        let mut arr = BitArray::<[u8; 16]>::new([0; 16]);
        OsRng.fill_bytes(arr.as_raw_mut_slice());
        Present128bitKey { data: arr }
    }

    /// Generate a new 128 bit key from a bit array.
    pub fn from_bytes(bytes: [u8; 16]) -> Present128bitKey {
        Present128bitKey {
            data: BitArray::new(bytes),
        }
    }
}

impl Default for Present128bitKey {
    fn default() -> Self {
        Self::new()
    }
}

/// `sbox` operates on a nibble.
fn sbox(x: u8) -> u8 {
    assert!(x <= 0xF);
    const SUBSTITUTION: [u8; 16] = [
        0xC, 0x5, 0x6, 0xB, 0x9, 0x0, 0xA, 0xD, 0x3, 0xE, 0xF, 0x8, 0x4, 0x7, 0x1, 0x2,
    ];
    SUBSTITUTION[x as usize]
}

/// `sbox_u8` operates on a byte.
fn sbox_u8(x: u8) -> u8 {
    sbox((x & 0xF0) >> 4) | sbox(x & 0x0F)
}

/// Apply the sbox transformation on a given layer of state.
fn s_box_layer(state: &mut BitArray<[u8; 8]>) {
    for chunk in state.chunks_exact_mut(8) {
        let byte = chunk.load();
        chunk.store(sbox_u8(byte));
    }
}

/// Apply permutations on a single round.
fn p_layer(state: &mut BitVec<u64, Msb0>) {
    const PERMUTATION: [u8; 64] = [
        0, 16, 32, 48, 1, 17, 33, 49, 2, 18, 34, 50, 3, 19, 35, 51, 4, 20, 36, 52, 5, 21, 37, 53,
        6, 22, 38, 54, 7, 23, 39, 55, 8, 24, 40, 56, 9, 25, 41, 57, 10, 26, 42, 58, 11, 27, 43, 59,
        12, 28, 44, 60, 13, 29, 45, 61, 14, 30, 46, 62, 15, 31, 47, 63,
    ];
    let original = state.clone();
    for (i, &perm_index) in PERMUTATION.iter().enumerate() {
        state.set(i, original[perm_index as usize]);
    }
}

fn generate_round_keys(key: &mut PresentKey) -> Vec<BitArray<[u8; 8]>> {
    let mut result = Vec::with_capacity(10);
    match key {
        PresentKey::Key80(inside) => {
            for round_counter in 0..10 {
                let mut round_key = BitArray::<[u8; 8]>::new([0; 8]);
                round_key.clone_from_bitslice(&inside.data[0..64]);
                result.push(round_key);
                inside.data.shift_left(61); // rotate by 61 positions to the left
                let tmp: u8 = inside.data[76..79].load();
                inside.data[76..79].store(sbox(tmp)); // feed 79 to 76 through sbox
                let tmp: u8 = inside.data[15..19].load();
                inside.data[15..19].store(round_counter ^ tmp); // store
            }
        }
        PresentKey::Key128(_) => {
            todo!("Fixing key80 first!");
        }
    };
    result
}

// fn add_round_key(state: &mut BitVec::<u64, Msb0>, round_key: BitVec<u64, Msb0>) {
//     let result = BitVec<u64, Msb0>();

//      state.copy_from_bitslice(&state);
//     for i in 0..64 {
//         state.set(i, state.get(i).xor(round_key[i]));
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
