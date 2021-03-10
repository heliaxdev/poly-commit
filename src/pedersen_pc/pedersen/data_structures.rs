use ark_ec::AffineCurve;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, SerializationError};
use ark_std::io::{Read, Write};
use ark_std::vec::Vec;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct UniversalParams<G: AffineCurve> {
    pub(crate) generators: Vec<G>,
    pub(crate) hiding_generator: G,
}

#[derive(Clone, Debug, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
pub struct CommitterKey<G: AffineCurve> {
    pub(crate) generators: Vec<G>,
    pub(crate) hiding_generator: G,
    pub(crate) max_elems: usize,
}

impl<G: AffineCurve> CommitterKey<G> {
    pub fn supported_elems_len(&self) -> usize {
        self.generators.len()
    }

    pub fn max_elems_len(&self) -> usize {
        self.max_elems
    }
}