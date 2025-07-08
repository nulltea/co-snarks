use std::marker::PhantomData;

use ark_ec::pairing::Pairing;
use ark_ff::Field;
use co_builder::{
    TranscriptFieldType,
    prelude::{HonkCurve, NUM_DISABLED_ROWS_IN_SUMCHECK, ProverCrs},
};
use rand_chacha::ChaCha12Rng;
use ultrahonk::{
    Utils,
    plain_prover_flavour::PlainProverFlavour,
    prelude::{
        Decider, ProvingKey, SmallSubgroupIPAProver, SumcheckOutput, Transcript, TranscriptHasher,
        ZKSumcheckData, ZeroKnowledge,
    },
};

//TODO FLORIN MOVE THIS SOMEWHERE ELSE LATER
const CONST_ECCVM_LOG_N: usize = 16;
const NUM_RELATIONS: usize = 7;

struct ECCVM<
    P: HonkCurve<TranscriptFieldType>,
    H: TranscriptHasher<TranscriptFieldType>,
    L: PlainProverFlavour,
> {
    decider: Decider<P, H, L>,
}

impl<
    P: HonkCurve<TranscriptFieldType>,
    H: TranscriptHasher<TranscriptFieldType>,
    L: PlainProverFlavour,
> ECCVM<P, H, L>
{
    fn construct_proof(
        &mut self,
        transcript: &mut Transcript<TranscriptFieldType, H>,
        proving_key: &mut ProvingKey<P, L>,
        crs: &ProverCrs<P>,
    ) {
        self.execute_wire_commitments_round(transcript, proving_key);
        self.execute_log_derivative_commitments_round(transcript);
        self.execute_grand_product_computation_round();
        self.execute_relation_check_rounds(transcript, crs, proving_key.circuit_size);
        //TODO FLORIN REMOVE UNWRAP
        // self.execute_pcs_rounds();

        // return export_proof();
    }

    fn execute_wire_commitments_round(
        &mut self,
        transcript: &mut Transcript<TranscriptFieldType, H>,
        proving_key: &mut ProvingKey<P, L>,
    ) {
        // // To commit to the masked wires when `real_size` < `circuit_size`, we use
        // // `commit_structured` that ignores 0 coefficients between the real size and the last NUM_DISABLED_ROWS_IN_SUMCHECK
        // // wire entries.
        let unmasked_witness_size = proving_key.circuit_size - NUM_DISABLED_ROWS_IN_SUMCHECK;

        // CommitmentKey::CommitType commit_type =
        //     (circuit_size > key->real_size) ? CommitmentKey::CommitType::Structured : CommitmentKey::CommitType::Default;

        // // Commit to wires whose length is bounded by the real size of the ECCVM
        // for (const auto& [wire, label] : zip_view(key->polynomials.get_wires_without_accumulators(),
        //                                           commitment_labels.get_wires_without_accumulators())) {
        //     // AZTEC TODO(https://github.com/AztecProtocol/barretenberg/issues/1240) Structured Polynomials in
        //     // ECCVM/Translator/MegaZK
        //     const size_t start = circuit_size == wire.size() ? 0 : 1;
        //     std::vector<std::pair<size_t, size_t>> active_ranges{ { start, key->real_size + start },
        //                                                           { unmasked_witness_size, circuit_size } };
        //     commit_to_witness_polynomial(wire, label, commit_type, active_ranges);
        // }

        // // The accumulators are populated until the 2^{CONST_ECCVM_LOG_N}, therefore we commit to a full-sized polynomial
        // for (const auto& [wire, label] :
        //      zip_view(key->polynomials.get_accumulators(), commitment_labels.get_accumulators())) {
        //     commit_to_witness_polynomial(wire, label);
        // }
    }

    fn execute_log_derivative_commitments_round(
        &mut self,
        transcript: &mut Transcript<TranscriptFieldType, H>,
    ) {
        // // Compute and add beta to relation parameters
        let challs = transcript.get_challenges::<P>(&["BETA".to_string(), "GAMMA".to_string()]);
        let beta = challs[0];
        let gamma = challs[1];
        // // AZTEC TODO(#583)(@zac-williamson): fix Transcript to be able to generate more than 2 challenges per round! oof.
        let beta_sqr = beta * beta;
        self.decider.memory.relation_parameters.gamma = gamma;
        self.decider.memory.relation_parameters.beta = beta;
        self.decider.memory.relation_parameters.beta_sqr = beta_sqr;
        self.decider.memory.relation_parameters.beta_cube = beta_sqr * beta;
        self.decider
            .memory
            .relation_parameters
            .eccvm_set_permutation_delta = gamma
            * (gamma + beta_sqr)
            * (gamma + beta_sqr + beta_sqr)
            * (gamma + beta_sqr + beta_sqr + beta_sqr);
        self.decider
            .memory
            .relation_parameters
            .eccvm_set_permutation_delta = self
            .decider
            .memory
            .relation_parameters
            .eccvm_set_permutation_delta
            .inverse()
            .expect("Challenge should be non-zero");

        // TODO FLORIN
        // // Compute inverse polynomial for our logarithmic-derivative lookup method
        // compute_logderivative_inverse<typename Flavor::FF, typename Flavor::LookupRelation>(
        //     key->polynomials, relation_parameters, unmasked_witness_size);
        // commit_to_witness_polynomial(key->polynomials.lookup_inverses, commitment_labels.lookup_inverses);
    }

    fn execute_grand_product_computation_round(&mut self) {
        // // Compute permutation grand product and their commitments
        // compute_grand_products<Flavor>(key->polynomials, relation_parameters, unmasked_witness_size);
        // commit_to_witness_polynomial(key->polynomials.z_perm, commitment_labels.z_perm);
    }

    fn execute_relation_check_rounds(
        &mut self,
        transcript: &mut Transcript<TranscriptFieldType, H>,
        crs: &ProverCrs<P>,
        circuit_size: u32,
    ) {
        // using Sumcheck = SumcheckProver<Flavor, CONST_ECCVM_LOG_N>;

        // Sumcheck sumcheck(key->circuit_size, transcript);
        let alpha = transcript.get_challenge::<P>("Sumcheck:alpha".to_string());
        let mut gate_challenges: Vec<<P as Pairing>::ScalarField> =
            Vec::with_capacity(CONST_ECCVM_LOG_N);

        for idx in 0..CONST_ECCVM_LOG_N {
            let chall = transcript.get_challenge::<P>(format!("Sumcheck:gate_challenge_{idx}"));
            gate_challenges.push(chall);
        }
        let log_subgroup_size = Utils::get_msb64(P::SUBGROUP_SIZE as u64);
        let commitment_key = &crs.monomials[..1 << (log_subgroup_size + 1)];
        let mut zk_sumcheck_data: ZKSumcheckData<P> = ZKSumcheckData::<P>::new::<H, _>(
            Utils::get_msb64(circuit_size as u64) as usize,
            transcript,
            commitment_key,
            &mut self.decider.rng,
        )
        .unwrap(); //TODO FLORIN REMOVE UNWRAP

        let sumcheck_output = (
            self.decider
                .sumcheck_prove_zk(transcript, circuit_size, &mut zk_sumcheck_data),
            Some(zk_sumcheck_data),
        );
    }

    fn execute_pcs_rounds(
        &mut self,
        sumcheck_output: SumcheckOutput<P::ScalarField, L>,
        zk_sumcheck_data: ZKSumcheckData<P>,
        transcript: &mut Transcript<TranscriptFieldType, H>,
        crs: &ProverCrs<P>,
        circuit_size: u32,
    ) {
        let small_subgroup_ipa_prover = SmallSubgroupIPAProver::<_>::new::<H, _>(
            zk_sumcheck_data,
            &sumcheck_output.challenges,
            sumcheck_output
                .claimed_libra_evaluation
                .expect("We have ZK"),
            transcript,
            crs,
            &mut self.decider.rng,
        )
        .unwrap(); //TODO FLORIN REMOVE UNWRAP
        let witness_polynomials = small_subgroup_ipa_prover.into_witness_polynomials();
        let prover_opening_claim = self
            .decider
            .shplemini_prove(
                transcript,
                circuit_size,
                crs,
                sumcheck_output,
                Some(witness_polynomials),
            )
            .unwrap(); //TODO FLORIN REMOVE UNWRAP

        // using Curve = typename Flavor::Curve;
        // using Shplemini = ShpleminiProver_<Curve>;
        // using Shplonk = ShplonkProver_<Curve>;
        // using OpeningClaim = ProverOpeningClaim<Curve>;
        // using PolynomialBatcher = GeminiProver_<Curve>::PolynomialBatcher;

        // SmallSubgroupIPA small_subgroup_ipa_prover(zk_sumcheck_data,
        //                                            sumcheck_output.challenge,
        //                                            sumcheck_output.claimed_libra_evaluation,
        //                                            transcript,
        //                                            key->commitment_key);
        // small_subgroup_ipa_prover.prove();

        // // Execute the Shplemini (Gemini + Shplonk) protocol to produce a univariate opening claim for the multilinear
        // // evaluations produced by Sumcheck
        // PolynomialBatcher polynomial_batcher(key->circuit_size);
        // polynomial_batcher.set_unshifted(key->polynomials.get_unshifted());
        // polynomial_batcher.set_to_be_shifted_by_one(key->polynomials.get_to_be_shifted());

        // OpeningClaim multivariate_to_univariate_opening_claim =
        //     Shplemini::prove(key->circuit_size,
        //                      polynomial_batcher,
        //                      sumcheck_output.challenge,
        //                      key->commitment_key,
        //                      transcript,
        //                      small_subgroup_ipa_prover.get_witness_polynomials(),
        //                      sumcheck_output.round_univariates,
        //                      sumcheck_output.round_univariate_evaluations);

        // ECCVMProver::compute_translation_opening_claims();

        // opening_claims.back() = std::move(multivariate_to_univariate_opening_claim);

        // // Reduce the opening claims to a single opening claim via Shplonk
        // const OpeningClaim batch_opening_claim = Shplonk::prove(key->commitment_key, opening_claims, transcript);

        // // Compute the opening proof for the batched opening claim with the univariate PCS
        // PCS::compute_opening_proof(key->commitment_key, batch_opening_claim, ipa_transcript);
    }
}
