use crate::{decider::relations::Relation, prelude::Univariate};
use ark_ff::PrimeField;
use co_builder::flavours::eccvm_flavour::ECCVMFlavour;
use co_builder::polynomials::polynomial_flavours::{
    PrecomputedEntitiesFlavour, ShiftedWitnessEntitiesFlavour, WitnessEntitiesFlavour,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct EccMsmRelationAcc<F: PrimeField> {
    pub(crate) r0: Univariate<F, 8>,
    pub(crate) r1: Univariate<F, 8>,
    pub(crate) r2: Univariate<F, 8>,
    pub(crate) r3: Univariate<F, 8>,
    pub(crate) r4: Univariate<F, 8>,
    pub(crate) r5: Univariate<F, 8>,
    pub(crate) r6: Univariate<F, 8>,
    pub(crate) r7: Univariate<F, 8>,
    pub(crate) r8: Univariate<F, 8>,
    pub(crate) r9: Univariate<F, 8>,
    pub(crate) r10: Univariate<F, 8>,
    pub(crate) r11: Univariate<F, 8>,
    pub(crate) r12: Univariate<F, 8>,
    pub(crate) r13: Univariate<F, 8>,
    pub(crate) r14: Univariate<F, 8>,
    pub(crate) r15: Univariate<F, 8>,
    pub(crate) r16: Univariate<F, 8>,
    pub(crate) r17: Univariate<F, 8>,
    pub(crate) r18: Univariate<F, 8>,
    pub(crate) r19: Univariate<F, 8>,
    pub(crate) r20: Univariate<F, 8>,
    pub(crate) r21: Univariate<F, 8>,
    pub(crate) r22: Univariate<F, 8>,
    pub(crate) r23: Univariate<F, 8>,
    pub(crate) r24: Univariate<F, 8>,
    pub(crate) r25: Univariate<F, 8>,
    pub(crate) r26: Univariate<F, 8>,
    pub(crate) r27: Univariate<F, 8>,
    pub(crate) r28: Univariate<F, 8>,
    pub(crate) r29: Univariate<F, 8>,
    pub(crate) r30: Univariate<F, 8>,
    pub(crate) r31: Univariate<F, 8>,
    pub(crate) r32: Univariate<F, 8>,
    pub(crate) r33: Univariate<F, 8>,
    pub(crate) r34: Univariate<F, 8>,
    pub(crate) r35: Univariate<F, 8>,
}
#[derive(Clone, Debug, Default)]
pub(crate) struct EccMsmRelationEvals<F: PrimeField> {
    pub(crate) r0: F,
    pub(crate) r1: F,
    pub(crate) r2: F,
    pub(crate) r3: F,
    pub(crate) r4: F,
    pub(crate) r5: F,
    pub(crate) r6: F,
    pub(crate) r7: F,
    pub(crate) r8: F,
    pub(crate) r9: F,
    pub(crate) r10: F,
    pub(crate) r11: F,
    pub(crate) r12: F,
    pub(crate) r13: F,
    pub(crate) r14: F,
    pub(crate) r15: F,
    pub(crate) r16: F,
    pub(crate) r17: F,
    pub(crate) r18: F,
    pub(crate) r19: F,
    pub(crate) r20: F,
    pub(crate) r21: F,
    pub(crate) r22: F,
    pub(crate) r23: F,
    pub(crate) r24: F,
    pub(crate) r25: F,
    pub(crate) r26: F,
    pub(crate) r27: F,
    pub(crate) r28: F,
    pub(crate) r29: F,
    pub(crate) r30: F,
    pub(crate) r31: F,
    pub(crate) r32: F,
    pub(crate) r33: F,
    pub(crate) r34: F,
    pub(crate) r35: F,
}

pub(crate) struct EccMsmRelation {}
impl EccMsmRelation {
    pub(crate) const NUM_RELATIONS: usize = 19;
}

impl<F: PrimeField> EccMsmRelationAcc<F> {
    pub(crate) fn scale(&mut self, elements: &[F]) {
        assert!(elements.len() == EccMsmRelation::NUM_RELATIONS);
        self.r0 *= elements[0];
        self.r1 *= elements[1];
        self.r2 *= elements[2];
        self.r3 *= elements[3];
        self.r4 *= elements[4];
        self.r5 *= elements[5];
        self.r6 *= elements[6];
        self.r7 *= elements[7];
        self.r8 *= elements[8];
        self.r9 *= elements[9];
        self.r10 *= elements[10];
        self.r11 *= elements[11];
        self.r12 *= elements[12];
        self.r13 *= elements[13];
        self.r14 *= elements[14];
        self.r15 *= elements[15];
        self.r16 *= elements[16];
        self.r17 *= elements[17];
        self.r18 *= elements[18];
        self.r19 *= elements[0];
        self.r20 *= elements[1];
        self.r21 *= elements[2];
        self.r22 *= elements[3];
        self.r23 *= elements[4];
        self.r24 *= elements[5];
        self.r25 *= elements[6];
        self.r26 *= elements[7];
        self.r27 *= elements[8];
        self.r28 *= elements[9];
        self.r29 *= elements[10];
        self.r30 *= elements[11];
        self.r31 *= elements[12];
        self.r32 *= elements[13];
        self.r33 *= elements[14];
        self.r34 *= elements[15];
        self.r35 *= elements[16];
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
        self.r2.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r3.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r4.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r5.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r6.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r7.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r8.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r9.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r10.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r11.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r12.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r13.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r14.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r15.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r16.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r17.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r18.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r19.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r20.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r21.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r22.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r23.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r24.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r25.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r26.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r27.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r28.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r29.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r30.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r31.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r32.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r33.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r34.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
        self.r35.extend_and_batch_univariates(
            result,
            extended_random_poly,
            partial_evaluation_result,
            true,
        );
    }
}

impl<F: PrimeField> Relation<F, ECCVMFlavour> for EccMsmRelation {
    type Acc = EccMsmRelationAcc<F>;

    type VerifyAcc = EccMsmRelationEvals<F>;

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
        // let z1_zero = input.witness.transcript_z1zero();
        // let z2_zero = input.witness.transcript_z2zero();
        // let msm_count_zero_at_transition = input.witness.transcript_msm_count_zero_at_transition();
        // let q_add = input.witness.transcript_add();
        // let q_mul = input.witness.transcript_mul();
        // let q_eq = input.witness.transcript_eq();
        // let transcript_msm_transition = input.witness.transcript_msm_transition();
        // let is_accumulator_empty = input.witness.transcript_accumulator_empty();
        // let q_reset_accumulator = input.witness.transcript_reset_accumulator();
        // let transcript_pinfinity = input.witness.transcript_base_infinity();
        // let transcript_msm_infinity = input.witness.transcript_msm_infinity();
        // let transcript_add_x_equal = input.witness.transcript_add_x_equal();
        // let transcript_add_y_equal = input.witness.transcript_add_y_equal();
        // let precompute_point_transition = input.witness.precompute_point_transition();
        // let msm_transition = input.witness.msm_transition();
        // let msm_add = input.witness.msm_add();
        // let msm_double = input.witness.msm_double();
        // let msm_skew = input.witness.msm_skew();
        // let precompute_select = input.witness.precompute_select();
        // let minus_one = -F::one();

        // let mut tmp = q_eq.to_owned() + &minus_one;
        // tmp *= q_eq;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r0.evaluations.len() {
        //     univariate_accumulator.r0.evaluations[i] += tmp.evaluations[i];
        // }

        // let mut tmp = q_add.to_owned() + &minus_one;
        // tmp *= q_add;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r1.evaluations.len() {
        //     univariate_accumulator.r1.evaluations[i] += tmp.evaluations[i];
        // }

        // let mut tmp = q_mul.to_owned() + &minus_one;
        // tmp *= q_mul;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r2.evaluations.len() {
        //     univariate_accumulator.r2.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = q_reset_accumulator.to_owned() + &minus_one;
        // tmp *= q_reset_accumulator;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r3.evaluations.len() {
        //     univariate_accumulator.r3.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = transcript_msm_transition.to_owned() + &minus_one;
        // tmp *= transcript_msm_transition;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r4.evaluations.len() {
        //     univariate_accumulator.r4.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = is_accumulator_empty.to_owned() + &minus_one;
        // tmp *= is_accumulator_empty;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r5.evaluations.len() {
        //     univariate_accumulator.r5.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = z1_zero.to_owned() + &minus_one;
        // tmp *= z1_zero;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r6.evaluations.len() {
        //     univariate_accumulator.r6.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = z2_zero.to_owned() + &minus_one;
        // tmp *= z2_zero;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r7.evaluations.len() {
        //     univariate_accumulator.r7.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = transcript_add_x_equal.to_owned() + &minus_one;
        // tmp *= transcript_add_x_equal;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r8.evaluations.len() {
        //     univariate_accumulator.r8.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = transcript_add_y_equal.to_owned() + &minus_one;
        // tmp *= transcript_add_y_equal;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r9.evaluations.len() {
        //     univariate_accumulator.r9.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = transcript_pinfinity.to_owned() + &minus_one;
        // tmp *= transcript_pinfinity;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r10.evaluations.len() {
        //     univariate_accumulator.r10.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = transcript_msm_infinity.to_owned() + &minus_one;
        // tmp *= transcript_msm_infinity;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r11.evaluations.len() {
        //     univariate_accumulator.r11.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = msm_count_zero_at_transition.to_owned() + &minus_one;
        // tmp *= msm_count_zero_at_transition;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r12.evaluations.len() {
        //     univariate_accumulator.r12.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = msm_transition.to_owned() + &minus_one;
        // tmp *= msm_transition;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r13.evaluations.len() {
        //     univariate_accumulator.r13.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = precompute_point_transition.to_owned() + &minus_one;
        // tmp *= precompute_point_transition;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r14.evaluations.len() {
        //     univariate_accumulator.r14.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = msm_add.to_owned() + &minus_one;
        // tmp *= msm_add;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r15.evaluations.len() {
        //     univariate_accumulator.r15.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = msm_double.to_owned() + &minus_one;
        // tmp *= msm_double;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r16.evaluations.len() {
        //     univariate_accumulator.r16.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = msm_skew.to_owned() + &minus_one;
        // tmp *= msm_skew;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r17.evaluations.len() {
        //     univariate_accumulator.r17.evaluations[i] += tmp.evaluations[i];
        // }
        // let mut tmp = precompute_select.to_owned() + &minus_one;
        // tmp *= precompute_select;
        // tmp *= scaling_factor;
        // for i in 0..univariate_accumulator.r18.evaluations.len() {
        //     univariate_accumulator.r18.evaluations[i] += tmp.evaluations[i];
        // }
    let  x1 = input.witness.msm_x1();
    let  y1 = input.witness.msm_y1();
    let  x2 = input.witness.msm_x2();
    let  y2 = input.witness.msm_y2();
    let  x3 = input.witness.msm_x3();
    let  y3 = input.witness.msm_y3();
    let  x4 = input.witness.msm_x4();
    let  y4 = input.witness.msm_y4();
    let  collision_inverse1 = input.witness.msm_collision_x1();
    let  collision_inverse2 = input.witness.msm_collision_x2();
    let  collision_inverse3 = input.witness.msm_collision_x3();
    let  collision_inverse4 = input.witness.msm_collision_x4();
    let  lambda1 = input.witness.msm_lambda1();
    let  lambda2 = input.witness.msm_lambda2();
    let  lambda3 = input.witness.msm_lambda3();
    let  lambda4 = input.witness.msm_lambda4();
    let  lagrange_first = input.precomputed.lagrange_first();
    let  add1 = input.witness.msm_add1();
    let  add1_shift = input.witness.msm_add1_shift();
    let  add2 = input.witness.msm_add2();
    let  add3 = input.witness.msm_add3();
    let  add4 = input.witness.msm_add4();
    let  acc_x = input.witness.msm_accumulator_x();
    let  acc_y = input.witness.msm_accumulator_y();
    let  acc_x_shift = input.witness.msm_accumulator_x_shift();
    let  acc_y_shift = input.witness.msm_accumulator_y_shift();
    let  slice1 = input.witness.msm_slice1();
    let  slice2 = input.witness.msm_slice2();
    let  slice3 = input.witness.msm_slice3();
    let  slice4 = input.witness.msm_slice4();
    let  msm_transition = input.witness.msm_transition();
    let  msm_transition_shift = input.witness.msm_transition_shift();
    let  round = input.witness.msm_round();
    let  round_shift = input.witness.msm_round_shift();
    let  q_add = input.witness.msm_add();
    let  q_add_shift = input.witness.msm_add_shift();
    let  q_skew = input.witness.msm_skew();
    let  q_skew_shift = input.witness.msm_skew_shift();
    let  q_double = input.witness.msm_double();
    let  q_double_shift = input.witness.msm_double_shift();
    let  msm_size = input.witness.msm_size_of_msm();
    // const auto& msm_size_shift = View(in.msm_size_of_msm_shift);
    let  pc = input.witness.msm_pc();
    let  pc_shift = input.witness.msm_pc_shift();
    let  count = input.witness.msm_count();
    let  count_shift = input.witness.msm_count_shift();
    let minus_one= -F::one();
    let is_not_first_row =lagrange_first.to_owned()*&minus_one + &F::one();

    /*
     * @brief Evaluating ADDITION rounds
     *
     * This comment describes the algorithm we want the Prover to perform.
     * The relations we constrain are supposed to make an honest Prover compute witnesses consistent with the following:
     *
     * For an MSM of size-k...
     *
     * Algorithm to determine if round at shifted row is an ADDITION round:
     *     1. count_shift < msm_size
     *     2. round != 32
     *
     * Algorithm to process MSM ADDITION round:
     * 1. If `round == 0` set `count = 0`
     * 2. For j = pc + count, perform the following:
     * 2a.      If j + 3 < k: [P_{j + 3}] = T_{j+ 3}[slice_{j + 3}]
     * 2b.      If j + 2 < k: [P_{j + 2}] = T_{j+ 2}[slice_{j + 2}]
     * 2c.      If j + 1 < k: [P_{j + 1}] = T_{j+ 1}[slice_{j + 1}]
     * 2d.                    [P_{j}]     = T_{j}[slice_{j}]
     * 2e.      If j + 3 < k: [Acc_shift] = [Acc] + [P_j] + [P_{j+1}] + [P_{j+2}] + [P_{j+3}]
     * 2f. Else If j + 2 < k: [Acc_shift] = [Acc] + [P_j] + [P_{j+1}] + [P_{j+2}]
     * 2g. Else IF j + 1 < k: [Acc_shift] = [Acc] + [P_j] + [P_{j+1}]
     * 2h. Else             : [Acc_shift] = [Acc] + [P_j]
     * 3. `count_shift = count + 1 + (j + 1 < k) + (j + 2 < k) + (j + 3 < k)`
     */

    /*
     * @brief Constraining addition rounds
     *
     * The boolean column q_add describes whether a round is an ADDITION round.
     * The values of q_add are Prover-defined. We need to ensure they set q_add correctly.
     * We rely on the following statements that we assume are constrained to be true (from other relations):
     *      1. The set of reads into (pc, round, wnaf_slice) is constructed when q_add = 1
     *      2. The set of reads into (pc, round, wnaf_slice) must match the set of writes from the point_table columns
     *      3. The set of writes into (pc, round, wnaf_slice) from the point table columns is correct
     *      4. `round` only updates when `q_add = 1` at current row and `q_add = 0` at next row
     * If a Prover sets `q_add = 0` when an honest Prover would set `q_add = 1`,
     * this will produce an inequality in the set of reads / writes into the (pc, round, wnaf_slice) table.
     *
     * The addition algorithm has several IF/ELSE statements based on comparing `count` with `msm_size`.
     * Instead of directly constraining these, we define 4 boolean columns `q_add1, q_add2, q_add3, q_add4`.
     * Like `q_add`, their values are Prover-defined. We need to ensure they are set correctly.
     * We update the above conditions on reads into (pc, round, wnaf_slice) to the following:
     *      1. The set of reads into (pc_{count}, round, wnaf_slice_{count}) is constructed when q_add = 1 AND q_add1 =
     * 1
     *      2. The set of reads into (pc_{count + 1}, round, wnaf_slice_{count + 1}) is constructed when q_add = 1 AND
     * q_add2 = 1
     *      3. The set of reads into (pc_{count + 2}, round, wnaf_slice_{count + 2}) is constructed when q_add = 1 AND
     * q_add3 = 1
     *      4. The set of reads into (pc_{count + 3}, round, wnaf_slice_{count + 3}) is constructed when q_add = 1 AND
     * q_add4 = 1
     *
     * To ensure that all q_addi values are correctly set we apply consistency checks to q_add1/q_add2/q_add3/q_add4:
     * 1. If q_add2 = 1, require q_add1 = 1
     * 2. If q_add3 = 1, require q_add2 = 1
     * 3. If q_add4 = 1, require q_add3 = 1
     * 4. If q_add1_shift = 1 AND round does not update between rows, require q_add4 = 1
     *
     * We want to use all of the above to reason about the set of reads into (pc, round, wnaf_slice).
     * The goal is to conclude that any case where the Prover incorrectly sets q_add/q_add1/q_add2/q_add3/q_add4 will
     * produce a set inequality between the reads/writes into (pc, round, wnaf_slice)
     */

    /*
     * @brief Addition relation
     *
     * All addition operations in ECCVMMSMRelationImpl are conditional additions!
     * This method returns two Accumulators that represent x/y coord of output.
     * Output is either an addition of inputs, or xa/ya dpeending on value of `selector`.
     * Additionally, we require `lambda = 0` if `selector = 0`.
     * The `collision_relation` accumulator tracks a subrelation that validates xb != xa.
     * Repeated calls to this method will increase the max degree of the Accumulator output
     * Degree of x_out, y_out = max degree of x_a/x_b + 1
     * 4 Iterations will produce an output degree of 6
     */
    // auto add = [&](auto& xb,
    //                auto& yb,
    //                auto& xa,
    //                auto& ya,
    //                auto& lambda,
    //                auto& selector,
    //                auto& relation,
    //                auto& collision_relation) {
    //     // L * (1 - s) = 0
    //     // (combine) (L * (xb - xa - 1) - yb - ya) * s + L = 0
    //     relation += selector * (lambda * (xb - xa - 1) - (yb - ya)) + lambda;
    //     collision_relation += selector * (xb - xa);
    //     // x3 = L.L + (-xb - xa) * q + (1 - q) xa
    //     auto x_out = lambda.sqr() + (-xb - xa - xa) * selector + xa;

    //     // y3 = L . (xa - x3) - ya * q + (1 - q) ya
    //     auto y_out = lambda * (xa - x_out) + (-ya - ya) * selector + ya;
    //     return std::array<Accumulator, 2>{ x_out, y_out };
    // };

    /*
     * @brief First Addition relation
     *
     * The first add operation per row is treated differently.
     * Normally we add the point xa/ya with an accumulator xb/yb,
     * BUT, if this row STARTS a multiscalar multiplication,
     * We need to add the point xa/ya with the "offset generator point" xo/yo
     * The offset generator point's purpose is to ensure that no intermediate computations in the MSM will produce
     * points at infinity, for an honest Prover.
     * (we ensure soundness by validating the x-coordinates of xa/xb are not the same i.e. incomplete addition formula
     * edge cases have not been hit)
     * Note: this technique is only statistically complete, as there is a chance of an honest Prover creating a
     * collision, but this probability is equivalent to solving the discrete logarithm problem
     */
    auto first_add = [&](auto& xb,
                         auto& yb,
                         auto& xa,
                         auto& ya,
                         auto& lambda,
                         auto& selector,
                         auto& relation,
                         auto& collision_relation) {
                            acc_x, acc_y, x1, y1, lambda1, msm_transition, add_relation, x1_collision_relation
        // N.B. this is brittle - should be curve agnostic but we don't propagate the curve parameter into relations!
        let domain_separator = "ECCVM_OFFSET_GENERATOR";
        let domain_bytes=Vec::with_capacity(domain_separator.len());
        for i in domain_separator.chars() {
            domain_bytes.push(i as u8);
        }
        constexpr auto offset_generator = bb::g1::derive_generators("ECCVM_OFFSET_GENERATOR", 1)[0];
        constexpr uint256_t oxu = offset_generator.x;
        constexpr uint256_t oyu = offset_generator.y;
        const Accumulator xo(oxu);
        const Accumulator yo(oyu);

        auto x = xo * msm_transition + acc_x * (-msm_transition + 1);
        auto y = yo * msm_transition + acc_y * (-msm_transition + 1);
        add_relation += lambda1 * (x - x1) - (y - y1); // degree 3
        x1_collision_relation += (x1 - x);
        auto x_out = lambda1 * lambda1 + (-x - x1);
        auto y_out = lambda1 * (x1 - x_out) - y1;
        return std::array<Accumulator, 2>{ x_out, y_out };
    };

    // ADD operations (if row represents ADD round, not SKEW or DOUBLE)
    Accumulator add_relation(0);
    Accumulator x1_collision_relation(0);
    Accumulator x2_collision_relation(0);
    Accumulator x3_collision_relation(0);
    Accumulator x4_collision_relation(0);
    // If msm_transition = 1, we have started a new MSM. We need to treat the current value of [Acc] as the point at
    // infinity!
    auto [x_t1, y_t1] = first_add(acc_x, acc_y, x1, y1, lambda1, msm_transition, add_relation, x1_collision_relation);
    auto [x_t2, y_t2] = add(x2, y2, x_t1, y_t1, lambda2, add2, add_relation, x2_collision_relation);
    auto [x_t3, y_t3] = add(x3, y3, x_t2, y_t2, lambda3, add3, add_relation, x3_collision_relation);
    auto [x_t4, y_t4] = add(x4, y4, x_t3, y_t3, lambda4, add4, add_relation, x4_collision_relation);

    // Validate accumulator output matches ADD output if q_add = 1
    // (this is a degree-6 relation)
    std::get<0>(accumulator) += q_add * (acc_x_shift - x_t4) * scaling_factor;
    std::get<1>(accumulator) += q_add * (acc_y_shift - y_t4) * scaling_factor;
    std::get<2>(accumulator) += q_add * add_relation * scaling_factor;

    /*
     * @brief doubles a point.
     *
     * Degree of x_out = 2
     * Degree of y_out = 3
     * Degree of relation = 4
     */
    // auto dbl = [&](auto& x, auto& y, auto& lambda, auto& relation) {
    //     auto two_x = x + x;
    //     relation += lambda * (y + y) - (two_x + x) * x;
    //     auto x_out = lambda.sqr() - two_x;
    //     auto y_out = lambda * (x - x_out) - y;
    //     return std::array<Accumulator, 2>{ x_out, y_out };
    // };

    /*
     * @brief
     *
     * Algorithm to determine if round is a DOUBLE round:
     *    1. count_shift >= msm_size
     *    2. round != 32
     *
     * Algorithm to process MSM DOUBLE round:
     * [Acc_shift] = (([Acc].double()).double()).double()
     *
     * As with additions, the column q_double describes whether row is a double round. It is Prover-defined.
     * The value of `msm_round` can only update when `q_double = 1` and we use this to ensure Prover correctly sets
     * `q_double`. (see round transition relations further down)
     */
    Accumulator double_relation(0);
    auto [x_d1, y_d1] = dbl(acc_x, acc_y, lambda1, double_relation);
    auto [x_d2, y_d2] = dbl(x_d1, y_d1, lambda2, double_relation);
    auto [x_d3, y_d3] = dbl(x_d2, y_d2, lambda3, double_relation);
    auto [x_d4, y_d4] = dbl(x_d3, y_d3, lambda4, double_relation);
    std::get<10>(accumulator) += q_double * (acc_x_shift - x_d4) * scaling_factor;
    std::get<11>(accumulator) += q_double * (acc_y_shift - y_d4) * scaling_factor;
    std::get<12>(accumulator) += q_double * double_relation * scaling_factor;

    /*
     * @brief SKEW operations
     * When computing x * [P], if x is even we must subtract [P] from accumulator
     * (this is because our windowed non-adjacent-form can only represent odd numbers)
     * Round 32 represents "skew" round.
     * If scalar slice == 7, we add into accumulator (point_table[7] maps to -[P])
     * If scalar slice == 0, we do not add into accumulator
     * i.e. for the skew round we can use the slice values as our "selector" when doing conditional point adds
     */
    Accumulator skew_relation(0);
    static FF inverse_seven = FF(7).invert();
    auto skew1_select = slice1 * inverse_seven;
    auto skew2_select = slice2 * inverse_seven;
    auto skew3_select = slice3 * inverse_seven;
    auto skew4_select = slice4 * inverse_seven;
    Accumulator x1_skew_collision_relation(0);
    Accumulator x2_skew_collision_relation(0);
    Accumulator x3_skew_collision_relation(0);
    Accumulator x4_skew_collision_relation(0);
    // add skew points iff row is a SKEW row AND slice = 7 (point_table[7] maps to -[P])
    // N.B. while it would be nice to have one `add` relation for both ADD and SKEW rounds,
    // this would increase degree of sumcheck identity vs evaluating them separately.
    // This is because, for add rounds, the result of adding [P1], [Acc] is [P1 + Acc] or [P1]
    //             but for skew rounds, the result of adding [P1], [Acc] is [P1 + Acc] or [Acc]
    auto [x_s1, y_s1] = add(x1, y1, acc_x, acc_y, lambda1, skew1_select, skew_relation, x1_skew_collision_relation);
    auto [x_s2, y_s2] = add(x2, y2, x_s1, y_s1, lambda2, skew2_select, skew_relation, x2_skew_collision_relation);
    auto [x_s3, y_s3] = add(x3, y3, x_s2, y_s2, lambda3, skew3_select, skew_relation, x3_skew_collision_relation);
    auto [x_s4, y_s4] = add(x4, y4, x_s3, y_s3, lambda4, skew4_select, skew_relation, x4_skew_collision_relation);

    // Validate accumulator output matches SKEW output if q_skew = 1
    // (this is a degree-6 relation)
    std::get<3>(accumulator) += q_skew * (acc_x_shift - x_s4) * scaling_factor;
    std::get<4>(accumulator) += q_skew * (acc_y_shift - y_s4) * scaling_factor;
    std::get<5>(accumulator) += q_skew * skew_relation * scaling_factor;

    // Check x-coordinates do not collide if row is an ADD row or a SKEW row
    // if either q_add or q_skew = 1, an inverse should exist for each computed relation
    // Step 1: construct boolean selectors that describe whether we added a point at the current row
    const auto add_first_point = add1 * q_add + q_skew * skew1_select;
    const auto add_second_point = add2 * q_add + q_skew * skew2_select;
    const auto add_third_point = add3 * q_add + q_skew * skew3_select;
    const auto add_fourth_point = add4 * q_add + q_skew * skew4_select;
    // Step 2: construct the delta between x-coordinates for each point add (depending on if row is ADD or SKEW)
    const auto x1_delta = x1_skew_collision_relation * q_skew + x1_collision_relation * q_add;
    const auto x2_delta = x2_skew_collision_relation * q_skew + x2_collision_relation * q_add;
    const auto x3_delta = x3_skew_collision_relation * q_skew + x3_collision_relation * q_add;
    const auto x4_delta = x4_skew_collision_relation * q_skew + x4_collision_relation * q_add;
    // Step 3: x_delta * inverse - 1 = 0 if we performed a point addition (else x_delta * inverse = 0)
    std::get<6>(accumulator) += (x1_delta * collision_inverse1 - add_first_point) * scaling_factor;
    std::get<7>(accumulator) += (x2_delta * collision_inverse2 - add_second_point) * scaling_factor;
    std::get<8>(accumulator) += (x3_delta * collision_inverse3 - add_third_point) * scaling_factor;
    std::get<9>(accumulator) += (x4_delta * collision_inverse4 - add_fourth_point) * scaling_factor;

    // Validate that if q_add = 1 or q_skew = 1, add1 also is 1
    // TODO(@zac-williamson) Once we have a stable base to work off of, remove q_add1 and replace with q_msm_add +
    // q_msm_skew (issue #2222)
    std::get<32>(accumulator) += (add1 - q_add - q_skew) * scaling_factor;

    // If add_i = 0, slice_i = 0
    // When add_i = 0, force slice_i to ALSO be 0
    std::get<13>(accumulator) += (-add1 + 1) * slice1 * scaling_factor;
    std::get<14>(accumulator) += (-add2 + 1) * slice2 * scaling_factor;
    std::get<15>(accumulator) += (-add3 + 1) * slice3 * scaling_factor;
    std::get<16>(accumulator) += (-add4 + 1) * slice4 * scaling_factor;

    // only one of q_skew, q_double, q_add can be nonzero
    std::get<17>(accumulator) += (q_add * q_double + q_add * q_skew + q_double * q_skew) * scaling_factor;

    // We look up wnaf slices by mapping round + pc -> slice
    // We use an exact set membership check to validate that
    // wnafs written in wnaf_relation == wnafs read in msm relation
    // We use `add1/add2/add3/add4` to flag whether we are performing a wnaf read op
    // We can set these to be Prover-defined as the set membership check implicitly ensures that the correct reads
    // have occurred.
    // if msm_transition = 0, round_shift - round = 0 or 1
    const auto round_delta = round_shift - round;

    // ROUND TRANSITION LOGIC (when round does not change)
    // If msm_transition = 0 (next row) then round_delta = 0 or 1
    const auto round_transition = round_delta * (-msm_transition_shift + 1);
    std::get<18>(accumulator) += round_transition * (round_delta - 1) * scaling_factor;

    // ROUND TRANSITION LOGIC (when round DOES change)
    // round_transition describes whether we are transitioning between rounds of an MSM
    // If round_transition = 1, the next row is either a double (if round != 31) or we are adding skew (if round ==
    // 31) round_transition * skew * (round - 31) = 0 (if round tx and skew, round == 31) round_transition * (skew +
    // double - 1) = 0 (if round tx, skew XOR double = 1) i.e. if round tx and round != 31, double = 1
    std::get<19>(accumulator) += round_transition * q_skew_shift * (round - 31) * scaling_factor;
    std::get<20>(accumulator) += round_transition * (q_skew_shift + q_double_shift - 1) * scaling_factor;

    // if no double or no skew, round_delta = 0
    std::get<21>(accumulator) += round_transition * (-q_double_shift + 1) * (-q_skew_shift + 1) * scaling_factor;

    // if double, next double != 1
    std::get<22>(accumulator) += q_double * q_double_shift * scaling_factor;

    // if double, next add = 1
    std::get<23>(accumulator) += q_double * (-q_add_shift + 1) * scaling_factor;

    // updating count
    // if msm_transition = 0 and round_transition = 0, count_shift = count + add1 + add2 + add3 + add4
    // todo: we need this?
    std::get<24>(accumulator) += (-msm_transition_shift + 1) * (-round_delta + 1) *
                                 (count_shift - count - add1 - add2 - add3 - add4) * scaling_factor;

    std::get<25>(accumulator) +=
        is_not_first_row * (-msm_transition_shift + 1) * round_delta * count_shift * scaling_factor;

    // if msm_transition = 1, count_shift = 0
    std::get<26>(accumulator) += is_not_first_row * msm_transition_shift * count_shift * scaling_factor;

    // if msm_transition = 1, pc = pc_shift + msm_size
    // `ecc_set_relation` ensures `msm_size` maps to `transcript.msm_count` for the current value of `pc`
    std::get<27>(accumulator) += is_not_first_row * msm_transition_shift * (msm_size + pc_shift - pc) * scaling_factor;

    // Addition continuity checks
    // We want to RULE OUT the following scenarios:
    // Case 1: add2 = 1, add1 = 0
    // Case 2: add3 = 1, add2 = 0
    // Case 3: add4 = 1, add3 = 0
    // These checks ensure that the current row does not skip points (for both ADD and SKEW ops)
    // This is part of a wider set of checks we use to ensure that all point data is used in the assigned
    // multiscalar multiplication operation.
    // (and not in a different MSM operation)
    std::get<28>(accumulator) += add2 * (-add1 + 1) * scaling_factor;
    std::get<29>(accumulator) += add3 * (-add2 + 1) * scaling_factor;
    std::get<30>(accumulator) += add4 * (-add3 + 1) * scaling_factor;

    // Final continuity check.
    // If an addition spans two rows, we need to make sure that the following scenario is RULED OUT:
    //   add4 = 0 on the CURRENT row, add1 = 1 on the NEXT row
    // We must apply the above for the two cases:
    // Case 1: q_add = 1 on the CURRENT row, q_add = 1 on the NEXT row
    // Case 2: q_skew = 1 on the CURRENT row, q_skew = 1 on the NEXT row
    // (i.e. if q_skew = 1, q_add_shift = 1 this implies an MSM transition so we skip this continuity check)
    std::get<31>(accumulator) +=
        (q_add * q_add_shift + q_skew * q_skew_shift) * (-add4 + 1) * add1_shift * scaling_factor;

    // remaining checks (done in ecc_set_relation.hpp, ecc_lookup_relation.hpp)
    // when transition occurs, perform set membership lookup on (accumulator / pc / msm_size)
    // perform set membership lookups on add_i * (pc / round / slice_i)
    // perform lookups on (pc / slice_i / x / y)
    }

    fn verify_accumulate(
        _univariate_accumulator: &mut Self::VerifyAcc,
        _input: &crate::prelude::ClaimedEvaluations<F, ECCVMFlavour>,
        _relation_parameters: &crate::prelude::RelationParameters<F, ECCVMFlavour>,
        _scaling_factor: &F,
    ) {
        todo!()
    }
}
