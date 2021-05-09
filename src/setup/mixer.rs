use crate::{
	arbitrary::mixer_data::{constraints::MixerDataGadget, Input as MixerDataInput, MixerData},
	circuit::mixer_circuit::MixerCircuit,
	leaf::{
		bridge::{
			constraints::BridgeLeafGadget, BridgeLeaf, Private as LeafPrivate, Public as LeafPublic,
		},
		LeafCreation,
	},
	merkle_tree::{Config as MerkleConfig, Path, SparseMerkleTree},
	set::membership::{constraints::SetMembershipGadget, SetMembership},
	test_data::{get_mds_3, get_mds_5, get_rounds_3, get_rounds_5},
};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::fields::PrimeField;
use ark_std::{rand::Rng, rc::Rc};
use webb_crypto_primitives::{
	crh::poseidon::{constraints::CRHGadget, sbox::PoseidonSbox, PoseidonParameters, Rounds, CRH},
	SNARK,
};

type MixerConstraintData = MixerData<BlsFr>;
type MixerConstraintDataGadget = MixerDataGadget<BlsFr>;
#[derive(Default, Clone)]
struct PoseidonRounds5;

impl Rounds for PoseidonRounds5 {
	const FULL_ROUNDS: usize = 8;
	const PARTIAL_ROUNDS: usize = 57;
	const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
	const WIDTH: usize = 5;
}

type PoseidonCRH5 = CRH<BlsFr, PoseidonRounds5>;
type PoseidonCRH5Gadget = CRHGadget<BlsFr, PoseidonRounds5>;

#[derive(Default, Clone)]
struct PoseidonRounds3;

impl Rounds for PoseidonRounds3 {
	const FULL_ROUNDS: usize = 8;
	const PARTIAL_ROUNDS: usize = 57;
	const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
	const WIDTH: usize = 3;
}

type PoseidonCRH3 = CRH<BlsFr, PoseidonRounds3>;
type PoseidonCRH3Gadget = CRHGadget<BlsFr, PoseidonRounds3>;

type Leaf = BridgeLeaf<BlsFr, PoseidonCRH5>;
type LeafGadget = BridgeLeafGadget<BlsFr, PoseidonCRH5, PoseidonCRH5Gadget, Leaf>;

#[derive(Clone)]
struct MixerTreeConfig;
impl MerkleConfig for MixerTreeConfig {
	type H = PoseidonCRH3;
	type LeafH = PoseidonCRH3;

	const HEIGHT: u8 = 30;
}

type MixerTree = SparseMerkleTree<MixerTreeConfig>;

type TestSetMembership = SetMembership<BlsFr>;
type TestSetMembershipGadget = SetMembershipGadget<BlsFr>;

type Circuit = MixerCircuit<
	BlsFr,
	MixerConstraintData,
	MixerConstraintDataGadget,
	PoseidonCRH5,
	PoseidonCRH5Gadget,
	MixerTreeConfig,
	PoseidonCRH3Gadget,
	PoseidonCRH3Gadget,
	Leaf,
	LeafGadget,
	TestSetMembership,
	TestSetMembershipGadget,
>;

fn setup_params_3<F: PrimeField>() -> PoseidonParameters<F> {
	// Making params for poseidon in merkle tree
	let rounds3 = get_rounds_3::<F>();
	let mds3 = get_mds_3::<F>();
	let params3 = PoseidonParameters::<F>::new(rounds3, mds3);
	params3
}

fn setup_params_5<F: PrimeField>() -> PoseidonParameters<F> {
	// Round params for the poseidon in leaf creation gadget
	let rounds5 = get_rounds_5::<F>();
	let mds5 = get_mds_5::<F>();
	let params5 = PoseidonParameters::<F>::new(rounds5, mds5);
	params5
}

