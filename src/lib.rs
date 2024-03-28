//! MD5 hash algorithm
//! 
//! # References
//! 
//! - [RFC 1321](https://tools.ietf.org/html/rfc1321)
//! - [MD5 Collision Attack](https://www.exploit-db.com/exploits/39304)
//! 
//! 
//! # Examples
//! 
/// ```
/// use sunnyutil::md5;
/// fn main(){
///     println!("md5 {}",md5::md5("hello world！"));
/// }
/// ```

use crypto::digest::Digest;
use crypto::md5::Md5;

#[allow(non_camel_case_types)]
pub struct md5 {
  
}
/// impl 名称 for m
impl md5 {
    pub fn md5<S:Into<String>>(input: S) -> String {
        let mut md5 = Md5::new();
        md5.input_str(&input.into());
        md5.result_str()
    }
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use md5;
    #[test]
    fn test_md5_first() {
        println!("md5 {}",md5::md5("hello world!"));
    }
    #[test]
    fn test_md5_second() {
        let result = md5::md5("hello world!");
        assert_eq!(result, "fc3ff98e8c6a0d3087d515c0473f8677");
    }
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
