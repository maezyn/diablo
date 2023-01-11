fn db(t: usize, p: usize, a: &mut Vec<u8>, seq: &mut Vec<u8>, k: usize, n: usize) {
    if t > n {
        if n % p == 0 {
            for i in 1..p+1 {
                seq.push(a[i]);
            }
        }
    }
    else {
        a[t] = a[t - p];
        db(t + 1, p, a, seq, k, n);
        let mut j: u8 = a[t - p] + 1;
        while (usize::from(j) < k) {
            a[t] = j & 0xff;
            db(t + 1, t, a, seq, k, n);
            j += 1;
        }
    }
}


fn de_bruijn(k: usize, n: usize) -> Vec<u8> {
    let mut a: Vec<u8> = vec![0; k*n];
    let mut seq: Vec<u8> = Vec::new();

    db(1, 1, &mut a, &mut seq, k, n);

    let mut buf: Vec<u8> = Vec::new();
    for i in seq.iter() {
        buf.push(*i + 0x61);
    }

    return buf;
}

pub fn offset(query: &str) -> usize {
    let pattern: String = String::from_utf8(de_bruijn(26, 4)).unwrap();
    let offset = pattern.find(query);

    match offset {
        None => {
            panic!("Unable to find offset of string");
        },
        Some(offset) => {
            println!("Found! {}", offset);
            return offset;
        }
    }

}

pub fn generate(length: usize) -> String {
    if length > 456976 {
        panic!("Unable to generate pattern of length: {}", length);
    }
    let pattern: String = String::from_utf8(de_bruijn(26, 4)).unwrap();
    let slice_pattern = &pattern[..length];

    println!("{}", slice_pattern);
    return slice_pattern.to_owned();
}

#[test]
fn test_de_bruijn() {
    let pattern: String = String::from_utf8(de_bruijn(26, 4)).unwrap();
    assert_eq!(pattern.len(), 456976);
}

#[test]
fn test_generate_pattern() {
    assert_eq!(generate(12), "aaaabaaacaaa");
}

#[test]
fn test_generate_odd_len_pattern() {
    assert_eq!(generate(1), "a");
}

#[test]
#[should_panic]
fn test_generate_exceeds_max_len() {
    (generate(456976+1));
}

#[test]
fn test_find_offset_valid_pattern() {
    assert_eq!(offset("caaa"), 8);
}

#[test]
#[should_panic]
fn test_find_offset_invalid_pattern() {
    offset("????");
}


