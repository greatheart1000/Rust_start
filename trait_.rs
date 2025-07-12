use std::fmt::format;

//trait 特质 特性 
pub trait Summary {
    fn Summarize(&self)->String;
    //fn Summarize1(&self)->String;
}

pub struct product{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}
impl Summary for product {
    fn Summarize(&self)->String {
        format!("{},{},{},{}",self.headline,self.author,self.content,self.location)
    }
}

pub struct street {
    pub usename:String,
    pub content:String,
    pub reply:bool,
    pub retreet:bool,
} 

impl Summary for street {
    fn Summarize(&self)->String {
        format!("{},{}",self.usename,self.content)
    }
}
fn main(){

}
