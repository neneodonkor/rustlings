#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonZeroInteger(u64);

impl PositiveNonZeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Read the tests below to clarify what should be returned.
        match value {
            0 => Err(CreationError::Zero),
            value if value < 0 => Err(CreationError::Negative),
            value => Ok(PositiveNonZeroInteger(value as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonZeroInteger::new(10),
            Ok(PositiveNonZeroInteger(10)),
        );
        assert_eq!(
            PositiveNonZeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonZeroInteger::new(0), Err(CreationError::Zero));
    }
}
