use std::fmt::Display;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn hello_world () {
    println!("Hello world!");
}

pub trait Authenticator<T> {
    fn serialize(&mut self, data: T) where T: Display{
        println!("{}", data);
    }
}