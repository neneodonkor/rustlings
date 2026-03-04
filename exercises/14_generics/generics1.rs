// `Vec<T>` is generic over the type `T`. In most cases, the compiler is able to
// infer `T`, for example after pushing a value with a concrete type to the vector.
// But in this exercise, the compiler needs some help through a type annotation.

fn main() {
    // TODO: Fix the compiler error by annotating the type of the vector
    // `Vec<T>`. Choose `T` as some integer type that can be created from
    // `u8` and `i8`.
    let mut numbers: Vec<IntType> = Vec::new();

    #[derive(PartialEq, Debug)]
    enum IntType {
        Signed(i8),
        Unsigned(u8),
    }

    // Implement From<u8> for IntType
    impl From<u8> for IntType {
        fn from(num: u8) -> Self {
            IntType::Unsigned(num)
        }
    }

    // Implement From<i8> for IntType
    impl From<i8> for IntType {
        fn from(num: i8) -> Self {
            IntType::Signed(num)
        }
    }

    // Don't change the lines below.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
