use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let blue = "Blue";
    let yellow = "Yellow";

    scores.insert(blue.to_string(), 10);
    // 覆盖
    scores.insert(blue.to_string(), 20);

    scores.insert(yellow.to_string(), 30);

    println!("{}", get_hash_value(&scores, blue));
    println!("{:?}", scores);

    println!("{:?}", get_word_map("hello world wonderful world"));
}

fn get_hash_value(hash: &HashMap<String, i32>, s: &str) -> i32 {
    return match hash.get(s) {
        None => 0,
        Some(i) => *i
    };
}

fn get_word_map(words: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        let count = map.entry(word.to_string()).or_insert(0);
         *count += 1;
    }

    return map;
}
