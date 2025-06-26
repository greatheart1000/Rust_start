fn first_word(s: &str) -> &str {
    for (index, &byte) in s.as_bytes().iter().enumerate() {
        if byte == b' ' {
            // 在遇到第一个空格时，切出 [0, index) 范围的 &str
            return &s[0..index];
        }
    }
    // 如果没有空格，就返回整个 &str
    &s[..]
}

fn main() {
    let s = String::from("hello world");
    // 既可以传 String 的引用，也可以传字面量
    println!("第一个单词是: {}", first_word(&s));
    println!("第一个单词是: {}", first_word("goodbye"));
}
