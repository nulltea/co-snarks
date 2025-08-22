use ark_ff::{PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde::{Deserialize, Serialize};

use crate::protocols::rep3::id::PartyID;
// use crate::serde_compat::{ark_de, ark_se};

/// This type represents a replicated shared value. Since a replicated share of a field element contains additive shares of two parties, this type contains two field elements.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    CanonicalSerialize,
    CanonicalDeserialize,
    Serialize,
    Deserialize,
)]
pub struct AdditivePrimeFieldShare<F: PrimeField>(pub(crate) F);

impl<F: PrimeField> Default for AdditivePrimeFieldShare<F> {
    fn default() -> Self {
        Self(F::zero())
    }
}

impl<F: PrimeField> AdditivePrimeFieldShare<F> {
    /// Constructs a zero share.
    pub fn zero() -> Self {
        Self(F::zero())
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Double the share in place
    pub fn double_in_place(&mut self) {
        self.0.double_in_place();
    }

    /// Double the share in place
    pub fn double(&self) -> Self {
        Self(self.0.double())
    }

    /// Promotes a public field element to a replicated share by setting the additive share of the party with id=0 and leaving all other shares to be 0. Thus, the replicated shares of party 0 and party 1 are set.
    pub fn promote_from_trivial(public_value: F, id: PartyID) -> Self {
        match id {
            PartyID::ID0 => Self(public_value),
            PartyID::ID1 => Self(F::zero()),
            PartyID::ID2 => Self(F::zero()),
        }
    }

    /// Casts the additive share into a field element. Use with caution.
    pub fn into_fe(self) -> F {
        self.0
    }

    /// Casts a field element into an additive share. Don't use this to `promote_from_trivial`.
    pub fn from_fe(value: F) -> Self {
        Self(value)
    }
}