fn setup_leaf<R: Rng>(
	chain_id: BlsFr,
	params: &PoseidonParameters<BlsFr>,
	rng: &mut R,
) -> (
	LeafPrivate<BlsFr>,
	LeafPublic<BlsFr>,
	<Leaf as LeafCreation<PoseidonCRH5>>::Leaf,
	<Leaf as LeafCreation<PoseidonCRH5>>::Nullifier,
) {
	// Secret inputs for the leaf
	let leaf_private = Leaf::generate_secrets(rng).unwrap();
	// Public inputs for the leaf
	let leaf_public = LeafPublic::new(chain_id);

	// Creating the leaf
	let leaf = Leaf::create_leaf(&leaf_private, &leaf_public, params).unwrap();
	let nullifier_hash = Leaf::create_nullifier(&leaf_private, params).unwrap();
	(leaf_private, leaf_public, leaf, nullifier_hash)
}

fn setup_tree_and_create_path(
	leaves: &[BlsFr],
	index: u64,
	params: &PoseidonParameters<BlsFr>,
) -> (BlsFr, Path<MixerTreeConfig>) {
	// Making the merkle tree
	let inner_params = Rc::new(params.clone());
	let leaf_params = inner_params.clone();
	let mt = MixerTree::new_sequential(inner_params, leaf_params, leaves).unwrap();
	// Getting the proof path
	let path = mt.generate_membership_proof(index);
	let root = mt.root();
	(root.inner(), path)
}

#[macro_export]
macro_rules! setup_types {
	($test_field:ty) => {
		type MixerConstraintData = MixerData<$test_field>;
		type MixerConstraintDataGadget = MixerDataGadget<$test_field>;
		#[derive(Default, Clone)]
		struct PoseidonRounds5;

		impl Rounds for PoseidonRounds5 {
			const FULL_ROUNDS: usize = 8;
			const PARTIAL_ROUNDS: usize = 57;
			const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
			const WIDTH: usize = 5;
		}

		type PoseidonCRH5 = CRH<$test_field, PoseidonRounds5>;
		type PoseidonCRH5Gadget = CRHGadget<$test_field, PoseidonRounds5>;

		#[derive(Default, Clone)]
		struct PoseidonRounds3;

		impl Rounds for PoseidonRounds3 {
			const FULL_ROUNDS: usize = 8;
			const PARTIAL_ROUNDS: usize = 57;
			const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
			const WIDTH: usize = 3;
		}

		type PoseidonCRH3 = CRH<$test_field, PoseidonRounds3>;
		type PoseidonCRH3Gadget = CRHGadget<$test_field, PoseidonRounds3>;

		type Leaf = BridgeLeaf<$test_field, PoseidonCRH5>;
		type LeafGadget = BridgeLeafGadget<$test_field, PoseidonCRH5, PoseidonCRH5Gadget, Leaf>;

		#[derive(Clone)]
		struct MixerTreeConfig;
		impl MerkleConfig for MixerTreeConfig {
			type H = PoseidonCRH3;
			type LeafH = PoseidonCRH3;

			const HEIGHT: u8 = 10;
		}

		type MixerTree = SparseMerkleTree<MixerTreeConfig>;

		type TestSetMembership = SetMembership<$test_field>;
		type TestSetMembershipGadget = SetMembershipGadget<$test_field>;

		type Circuit = MixerCircuit<
			$test_field,
			MixerConstraintData,
			MixerConstraintDataGadget,
			PoseidonCRH5,
			PoseidonCRH5Gadget,
			MixerTreeConfig,
			PoseidonCRH3Gadget,
			PoseidonCRH3Gadget,
			Leaf,
			LeafGadget,
			TestSetMembership,
			TestSetMembershipGadget,
		>;
	};
}

#[macro_export]
macro_rules! setup_params_3 {
	($test_field:ty) => {{
		// Making params for poseidon in merkle tree
		let rounds3 = get_rounds_3::<$test_field>();
		let mds3 = get_mds_3::<$test_field>();
		let params3 = PoseidonParameters::<$test_field>::new(rounds3, mds3);
		params3
	}};
}

#[macro_export]
macro_rules! setup_params_5 {
	($test_field:ty) => {{
		// Round params for the poseidon in leaf creation gadget
		let rounds5 = get_rounds_5::<$test_field>();
		let mds5 = get_mds_5::<$test_field>();
		let params5 = PoseidonParameters::<$test_field>::new(rounds5, mds5);
		params5
	}};
}

