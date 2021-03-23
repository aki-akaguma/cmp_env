pub fn setup_env() -> (Vec<String>, usize, &'static str) {
    let s1 =
        "abcdefghijk1234567890".repeat(10) + "ErrWarnAlert" + "abcdefghijklmno".repeat(10).as_str();
    let s2 = "abcdefghijk1234567890";
    let mut v: Vec<String> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        if i > 140 {
            break;
        }
        let value = if i % 2 == 0 { s1.as_str() } else { s2 };
        let key = format!("TEST_KEY_{}", i);
        std::env::set_var(key.as_str(), value);
        v.push(key.clone());
    }
    let match_cnt = v.len() / 2;
    (v, match_cnt, s2)
}
