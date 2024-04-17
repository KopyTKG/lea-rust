mod lea;
use lea::{prelude::*, Lea128};

fn string_to_u8_array(s: &str) -> [u8; 16] {
    let mut arr = [0u8; 16];
    for (i, c) in s.bytes().enumerate() {
        if i >= 16 {
            break;
        }
        arr[i] = c;
    }
    arr
}

fn u8_array_to_string(arr: &[u8; 16]) -> String {
    let mut s = String::new();
    for c in arr.iter() {
        s.push(*c as char);
    }
    s
}

fn main() {
    let key = arr![u8; 0x0F, 0x1E, 0x2D, 0x3C, 0x4B, 0x5A, 0x69, 0x78, 0x87, 0x96, 0xA5, 0xB4, 0xC3, 0xD2, 0xE1, 0xF0];
    let data = "Hello, World!";
    let ptxt = string_to_u8_array(data);

    let lea128 = Lea128::new(&key);
    let mut block = ptxt.clone();
    lea128.encrypt_block((&mut block).into());


    lea128.decrypt_block((&mut block).into());
    let text = u8_array_to_string(&block);
    println!("{:?}", text);

}
