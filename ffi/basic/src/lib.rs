#![allow(dead_code)]

// call C's abs
extern "C" {
    pub fn abs(input: i32) -> i32;
}

pub fn c_abs(input: i32) -> i32 {
    let ret = unsafe { abs(input) } ;
    println!("calling abs of {input} from C: {ret}", input=input, ret=ret);
    ret
}

// expose rust_abs (which calls C abs for fun!) to C
#[no_mangle]
pub extern "C" fn rust_abs(input: i32) -> i32 {
    let ret = c_abs(input);
    println!("calling abs from Rust from C of {input}: {ret}", input=input, ret=ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extern_abs() {
        assert_eq!(1, c_abs(-1));
    }
}
