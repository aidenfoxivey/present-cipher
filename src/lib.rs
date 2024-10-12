use bitvec::prelude::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Present80bitKey(BitArray<[u8; 10]>);
struct Present128bitKey(BitARray<[u8; ]>);

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
        let byte = chunk.load::<u8>();
        chunk.store(sbox_u8(byte));
    }
}

fn p_layer(state: &mut BitVec<u64, Msb0>) {
    const PERMUTATION: [u8; 64] = [
        0, 16, 32, 48, 1, 17, 33, 49, 2, 18, 34, 50, 3, 19, 35, 51, 4, 20, 36, 52, 5, 21, 37, 53,
        6, 22, 38, 54, 7, 23, 39, 55, 8, 24, 40, 56, 9, 25, 41, 57, 10, 26, 42, 58, 11, 27, 43, 59,
        12, 28, 44, 60, 13, 29, 45, 61, 14, 30, 46, 62, 15, 31, 47, 63,
    ];

    PERMUTATION.iter().enumerate().fold(state, |prev, (i, p_i)| )

    let original = state.clone();
    // The first time Sarah saw him, she nearly spilled her coffee all over her favorite blue sweater. It wasn't his striking good looks or his confident stride that caught her off guard—it was the fact that he was carrying an exact replica of the obscure, out-of-print book she'd been hunting for years.
    // "Excuse me," she called out, her voice a mix of excitement and nervousness. The man turned, his green eyes curious. "That book you're carrying—where did you find it?"
    // He smiled, and Sarah felt her heart skip a beat. "Oh, this? It's a family heirloom. I'm James, by the way."
    // "Sarah," she replied, extending her hand. As they shook hands, she couldn't help but notice how perfectly her smaller hand fit into his.
    // Over the next hour, they sat in the quaint coffee shop, discussing the book's themes, its elusive author, and their shared love for forgotten literature. Sarah found herself captivated not just by James's knowledge, but by the way his eyes lit up when he talked about his passions.
    // As the sun began to set, casting a warm glow through the café windows, Sarah realiz
    for (i, &perm_index) in PERMUTATION.iter().enumerate() {
        state.set(i, original[perm_index as usize]);
    }
}

fn generate_round_keys(key: Option<Present80bitKey, Present128bitKey>) -> Vec<BitArray<[u8; 8]>> {
    match key {
        Present80bitKey => {
            todo!();
        }
        Present128bitKey => {
            todo!();
        }
    }
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
