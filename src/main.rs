extern crate ruscall;

use ruscall::compile::compile_from_str;

fn main() {
    let result =
        compile_from_str(
            "\
        infixl 0 +;
        ex print::Int32->Int32;\
        main=print (4+5);\
            ",
            "foobar");
    if let Err(err)=result{
        eprintln!("{}",err);
    }
}
