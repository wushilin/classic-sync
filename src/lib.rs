pub mod lock;
pub mod semaphore;
pub mod cyclic_barrier;
mod binding;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
