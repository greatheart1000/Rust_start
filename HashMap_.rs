// Hashmap 
use std::collections::HashMap;

fn main(){
    let mut score: HashMap<String, u32> =HashMap::new();
    score.insert(String::from("haha"), 10);
    

}

//得到HashMap里面的值
use std :: collections ::HashMap;
fn main(){
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name);
match score {
    Some(s) => println!("{}", s),
    None => println!("team not exist"),
}
  for (k,v) in &scores{
        println!("{}:{}",k,v);
}

//遍历HashMap 

更新 HashMap<K，V>
HashMap 大小可变
）每个K同时只能对应一个V
更新HashMap 中的数据：
－K已经存在，对应一个V
·替换现有的√
·保留现有的V，忽略新的√
·合并现有的√和新的√
－K不存在
·添加一对K，
use std::collections::HashMap;

fn main(){
    let mut score =HashMap::new();
    score.insert("aaa", 100);
    score.insert("aaa", 200);
    println!("{:?}",score);
} //{"aaa": 200}

fn main(){
    let mut score =HashMap::new();
    score.insert(String::from("aaa"), 100);
    let b =score.entry(String::from("yellow"));
    println!("{:?}",b);
    b.or_insert(50);//如果不存在 则给这个赋予这个数
    score.entry(String::from("aaa")).or_insert(50);
    println!("{:?}",score);
} //{"aaa": 100, "yellow": 50}

#统计字符串的单词总数
fn main(){
    let text ="hello world hello hi";
    let mut score =HashMap::new();
    for word in text.split_whitespace(){
        let count = score.entry(word).or_insert(0);
        *count+=1;

    }
    println!("{:#?}",score);
}
{
    "hi": 1,
    "hello": 2,
    "world": 1,
}

