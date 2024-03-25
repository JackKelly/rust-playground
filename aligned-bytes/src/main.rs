use bytes;
use std::alloc::{alloc, handle_alloc_error, Layout};
use std::slice;

fn get_aligned_bytes(len: usize, align: usize) -> bytes::Bytes {
    assert_ne!(len, 0);
    let layout = Layout::from_size_align(len, align)
        .expect("failed to create Layout!")
        .pad_to_align();
    let boxed_slice = unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        };
        let s = slice::from_raw_parts_mut(ptr, len);
        // Write into `s` to mimick loading data from IO:
        (0..len).into_iter().for_each(|i| s[i] = i as u8);
        assert_eq!(s.len(), len);
        Box::from_raw(s as *mut [u8])
    };
    bytes::Bytes::from(boxed_slice)
}

fn main() {
    const LEN: usize = 128;
    const HALF_WAY: usize = LEN / 2;
    let aligned_bytes = get_aligned_bytes(LEN, 64);

    // Check this single instance of `aligned_bytes` is behaving itself:
    assert!(!aligned_bytes.is_empty());
    assert_eq!(aligned_bytes.len(), LEN);
    assert!(aligned_bytes.is_unique());
    (0..LEN)
        .into_iter()
        .for_each(|i| assert_eq!(aligned_bytes[i], i as u8));

    // Create a second view:
    let mut first_half = aligned_bytes.clone();
    let second_half = first_half.split_off(HALF_WAY);

    // Check the original `aligned_bytes`:
    assert!(!aligned_bytes.is_unique());

    // Check `first_half`:
    assert_eq!(first_half.len(), HALF_WAY);
    assert!(!first_half.is_unique());
    (0..HALF_WAY)
        .into_iter()
        .for_each(|i| assert_eq!(first_half[i], i as u8));

    // Check `second_half`:
    assert_eq!(second_half.len(), HALF_WAY);
    assert!(!second_half.is_unique());
    (0..HALF_WAY)
        .into_iter()
        .for_each(|i| assert_eq!(second_half[i], (i + HALF_WAY) as u8));

    println!("{}", 0x1);
}
