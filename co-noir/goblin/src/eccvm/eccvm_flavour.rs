// trait ECCVMPolynomialFlavour {
//     type ProverWitnessEntities<T: Default + Sync + Clone>;
//     type PrecomputedEntities<T: Default + Clone + Sync>;

// use co_builder::{
//     flavours::eccvm_flavour::{ECCVMFlavour, ECCVMProverWitnessEntities, ECCVMWitnessEntities},
//     prover_flavour::ProverFlavour,
// };

// pub trait ECCVMWitnessEntitiesFlavour<T: Default> {
//     fn get_wire_non_shifted_entities(&self) -> &[T];
//     fn get_wire_non_shifted_entities_mut(&mut self) -> &mut [T];
//     fn get_wire_to_be_shifted_without_accumulators_entities(&self) -> &[T];
//     fn get_wire_to_be_shifted_without_accumulators_entities_mut(&mut self) -> &mut [T];
//     fn get_wire_non_shifted_entities_labels() -> &'static [&'static str];
//     fn get_wire_to_be_shifted_without_accumulators_entities_labels() -> &'static [&'static str];
// }
// impl<T: Default + std::marker::Sync> ECCVMWitnessEntitiesFlavour<T>
//     for <ECCVMFlavour as ProverFlavour>::ProverWitnessEntities<T>
// {
//     fn get_wire_non_shifted_entities(&self) -> &[T] {
//         todo!()
//     }

//     fn get_wire_non_shifted_entities_mut(&mut self) -> &mut [T] {
//         todo!()
//     }

//     fn get_wire_to_be_shifted_without_accumulators_entities(&self) -> &[T] {
//         todo!()
//     }

//     fn get_wire_to_be_shifted_without_accumulators_entities_mut(&mut self) -> &mut [T] {
//         todo!()
//     }

//     fn get_wire_non_shifted_entities_labels() -> &'static [&'static str] {
//         todo!()
//     }

//     fn get_wire_to_be_shifted_without_accumulators_entities_labels() -> &'static [&'static str] {
//         todo!()
//     }
// }
// pub trait ECCVMPlainProverFlavour {
//     type WitnessEntities<T: Default + std::marker::Sync>: ECCVMWitnessEntitiesFlavour<T>
//         + Default
//         + std::marker::Sync;
// }
// impl ECCVMPlainProverFlavour for ECCVMFlavour {
//     type WitnessEntities<T: Default + std::marker::Sync> = ECCVMProverWitnessEntities<T>;
// }
// fn non_shifted(&self) -> &[T] {
//     panic!("This should not be called with this Flavour");
// }

// fn non_shifted_mut(&mut self) -> &mut [T] {
//     panic!("This should not be called with this Flavour");
// }

// fn to_be_shifted_without_accumulators(&self) -> &[T] {
//     panic!("This should not be called with this Flavour");
// }

// fn get_wire_to_be_shifted_without_accumulators_entities_mut(&mut self) -> &mut [T] {
//     panic!("This should not be called with this Flavour");
// }

// fn non_shifted_labels() -> &'static [&'static str] {
//     panic!("This should not be called with this Flavour");
// }

// fn to_be_shifted_without_accumulators_labels() -> &'static [&'static str] {
//     panic!("This should not be called with this Flavour");
// }
