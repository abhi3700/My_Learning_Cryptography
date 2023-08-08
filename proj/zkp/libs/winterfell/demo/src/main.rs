use std::time::Instant;

use winterfell::{
    crypto::{hashers::Blake3_256, DefaultRandomCoin},
    math::{fields::f128::BaseElement as Felt, FieldElement, ToElements},
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, ProofOptions, Prover, Serializable,
    StarkProof, Trace, TraceInfo, TraceTable, TransitionConstraintDegree,
};

const ALPHA: u64 = 3;
const INV_ALPHA: u128 = 226854911280625642308916371969163307691;
const FORTY_TWO: Felt = Felt::new(42);

fn main() {
    let n: usize = 1024 * 1024;
    let seed = Felt::new(5);

    // compute result
    let now = Instant::now();
    // last value (n-1, n-1) of the table. In case of single column, it's (0, n-1) cell value
    let result = vdf(seed, n);
    println!("Computed result in {:} ms", now.elapsed().as_millis());

    // specify parameters for the STARK protocol
    let stark_params = ProofOptions::new(40, 4, 21, winterfell::FieldExtension::None, 8, 64);

    // instantiate the prover
    let prover = VdfProver::new(stark_params);

    // build execution trace
    let now = Instant::now();
    let trace = VdfProver::build_trace(seed, n);

    println!("Built execution trace in {} ms", now.elapsed().as_millis());
    assert_eq!(result, trace.get(0, n - 1)); // assertion

    // generate the proof
    let now = Instant::now();
    let proof = prover.prove(trace).unwrap();
    println!("Generated proof in {} ms", now.elapsed().as_millis());

    // serialize proof and check security level
    let proof_bytes = proof.to_bytes();
    println!("Proof size: {} KB", proof_bytes.len() as f64 / 1024_f64);
    println!(
        "Proof security: {} bits",
        proof.security_level::<Blake3_256<Felt>>(true)
    );

    // deserialize the proof
    let parsed_proof = StarkProof::from_bytes(proof_bytes.as_slice()).unwrap();
    assert_eq!(parsed_proof, proof);

    // get the public inputs
    let now = Instant::now();
    let pub_inputs = prover.get_pub_inputs(&trace);

    // verify the proof
    match winterfell::verify::<VdfAir, Blake3_256<Felt>, DefaultRandomCoin<Blake3_256<Felt>>>(
        proof, pub_inputs,
    ) {
        Ok(_) => println!("Proof verified in {:.1} ms", now.elapsed().as_millis()),
        Err(msg) => println!("{}", msg),
    }
}

// VDF Function
// ============
fn vdf(seed: Felt, n: usize) -> Felt {
    let mut state = seed;

    for _ in 0..n {
        state = (state - FORTY_TWO).exp(INV_ALPHA)
    }

    state
}

// Public inputs
// ============
#[derive()]
struct VdfInputs {
    seed: Felt,
    result: Felt,
}

impl Serializable for VdfInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.seed);
        target.write(self.result);
    }
}

impl ToElements<Felt> for VdfInputs {
    fn to_elements(&self) -> Vec<Felt> {
        vec![self.result]
    }
}

// VDF Air
// ============
struct VdfAir {
    context: AirContext<Felt>,
    seed: Felt,
    result: Felt,
}

impl Air for VdfAir {
    type BaseField = Felt;
    type PublicInputs = VdfInputs;

    fn new(trace_info: TraceInfo, pub_inputs: VdfInputs, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(3)];
        let context = AirContext::new(trace_info, degrees, 4, options);
        VdfAir {
            context,
            seed: pub_inputs.seed,
            result: pub_inputs.result,
        }
    }

    fn evaluate_transition<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        periodic_values: &[E],
        result: &mut [E],
    ) {
        let current_state = frame.current()[0];
        let next_state = frame.next()[0];

        result[0] = current_state - (next_state.exp(ALPHA.into()) + FORTY_TWO.into());
    }

    fn get_assertions(&self) -> Vec<Assertion<Felt>> {
        let last_step = self.trace_length() - 1;

        // winterfell converts the assertions to boundary constraints
        vec![
            Assertion::single(0, 0, self.seed),
            Assertion::single(0, last_step, self.result),
        ]
    }

    fn context(&self) -> &AirContext<Felt> {
        &self.context
    }
}

// Prover
// ======
struct VdfProver {
    options: ProofOptions,
}

impl VdfProver {
    fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    fn build_trace(seed: Felt, n: usize) -> TraceTable<Felt> {
        let mut trace = Vec::with_capacity(n);

        // populate the trace vector with different states.
        let mut state = seed;
        trace.push(state);

        for _ in 0..n {
            state = (state - FORTY_TWO).exp(INV_ALPHA);
            trace.push(state);
        }

        // Here, we have just one column with multiple rows.
        // NOTE: ideally, there are multiple columns & rows, then, it would have been
        // `TraceTable::init(vec![trace0, trace1, trace2, ...tracen])`
        TraceTable::init(vec![trace])
    }
}

impl Prover for VdfProver {
    type BaseField = Felt;
    type Air = VdfAir;
    type HashFn = Blake3_256<Felt>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type Trace = TraceTable<Felt>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> VdfInputs {
        let last_step = trace.length() - 1;

        VdfInputs {
            seed: trace.get(0, 0),
            result: trace.get(0, last_step),
        }
    }
    fn options(&self) -> &ProofOptions {
        &self.options()
    }
}
