
use std::cmp;

pub fn rabin_karp(pattern: String, text: String) -> bool {
    let PRIME = 31;
    let MOD = 1000000009;
    let S = pattern.len();
    let T = text.len();

    let mut pow: Vec<i64> = vec![0; cmp::max(S,T)];
    pow[0] = 1;
    for i in 1..pow.len(){
        pow[i] = (pow[i - 1] * PRIME) % MOD;
    }

    let mut text_hash: Vec<i64> = vec![0; T+1];
    let mut pattern_hash: i64 = 0;
    for i in 0..T{
        let Some(ch) = text.chars().nth(i) else { panic!("Pattern is empty") };
        text_hash[i+1] = (text_hash[i] + (ch as i64 - 'a' as i64 + 1)*pow[i]) % MOD;
    }
    for i in 0..S{
        let Some(ch) = pattern.chars().nth(i) else { panic!("Pattern is empty") };
        pattern_hash = (pattern_hash + (ch as i64 - 'a' as i64 + 1)*pow[i]) % MOD;
    }
    let mut i = 0;
    while i + S - 1  < T {
        let curr_hash = (text_hash[i+S] - text_hash[i] + MOD) % MOD;
        if curr_hash == pattern_hash * pow[i] % MOD{
            println!("Pattern found at {}", i);
        }
        i += 1;
    }
    return true;
}