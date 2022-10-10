use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // println!("Hello, world!");
    let f =File::open("hello.txt");
    let f = match f {
       Ok(file)=>file,
        Err(error)=>match error.kind(){
            // other_error => panic!("打不开文件错误：{:?}",other_error),
            ErrorKind::NotFound=>match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=>panic!("{:?}",e)
            }
            other_error => panic!("打不开文件错误：{:?}",other_error)
        }
    };

}
