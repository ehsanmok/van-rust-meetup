#![allow(dead_code)]

extern "C" {
    pub fn abs(input: i32) -> i32;
}

pub fn c_abs(input: i32) -> i32 {
    unsafe {
        println!("calling abs of {input} from C: {ret}", input=input, ret=abs(input));
        abs(input)
    }
}

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
