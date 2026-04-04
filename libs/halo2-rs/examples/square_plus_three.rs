use halo2_proofs::poly::kzg::multiopen::{ProverSHPLONK, VerifierSHPLONK};
use halo2_proofs::poly::kzg::strategy::SingleStrategy;
use halo2_proofs::transcript::{Blake2bRead, TranscriptReadBuffer};
use halo2_proofs::{
    circuit::{SimpleFloorPlanner, Value},
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{self, keygen_pk, keygen_vk, Advice, Circuit, Column, Instance, Selector},
    poly::{commitment::ParamsProver, kzg::commitment::ParamsKZG, Rotation},
    transcript::{Blake2bWrite, Challenge255, TranscriptWriterBuffer},
};
use rand_core::OsRng;

#[derive(Debug, Clone)]
struct SquarePlusThreeConfig {
    x: Column<Advice>,
    t: Column<Advice>,
    y: Column<Advice>,
    q_enable: Selector,
    instance: Column<Instance>,
}

#[derive(Default)]
struct SquarePlusThreeCircuit {
    /// private witness
    x: Value<Fr>,
}

impl Circuit<Fr> for SquarePlusThreeCircuit {
    type Config = SquarePlusThreeConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self { x: Value::unknown() }
    }

    fn configure(meta: &mut halo2_proofs::plonk::ConstraintSystem<Fr>) -> Self::Config {
        let x = meta.advice_column();
        let t = meta.advice_column();
        let y = meta.advice_column();
        let instance = meta.instance_column();
        let q_enable = meta.selector();

        meta.enable_equality(x);
        meta.enable_equality(y);
        meta.enable_equality(instance);

        // Enforce: y = x*x + 3
        meta.create_gate("square plus three", |meta| {
            let q = meta.query_selector(q_enable);
            let x_expr = meta.query_advice(x, Rotation::cur());
            let t_expr = meta.query_advice(t, Rotation::cur());
            let y_expr = meta.query_advice(y, Rotation::cur());

            vec![
                q.clone() * (x_expr.clone() * x_expr - t_expr.clone()),
                q * (t_expr + plonk::Expression::Constant(Fr::from(3)) - y_expr),
            ]
        });

        SquarePlusThreeConfig { x, t, y, q_enable, instance }
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl halo2_proofs::circuit::Layouter<Fr>,
    ) -> Result<(), halo2_proofs::plonk::ErrorFront> {
        let y_cell = layouter.assign_region(
            || "compute y = x^2 + 3",
            |mut region| {
                config.q_enable.enable(&mut region, 0)?;

                let x_val = self.x;
                let t_val = x_val.map(|v| v * v);
                let y_val = t_val.map(|v| v + Fr::from(3));

                region.assign_advice(|| "x", config.x, 0, || x_val)?;
                region.assign_advice(|| "t", config.t, 0, || t_val)?;
                let y_cell = region.assign_advice(|| "y", config.y, 0, || y_val)?;

                Ok(y_cell)
            },
        )?;

        layouter.constrain_instance(y_cell.cell(), config.instance, 0)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::dev::MockProver;

    #[test]
    fn test_square_plus_three() {
        let x = Fr::from(5);
        let y = x.square() + Fr::from(3);

        let circuit = SquarePlusThreeCircuit { x: Value::known(x) };

        let public_inputs = vec![vec![y]];
        let k = 4;

        let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
        prover.assert_satisfied();
    }

    #[test]
    fn test_invalid() {
        let x = Fr::from(5);
        let wrong_y = x.square() + Fr::from(999);

        let circuit = SquarePlusThreeCircuit { x: Value::known(x) };

        let public_inputs = vec![vec![wrong_y]];
        let k = 4;

        let prover = MockProver::run(k, &circuit, public_inputs).unwrap();
        assert!(prover.verify().is_err());
    }
}

fn main() {
    // params
    let k = 4;
    let params: ParamsKZG<Bn256> = ParamsKZG::new(k);

    // keygen with empty circuit
    let empty_circuit = SquarePlusThreeCircuit::default();
    let vk = keygen_vk(&params, &empty_circuit).expect("vk");
    let pk = keygen_pk(&params, vk, &empty_circuit).expect("pk");

    let x = Fr::from(5);
    let y = x.square() + Fr::from(3);
    let circuit = SquarePlusThreeCircuit { x: Value::known(x) };

    let mut rng = OsRng;
    let public_inputs = vec![vec![y]];

    let mut transcript =
        Blake2bWrite::<Vec<u8>, G1Affine, Challenge255<G1Affine>>::init(Vec::new());

    halo2_proofs::plonk::create_proof::<
        halo2_proofs::poly::kzg::commitment::KZGCommitmentScheme<Bn256>,
        ProverSHPLONK<'_, Bn256>,
        _,
        _,
        _,
        _,
    >(&params, &pk, &[circuit], &[public_inputs.clone()], &mut rng, &mut transcript)
    .expect("proof generation should succeed");

    let proof = transcript.finalize();

    std::fs::write("square_plus_three.proof", &proof).expect("should write proof file");

    println!("Proof generated: {} bytes", proof.len());
    println!("Saved to: square_plus_three.proof");
    println!("Proof bytes: {:02x?}", proof);

    // ===== verification

    let proof_from_file = std::fs::read("square_plus_three.proof").expect("should read proof file");
    let mut verifier_transcript =
        Blake2bRead::<_, G1Affine, Challenge255<G1Affine>>::init(&proof_from_file[..]);
    let strategy = SingleStrategy::new(&params.verifier_params());

    halo2_proofs::plonk::verify_proof::<
        halo2_proofs::poly::kzg::commitment::KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<Bn256>,
        _,
        _,
        _,
    >(
        &params.verifier_params(), pk.get_vk(), strategy, &[public_inputs], &mut verifier_transcript
    )
    .expect("proof verification should succeed");

    println!("Proof verification succeeded");
}
