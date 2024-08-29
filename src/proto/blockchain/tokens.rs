use super::result::{BlockchainProtoError, BlockchainProtoResult};

/// # Parrot token
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Parrots {
    wings: Wings,
}

impl Parrots {
    pub fn new(wings: Wings) -> Self {
        Self { wings }
    }

    pub fn as_wings(&self) -> &Wings {
        &self.wings
    }
}

/// ## Wings
///
/// 1_000_000 **Wings** => 1 **Parrot**
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Wings(u64);

impl Wings {
    pub const WINGS_PER_PARROT: u64 = 1_000_000;

    pub(crate) const fn zero() -> Self {
        Self(0)
    }

    pub fn new(wings: u64) -> Self {
        Self(wings)
    }

    pub fn to_parrots(self) -> BlockchainProtoResult<Parrots> {
        Ok(self.try_into()?)
    }

    pub(crate) fn as_bytes(&self) -> [u8; size_of::<u64>()] {
        self.0.to_be_bytes()
    }
}

impl From<Parrots> for Wings {
    fn from(parrots: Parrots) -> Self {
        parrots.wings
    }
}

impl TryFrom<Wings> for Parrots {
    type Error = BlockchainProtoError;

    fn try_from(wings: Wings) -> Result<Self, Self::Error> {
        if wings.0 % Wings::WINGS_PER_PARROT == 0 {
            Ok(Parrots::new(wings))
        } else {
            Err(BlockchainProtoError::TokenConversion(
                "Number of wings must be divisible by WINGS_PER_PARROT".into(),
            ))
        }
    }
}

// Additional utility implementations

impl std::ops::Add for Wings {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Wings {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl std::ops::Mul<u64> for Wings {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self(self.0 * rhs)
    }
}

impl std::ops::Div<u64> for Wings {
    type Output = Self;

    fn div(self, rhs: u64) -> Self {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wings_creation() {
        let wings = Wings::new(500_000);
        assert_eq!(wings.0, 500_000);
    }

    #[test]
    fn test_wings_zero() {
        let zero_wings = Wings::zero();
        assert_eq!(zero_wings.0, 0);
    }

    #[test]
    fn test_wings_as_bytes() {
        let wings = Wings::new(1_234_567);
        let bytes = wings.as_bytes();
        assert_eq!(bytes, 1_234_567u64.to_be_bytes());
    }

    #[test]
    fn test_wings_from_u64() {
        let wings = Wings::new(1_000_000);
        assert_eq!(wings.0, 1_000_000);
    }

    #[test]
    fn test_parrots_creation() {
        let wings = Wings::new(1_000_000);
        let parrots = Parrots::new(wings.clone());
        assert_eq!(*parrots.as_wings(), wings);
    }

    #[test]
    fn test_wings_to_parrots() {
        let wings = Wings::new(2_000_000);
        let parrots = wings.clone().to_parrots().unwrap();
        assert_eq!(*parrots.as_wings(), wings);
    }

    #[test]
    fn test_parrots_to_wings() {
        let wings = Wings::new(3_000_000);
        let parrots = Parrots::new(wings.clone());
        assert_eq!(*parrots.as_wings(), wings);
    }

    #[test]
    fn test_wings_from_parrots() {
        let original_wings = Wings::new(4_000_000);
        let parrots = Parrots::new(original_wings);
        let wings: Wings = parrots.into();
        assert_eq!(wings.0, 4_000_000);
    }

    #[test]
    fn test_parrots_try_from_wings_success() {
        let wings = Wings::new(5_000_000);
        let parrots = Parrots::try_from(wings.clone());
        assert!(parrots.is_ok());
        assert_eq!(*parrots.unwrap().as_wings(), wings);
    }

    #[test]
    fn test_parrots_try_from_wings_failure() {
        let wings = Wings::new(5_500_000);
        let parrots = Parrots::try_from(wings);
        assert!(parrots.is_err());
    }

    #[test]
    fn test_wings_addition() {
        let wings1 = Wings::new(1_000_000);
        let wings2 = Wings::new(2_000_000);
        let sum = wings1 + wings2;
        assert_eq!(sum.0, 3_000_000);
    }

    #[test]
    fn test_wings_subtraction() {
        let wings1 = Wings::new(3_000_000);
        let wings2 = Wings::new(1_000_000);
        let difference = wings1 - wings2;
        assert_eq!(difference.0, 2_000_000);
    }

    #[test]
    fn test_wings_multiplication() {
        let wings = Wings::new(1_000_000);
        let result = wings * 3;
        assert_eq!(result.0, 3_000_000);
    }

    #[test]
    fn test_wings_division() {
        let wings = Wings::new(4_000_000);
        let result = wings / 2;
        assert_eq!(result.0, 2_000_000);
    }
}
