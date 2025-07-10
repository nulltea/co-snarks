use ark_ff::PrimeField;

pub(crate) mod eccvm;
pub(crate) mod goblin_prover;
pub(crate) mod ipa;
pub(crate) mod translator;

pub(crate) struct Utils;
impl Utils {
    pub fn convert_to_wnaf<F: PrimeField>(s0: &F, s1: &F) -> F {
        let mut t = *s0 + s0;
        t += t;
        t += s1;

        let naf = t + t - F::from(15u32);
        naf
    }
}