#[macro_export]
macro_rules! setup_leaf {
	($test_field:ty, $params:expr) => {{
		let rng = &mut test_rng();

		// Secret inputs for the leaf
		let leaf_private = Leaf::generate_secrets(rng).unwrap();
		// Public inputs for the leaf
		let chain_id = <$test_field>::one();
		let leaf_public = LeafPublic::new(chain_id);

		// Creating the leaf
		let leaf = Leaf::create_leaf(&leaf_private, &leaf_public, &$params).unwrap();
		let nullifier_hash = Leaf::create_nullifier(&leaf_private, &$params).unwrap();
		(leaf_private, leaf_public, leaf, nullifier_hash, chain_id)
	}};
}

#[macro_export]
macro_rules! setup_tree {
	($test_field:ty, $leaf:expr, $params3:expr) => {{
		let rng = &mut test_rng();
		let leaves = vec![
			<$test_field>::rand(rng),
			<$test_field>::rand(rng),
			$leaf,
			<$test_field>::rand(rng),
		];
		let inner_params = Rc::new($params3.clone());
		let leaf_params = inner_params.clone();
		// Making the merkle tree
		let mt = MixerTree::new_sequential(inner_params, leaf_params, &leaves).unwrap();
		// Getting the proof path
		let path = mt.generate_membership_proof(2);
		let root = mt.root();
		(root, path)
	}};
}

#[macro_export]
macro_rules! setup_set {
	($test_field:ty, $root:expr) => {{
		let rng = &mut test_rng();
		let roots = vec![
			<$test_field>::rand(rng),
			<$test_field>::rand(rng),
			<$test_field>::rand(rng),
			$root,
		];
		let set_private_inputs = TestSetMembership::generate_secrets(&$root, &roots).unwrap();
		(set_private_inputs, roots)
	}};
}

#[macro_export]
macro_rules! setup_arbitrary_data {
	($test_field:ty) => {{
		let rng = &mut test_rng();
		let fee = <$test_field>::rand(rng);
		let recipient = <$test_field>::rand(rng);
		let relayer = <$test_field>::rand(rng);
		// Arbitrary data
		let arbitrary_input = MixerDataInput::new(recipient, relayer, fee);
		arbitrary_input
	}};
}

#[macro_export]
macro_rules! setup_circuit {
	($test_field:ty) => {{
		setup_types!($test_field);
		let params5 = setup_params_5!($test_field);
		let (leaf_private, leaf_public, leaf, nullifier_hash, chain_id) =
			setup_leaf!($test_field, params5);
		let arbitrary_input = setup_arbitrary_data!($test_field);
		let params3 = setup_params_3!($test_field);
		let (root, path) = setup_tree!($test_field, leaf, params3);
		let (set_private_inputs, roots) = setup_set!($test_field, root.clone().inner());

		let mc = Circuit::new(
			arbitrary_input.clone(),
			leaf_private,
			leaf_public,
			set_private_inputs,
			roots.clone(),
			params5,
			path,
			root.clone().inner(),
			nullifier_hash,
		);
		let mut public_inputs = Vec::new();
		public_inputs.push(chain_id);
		public_inputs.push(nullifier_hash);
		public_inputs.extend(roots);
		public_inputs.push(root.inner());
		public_inputs.push(arbitrary_input.recipient);
		public_inputs.push(arbitrary_input.relayer);
		public_inputs.push(arbitrary_input.fee);
		(public_inputs, mc)
	}};
}

#[macro_export]
macro_rules! verify_groth16 {
	($engine:ty, $circuit:expr, $public_inputs:expr) => {{
		let rng = &mut test_rng();
		let (pk, vk) = Groth16::<$engine>::circuit_specific_setup($circuit.clone(), rng).unwrap();
		let proof = Groth16::<$engine>::prove(&pk, $circuit, rng).unwrap();
		let res = Groth16::<$engine>::verify(&vk, &$public_inputs, &proof).unwrap();
		res
	}};
}
