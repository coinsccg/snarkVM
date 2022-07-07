// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::{push_constraints, r1cs_to_qap::R1CStoQAP, Proof, ProvingKey};
use crate::{cfg_into_iter, msm::VariableBaseMSM};
use snarkvm_curves::traits::{AffineCurve, PairingEngine, ProjectiveCurve};
use snarkvm_fields::{One, PrimeField, Zero};
use snarkvm_r1cs::errors::SynthesisError;

use snarkvm_profiler::{end_timer, start_timer};
use snarkvm_r1cs::{ConstraintSynthesizer, ConstraintSystem, Index, LinearCombination, Variable};
use snarkvm_utilities::rand::UniformRand;

use core::ops::Mul;
use rand::Rng;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct ProvingAssignment<E: PairingEngine> {
    // Constraints
    pub(crate) at: Vec<Vec<(E::Fr, Index)>>,
    pub(crate) bt: Vec<Vec<(E::Fr, Index)>>,
    pub(crate) ct: Vec<Vec<(E::Fr, Index)>>,

    // Assignments of variables
    pub(crate) public_variables: Vec<E::Fr>,
    pub(crate) private_variables: Vec<E::Fr>,
}

impl<E: PairingEngine> ConstraintSystem<E::Fr> for ProvingAssignment<E> {
    type Root = Self;

    #[inline]
    fn alloc<F, A, AR>(&mut self, _: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.private_variables.len();
        self.private_variables.push(f()?);
        Ok(Variable::new_unchecked(Index::Private(index)))
    }

    #[inline]
    fn alloc_input<F, A, AR>(&mut self, _: A, f: F) -> Result<Variable, SynthesisError>
    where
        F: FnOnce() -> Result<E::Fr, SynthesisError>,
        A: FnOnce() -> AR,
        AR: AsRef<str>,
    {
        let index = self.public_variables.len();
        self.public_variables.push(f()?);
        Ok(Variable::new_unchecked(Index::Public(index)))
    }

    #[inline]
    fn enforce<A, AR, LA, LB, LC>(&mut self, _: A, a: LA, b: LB, c: LC)
    where
        A: FnOnce() -> AR,
        AR: AsRef<str>,
        LA: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
        LB: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
        LC: FnOnce(LinearCombination<E::Fr>) -> LinearCombination<E::Fr>,
    {
        push_constraints(a(LinearCombination::zero()), &mut self.at);
        push_constraints(b(LinearCombination::zero()), &mut self.bt);
        push_constraints(c(LinearCombination::zero()), &mut self.ct);
    }

    fn push_namespace<NR, N>(&mut self, _: N)
    where
        NR: AsRef<str>,
        N: FnOnce() -> NR,
    {
        // Do nothing; we don't care about namespaces in this context.
    }

    fn pop_namespace(&mut self) {
        // Do nothing; we don't care about namespaces in this context.
    }

    fn get_root(&mut self) -> &mut Self::Root {
        self
    }

    fn num_constraints(&self) -> usize {
        self.at.len()
    }

    fn num_public_variables(&self) -> usize {
        self.public_variables.len()
    }

    fn num_private_variables(&self) -> usize {
        self.private_variables.len()
    }

    fn is_in_setup_mode(&self) -> bool {
        false
    }
}

pub fn create_random_proof<E, C, R>(
    circuit: &C,
    params: &ProvingKey<E>,
    rng: &mut R,
    index: usize
) -> Result<Proof<E>, SynthesisError>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<E::Fr>,
    R: Rng,
{
    let r = E::Fr::rand(rng);
    let s = E::Fr::rand(rng);

    create_proof::<E, C>(circuit, params, r, s, index)
}

pub fn create_proof_no_zk<E, C>(circuit: &C, params: &ProvingKey<E>) -> Result<Proof<E>, SynthesisError>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<E::Fr>,
{
    create_proof::<E, C>(circuit, params, E::Fr::zero(), E::Fr::zero(), 0)
}

