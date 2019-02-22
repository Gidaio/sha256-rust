use std::env;
use std::fs;


const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let mut initial_hash_values: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19
    ];

    let string_length: u64 = (contents.len() * 8) as u64;
    let mut string_bytes = contents.into_bytes();

    // Figure out how many zeroes are needed.
    let mut zero_bytes = 0;
    while (string_bytes.len() + 1 + zero_bytes + 8) % 64 != 0 {
        zero_bytes += 1;
    }

    string_bytes.push(0b10000000);

    for _ in 0 .. zero_bytes {
        string_bytes.push(0b00000000)
    }

    // Add the length in big-endian.
    string_bytes.push((string_length >> 56) as u8);
    string_bytes.push((string_length >> 48) as u8);
    string_bytes.push((string_length >> 40) as u8);
    string_bytes.push((string_length >> 32) as u8);
    string_bytes.push((string_length >> 24) as u8);
    string_bytes.push((string_length >> 16) as u8);
    string_bytes.push((string_length >> 8) as u8);
    string_bytes.push(string_length as u8);

    // Verify that the length is correct.
    assert!(string_bytes.len() % 64 == 0);

    for block_number in 0..string_bytes.len() / 64 {
        let start_index = block_number * 64;
        let current_string_bytes = &string_bytes[start_index..start_index + 64];

        let mut message_schedule_array: [u32; 64] = [0; 64];

        // Copy the string bytes into the first quarter of the array.
        for i in 0 .. 16 {
            let word: u32 =
                (current_string_bytes[i * 4] as u32) << 24 |
                (current_string_bytes[i * 4 + 1] as u32) << 16 |
                (current_string_bytes[i * 4 + 2] as u32) << 8 |
                (current_string_bytes[i * 4 + 3] as u32);

            message_schedule_array[i] = word;
        }

        // Extend the first quarter of the array to fill the rest.
        for i in 16 .. 64 {
            let s0 = message_schedule_array[i - 15].rotate_right(7)
                ^ message_schedule_array[i - 15].rotate_right(18)
                ^ (message_schedule_array[i - 15] >> 3);

            let s1 = message_schedule_array[i - 2].rotate_right(17)
                ^ message_schedule_array[i - 2].rotate_right(19)
                ^ (message_schedule_array[i - 2] >> 10);

            message_schedule_array[i] = message_schedule_array[i - 16]
                .wrapping_add(s0)
                .wrapping_add(message_schedule_array[i - 7])
                .wrapping_add(s1);
        }

        let mut a = initial_hash_values[0];
        let mut b = initial_hash_values[1];
        let mut c = initial_hash_values[2];
        let mut d = initial_hash_values[3];
        let mut e = initial_hash_values[4];
        let mut f = initial_hash_values[5];
        let mut g = initial_hash_values[6];
        let mut h = initial_hash_values[7];

        // Do the compression!
        for i in 0 .. 64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch)
                .wrapping_add(ROUND_CONSTANTS[i]).wrapping_add(message_schedule_array[i]);

            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        initial_hash_values[0] = initial_hash_values[0].wrapping_add(a);
        initial_hash_values[1] = initial_hash_values[1].wrapping_add(b);
        initial_hash_values[2] = initial_hash_values[2].wrapping_add(c);
        initial_hash_values[3] = initial_hash_values[3].wrapping_add(d);
        initial_hash_values[4] = initial_hash_values[4].wrapping_add(e);
        initial_hash_values[5] = initial_hash_values[5].wrapping_add(f);
        initial_hash_values[6] = initial_hash_values[6].wrapping_add(g);
        initial_hash_values[7] = initial_hash_values[7].wrapping_add(h);
    }

    println!(
        "{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
        (initial_hash_values[0]),
        (initial_hash_values[1]),
        (initial_hash_values[2]),
        (initial_hash_values[3]),
        (initial_hash_values[4]),
        (initial_hash_values[5]),
        (initial_hash_values[6]),
        (initial_hash_values[7])
    );
}