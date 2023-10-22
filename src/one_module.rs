pub fn print1(){

    for a in 'a'..='Z' {
        println!("{}",a)
    }
}


pub mod two_module{
    pub fn print2(){
        for a in 'A'..='z' {
            println!("{}",a)
        }
    }
}