pub fn create_proof<E, C>(circuit: &C, params: &ProvingKey<E>, r: E::Fr, s: E::Fr, index: usize) -> Result<Proof<E>, SynthesisError>
where
    E: PairingEngine,
    C: ConstraintSynthesizer<E::Fr>,
{
    let prover_time = start_timer!(|| "Prover");
    let mut prover = ProvingAssignment {
        at: vec![],
        bt: vec![],
        ct: vec![],
        public_variables: vec![],
        private_variables: vec![],
    };

    // Allocate the "one" input variable
    prover.alloc_input(|| "", || Ok(E::Fr::one()))?;

    // Synthesize the circuit.
    let synthesis_time = start_timer!(|| "Constraint synthesis");
    circuit.generate_constraints(&mut prover)?;
    end_timer!(synthesis_time);

    let witness_map_time = start_timer!(|| "R1CS to QAP witness map");
    let h = R1CStoQAP::witness_map::<E>(&prover)?;
    end_timer!(witness_map_time);

    let input_assignment = prover
        .public_variables
        .iter()
        .skip(1)
        .map(|s| s.to_repr())
        .collect::<Vec<_>>();

    let aux_assignment = cfg_into_iter!(prover.private_variables)
        .map(|s| s.to_repr())
        .collect::<Vec<_>>();

    let assignment = [&input_assignment[..], &aux_assignment[..]].concat();

    let h_assignment = cfg_into_iter!(h).map(|s| s.to_repr()).collect::<Vec<_>>();

    // Compute A
    let a_acc_time = start_timer!(|| "Compute A");
    let a_query = &params.a_query;
    let r_g1 = params.delta_g1.mul(r);

    let g_a = calculate_coeff(r_g1.into(), a_query, params.vk.alpha_g1, &assignment);

    end_timer!(a_acc_time);

    let mut pool = snarkvm_utilities::ExecutionPool::<ResultWrapper<E>>::with_capacity(4);
    // Compute B in G1 if needed

    if r != E::Fr::zero() {
        let b_g1_acc_time = start_timer!(|| "Compute B in G1");

        pool.add_job(|| {
            let s_g1 = params.delta_g1.mul(s).into();
            let b_query = &params.b_g1_query;
            let res = calculate_coeff(s_g1, b_query, params.beta_g1, &assignment);
            ResultWrapper::from_g1(res)
        });
        end_timer!(b_g1_acc_time);
    }

    // Compute B in G2
    let b_g2_acc_time = start_timer!(|| "Compute B in G2");
    pool.add_job(|| {
        let b_query = &params.b_g2_query;
        let s_g2 = params.vk.delta_g2.mul(s);
        let res = calculate_coeff(s_g2.into(), b_query, params.vk.beta_g2, &assignment);
        ResultWrapper::from_g2(res)
    });

    end_timer!(b_g2_acc_time);

    // Compute C
    let c_acc_time = start_timer!(|| "Compute C");

    pool.add_job(|| {
        let h_query = &params.h_query;
        let res = VariableBaseMSM::multi_scalar_mul(h_query, &h_assignment, index);
        ResultWrapper::from_g1(res)
    });

    pool.add_job(|| {
        let l_aux_source = &params.l_query;
        let res = VariableBaseMSM::multi_scalar_mul(l_aux_source, &aux_assignment, index);
        ResultWrapper::from_g1(res)
    });
    let results: Vec<_> = pool.execute_all();

    let g1_b = if r != E::Fr::zero() {
        results[0].into_g1()
    } else {
        E::G1Projective::zero()
    };
    let g2_b = results[1].into_g2();
    let h_acc = results[2].into_g1();
    let l_aux_acc = results[3].into_g1();

    let s_g_a = g_a.mul(s);
    let r_g1_b = g1_b.mul(r);
    let r_s_delta_g1 = params.delta_g1.into_projective().mul(r).mul(s);

    let mut g_c = s_g_a;
    g_c += r_g1_b;
    g_c -= &r_s_delta_g1;
    g_c += l_aux_acc;
    g_c += h_acc;
    end_timer!(c_acc_time);

    end_timer!(prover_time);

    Ok(Proof {
        a: g_a.into_affine(),
        b: g2_b.into_affine(),
        c: g_c.into_affine(),
        compressed: true,
    })
}

fn calculate_coeff<G: AffineCurve>(
    initial: G::Projective,
    query: &[G],
    vk_param: G,
    assignment: &[<G::ScalarField as PrimeField>::BigInteger],
) -> G::Projective {
    let el = query[0];
    let acc = VariableBaseMSM::multi_scalar_mul(&query[1..], assignment, 0);

    let mut res = initial;
    res.add_assign_mixed(&el);
    res += acc;
    res.add_assign_mixed(&vk_param);

    res
}

#[derive(derivative::Derivative)]
#[derivative(Copy(bound = ""), Clone(bound = ""))]
enum ResultWrapper<E: PairingEngine> {
    G1(E::G1Projective),
    G2(E::G2Projective),
}

impl<E: PairingEngine> ResultWrapper<E> {
    fn from_g1(g: E::G1Projective) -> Self {
        Self::G1(g)
    }

    fn from_g2(g: E::G2Projective) -> Self {
        Self::G2(g)
    }

    fn into_g1(self) -> E::G1Projective {
        match self {
            Self::G1(g) => g,
            _ => panic!("could not unwrap g2 into g1"),
        }
    }

    fn into_g2(self) -> E::G2Projective {
        match self {
            Self::G2(g) => g,
            _ => panic!("could not unwrap g1 into g2"),
        }
    }
}
