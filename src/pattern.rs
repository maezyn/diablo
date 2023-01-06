pub fn offset(query: &str) -> u32 {
    let mut offset: u32 = 0;
    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            for c3 in 'a'..='z' {
                for c4 in 'a'..='z' {
                    let v = vec![c1, c2, c3, c4];
                    let s = String::from_iter(v);
                    if s == query {
                        println!("{}", offset);
                        return offset;
                    }
                    offset += 4;
                }
            }
        }
    }

    return 0;
}

pub fn generate(length: u32) -> String {
    let mut pattern: String = "".to_string();
    let mut pattern_len: u32 = 0;

    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            for c3 in 'a'..='z' {
                for c4 in 'a'..='z' {
                    pattern.push(c1);
                    pattern.push(c2);
                    pattern.push(c3);
                    pattern.push(c4);
                    pattern_len += 4;
                    if pattern_len == length {
                        println!("{}", pattern);
                        return pattern;
                    }
                }
            }
        }
    }

    println!("{}", pattern);
    return pattern;
}
