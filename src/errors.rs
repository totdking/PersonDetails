use std::error::Error;
use std::fmt::{Debug,Display,Formatter, Result};
#[derive(Debug)]
pub enum MyError{
    NotInRange,
}
impl Display for MyError{
    fn fmt(&self, f: &mut Formatter<'_>)-> Result{
       
        write!(f, "{:?}",self)
    }
}
impl Error for MyError{}