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

use criterion::Criterion;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use snarkvm_algorithms::msm::*;
use snarkvm_curves::{
    bls12_377::{Fr, G1Projective},
    traits::ProjectiveCurve,
};
use snarkvm_fields::PrimeField;
use snarkvm_utilities::rand::UniformRand;

#[macro_use]
extern crate criterion;

fn variable_base(c: &mut Criterion) {
    const SAMPLES: usize = 200000;

    let mut rng = XorShiftRng::seed_from_u64(234872845u64);

    let v = (0..SAMPLES).map(|_| Fr::rand(&mut rng).to_repr()).collect::<Vec<_>>();
    let g = (0..SAMPLES)
        .map(|_| G1Projective::rand(&mut rng).into_affine())
        .collect::<Vec<_>>();

    c.bench_function("MSM Variable Base", move |b| {
        b.iter(|| {
            let mut task = Vec::new();
            for _ in 0..20 {
                let mut g1 = Vec::with_capacity(SAMPLES);
                unsafe {
                    g1.set_len(SAMPLES);
                }
                let v1 = v.clone();
                g1.clone_from_slice(&g[..]);
                let handle = std::thread::spawn(move || {
                    VariableBaseMSM::multi_scalar_mul(g1.as_slice(), v1.as_slice(), 0);
                });
                task.push(handle);
            }

            for t in task {
                t.join().unwrap();
            }

        })
    });
}

criterion_group! {
    name = variable_base_group;
    config = Criterion::default().sample_size(10);
    targets = variable_base
}

criterion_main!(variable_base_group);
