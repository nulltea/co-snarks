use crate::{decider::relations::Relation, prelude::Univariate};
use ark_ff::PrimeField;
use co_builder::flavours::eccvm_flavour::ECCVMFlavour;

#[derive(Clone, Debug, Default)]
pub(crate) struct EccSetRelationAcc<F: PrimeField> {
    pub(crate) r0: Univariate<F, 22>,
    pub(crate) r1: Univariate<F, 3>,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct EccSetRelationEvals<F: PrimeField> {
    pub(crate) r0: F,
    pub(crate) r1: F,
}

pub(crate) struct EccSetRelation {}
impl EccSetRelation {
    pub(crate) const NUM_RELATIONS: usize = 2;
}

impl<F: PrimeField> EccSetRelationAcc<F> {
    pub(crate) fn scale(&mut self, elements: &[F]) {
        assert!(elements.len() == EccSetRelation::NUM_RELATIONS);
        self.r0 *= elements[0];
        self.r1 *= elements[1];
    }

    pub(crate) fn extend_and_batch_univariates<const SIZE: usize>(
        &self,
        result: &mut Univariate<F, SIZE>,
        extended_random_poly: &Univariate<F, SIZE>,
        partial_evaluation_result: &F,
    ) {
        self.r0.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r1.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
    }
}
impl<F: PrimeField> Relation<F, ECCVMFlavour> for EccSetRelation {
    type Acc = EccSetRelationAcc<F>;

    type VerifyAcc = EccSetRelationEvals<F>;

    const SKIPPABLE: bool = false; //TODO FLORIN: Where does this come from?

    fn skip<const SIZE: usize>(
        input: &crate::decider::types::ProverUnivariatesSized<F, ECCVMFlavour, SIZE>,
    ) -> bool {
        todo!() //TODO FLORIN: Where does this come from?
    }

    fn accumulate<const SIZE: usize>(
        univariate_accumulator: &mut Self::Acc,
        input: &crate::decider::types::ProverUnivariatesSized<F, ECCVMFlavour, SIZE>,
        relation_parameters: &crate::prelude::RelationParameters<F, ECCVMFlavour>,
        scaling_factor: &F,
    ) {
        todo!()
    }

    fn verify_accumulate(
        univariate_accumulator: &mut Self::VerifyAcc,
        input: &crate::prelude::ClaimedEvaluations<F, ECCVMFlavour>,
        relation_parameters: &crate::prelude::RelationParameters<F, ECCVMFlavour>,
        scaling_factor: &F,
    ) {
        todo!()
    }
}
