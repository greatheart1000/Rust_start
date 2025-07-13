use std::env;

fn main() {
    // 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 最少要有三个元素：程序名、query、filename
    if args.len() < 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        std::process::exit(1);
    }

    let query = &args[1];
    let filename = &args[2];

    println!("search for {}", query);
    println!("in file    {}", filename);
}
运行命令： cargo run -- abcd readme.txt
输出结果:
search for abcd
in file    readme.txt
