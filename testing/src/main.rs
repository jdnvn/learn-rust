fn main() {
    println!("Hello, world!");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }
}