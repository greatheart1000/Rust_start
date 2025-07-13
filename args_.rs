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


use std::env;
use std::fs;
fn main() {
    // 收集命令行参数
    println!("cwd = {:?}", env::current_dir().unwrap());
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
    let contents = fs::read_to_string(filename)
    .expect("someing went wrong");
    println!("with texts :\n {}",contents);

}
输入命令：
cargo run -- 自己 poem.txt
输出:
cwd = "/mnt/d/project/Rust_project/my_crate"
search for 自己
in file    poem.txt
with texts :
 这样 cargo 会把 abcd 当成自己的子命令或选项，the ,报错说找不到这个命令。正确的写法是用 -- 分隔 cargo 的参数和你要传给程序的参数


//重构 
use std::env;
use std::fs;

struct Config{
    query:String,
    filename:String,
}
fn main() {
    // 收集命令行参数
    println!("cwd = {:?}", env::current_dir().unwrap());
    let args: Vec<String> = env::args().collect();
    // 最少要有三个元素：程序名、query、filename
    if args.len() < 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        std::process::exit(1); }
    // let query = &args[1];
    // let filename = &args[2];
    let config =parse_config(&args);
    
    println!("search for {}", config.query);
    println!("in file    {}", config.filename);
    let contents = fs::read_to_string(config.filename)
    .expect("someing went wrong");
    println!("with texts :\n {}",contents);

}
fn parse_config(args:&[String])->Config{
    let query =args[1].clone();
    let filename =args[2].clone();
    Config {query, filename}
}

//重构模块化 
use std::env;
use std::fs;
struct Config{
    query:String,
    filename:String,
}
impl Config {
    fn new(args:&[String])->Config{
        let query =args[1].clone();
        let filename =args[2].clone();
        Config {query, filename}
    }  
}

fn main() {
    // 收集命令行参数
    println!("cwd = {:?}", env::current_dir().unwrap());
    let args: Vec<String> = env::args().collect();
    // 最少要有三个元素：程序名、query、filename
    if args.len() < 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        std::process::exit(1); }
    // let query = &args[1];
    // let filename = &args[2];
    let config =Config::new(&args);
    
    println!("search for {}", config.query);
    println!("in file    {}", config.filename);
    let contents = fs::read_to_string(config.filename)
    .expect("someing went wrong");
    println!("with texts :\n {}",contents);
}

//错误处理 
use std::env;
use std::fs;
use std::os::unix::process;
use std::process;
struct Config{
    query:String,
    filename:String,
}

impl Config {
    fn new(args:&[String])->Result<Config,&'static str>{
        if args.len()<3{
            return Err("not enough arguments");
        }
        let query =args[1].clone();
        let filename =args[2].clone();
        Config {query, filename}
    }  
}

fn main() {
    // 收集命令行参数
    println!("cwd = {:?}", env::current_dir().unwrap());
    let args: Vec<String> = env::args().collect();
    // 最少要有三个元素：程序名、query、filename
    if args.len() < 3 {
        
        std::process::exit(1); }
    // let query = &args[1];
    // let filename = &args[2];
    let config =Config::new(&args).
    unwrap_or_else(|err|{
        println!("problem parseing argsment:{}",err);
        process::exit(1)
    });
    
    println!("search for {}", config.query);
    println!("in file    {}", config.filename);
    let contents = fs::read_to_string(config.filename)
    .expect("someing went wrong");
    println!("with texts :\n {}",contents);

}

//代码重构 
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // 1. 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 2. 解析配置
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 3. 执行核心逻辑
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query:    String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query    = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// 这里我们一次性返回 `Box<dyn Error>`，在 `main` 里统一处理
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching for {:?} in {:?}", config.query, config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("With texts:\n{}", contents);
    Ok(())
}

//mian.rs的代码 
use std::env;
use my_crate::Config;
use std::process;
use std::error::Error;

fn main() {
    // 1. 收集命令行参数
    let args: Vec<String> = env::args().collect();

    // 2. 解析配置
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 3. 执行核心逻辑
    if let Err(e) = my_crate::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
//lib.rs的代码 
use std::error::Error;
use std::fs;
pub struct Config {
    pub query:    String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query    = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

/// 这里我们一次性返回 `Box<dyn Error>`，在 `main` 里统一处理
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("searching for {:?} in {:?}", config.query, config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("With texts:\n{}", contents);
    Ok(())
}

