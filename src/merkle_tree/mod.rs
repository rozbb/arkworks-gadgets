use ark_ff::{to_bytes, ToBytes};
use ark_std::{
	collections::{BTreeMap, BTreeSet},
	vec::Vec,
};
use webb_crypto_primitives::{Error, FixedLengthCRH};

/// configuration of a Merkle tree
pub trait Config {
	/// Tree height
	const HEIGHT: u8;
	/// The CRH
	type H: FixedLengthCRH;
}

pub struct Path<P: Config> {
	pub(crate) path: Vec<(
		<P::H as FixedLengthCRH>::Output,
		<P::H as FixedLengthCRH>::Output,
	)>,
}

impl<P: Config> Path<P> {
	/// verify the lookup proof, just checking the membership
	pub fn verify<L: ToBytes>(
		&self,
		parameters: &<P::H as FixedLengthCRH>::Parameters,
		root_hash: &<P::H as FixedLengthCRH>::Output,
		leaf: &L,
	) -> Result<bool, Error> {
		if self.path.len() != P::HEIGHT as usize {
			return Ok(false);
		}
		// Check that the given leaf matches the leaf in the membership proof.
		if self.path.is_empty() {
			return Ok(false);
		}

		let claimed_leaf_hash = hash_leaf::<P::H, L>(parameters, leaf)?;

		// Check if claimed leaf hash is the same as one of
		// the provided hashes on level 0
		if claimed_leaf_hash != self.path[0].0 && claimed_leaf_hash != self.path[0].1 {
			return Ok(false);
		}

		let mut prev = claimed_leaf_hash;
		// Check levels between leaf level and root.
		for &(ref left_hash, ref right_hash) in &self.path {
			// Check if the previous hash matches the correct current hash.
			if &prev != left_hash && &prev != right_hash {
				return Ok(false);
			}
			prev = hash_inner_node::<P::H>(parameters, left_hash, right_hash)?;
		}

		Ok(root_hash == &prev)
	}
}

type HOutput<P> = <<P as Config>::H as FixedLengthCRH>::Output;
type HParameters<P> = <<P as Config>::H as FixedLengthCRH>::Parameters;

/// Merkle sparse tree
pub struct SparseMerkleTree<P: Config> {
	/// data of the tree
	pub tree: BTreeMap<u64, HOutput<P>>,
	empty_hashes: Vec<HOutput<P>>,
}

impl<P: Config> SparseMerkleTree<P> {
	/// obtain an empty tree
	pub fn blank(parameters: &HParameters<P>) -> Self {
		let empty_hashes = gen_empty_hashes::<P>(parameters).unwrap();

		SparseMerkleTree {
			tree: BTreeMap::new(),
			empty_hashes,
		}
	}

	pub fn insert_batch<L: Default + ToBytes>(
		&mut self,
		parameters: &HParameters<P>,
		leaves: &BTreeMap<u32, L>,
	) -> Result<(), Error> {
		let last_level_index: u64 = (1u64 << P::HEIGHT) - 1;

		let mut level_idxs: BTreeSet<u64> = BTreeSet::new();
		for (i, leaf) in leaves {
			let true_index = last_level_index + (*i as u64);
			let leaf_hash = hash_leaf::<P::H, _>(parameters, leaf)?;
			self.tree.insert(true_index, leaf_hash);
			level_idxs.insert(parent(true_index).unwrap());
		}

		for level in 0..P::HEIGHT {
			let mut new_idxs: BTreeSet<u64> = BTreeSet::new();
			for i in level_idxs {
				let left_index = left_child(i);
				let right_index = right_child(i);

				let empty_hash = self.empty_hashes[level as usize].clone();
				let left = self.tree.get(&left_index).unwrap_or(&empty_hash);
				let right = self.tree.get(&right_index).unwrap_or(&empty_hash);

				self.tree
					.insert(i, hash_inner_node::<P::H>(&parameters, &left, &right)?);

				let parent = match parent(i) {
					Some(i) => i,
					None => break,
				};
				new_idxs.insert(parent);
			}
			level_idxs = new_idxs;
		}

		Ok(())
	}

	/// initialize a tree (with optional data)
	pub fn new<L: Default + ToBytes>(
		parameters: &HParameters<P>,
		leaves: &BTreeMap<u32, L>,
	) -> Result<Self, Error> {
		let last_level_size = leaves.len().next_power_of_two();
		let tree_size = 2 * last_level_size - 1;
		let tree_height = tree_height(tree_size as u64);
		assert!(tree_height <= P::HEIGHT as u32);

		// Initialize the merkle tree.
		let tree: BTreeMap<u64, HOutput<P>> = BTreeMap::new();
		let empty_hashes = gen_empty_hashes::<P>(parameters)?;

		let mut smt = SparseMerkleTree { tree, empty_hashes };
		smt.insert_batch(parameters, leaves)?;

		Ok(smt)
	}

	#[inline]
	/// obtain the root hash
	pub fn root(&self) -> HOutput<P> {
		self.tree.get(&0).cloned().unwrap()
	}

	/// generate a membership proof (does not check the data point)
	pub fn generate_membership_proof(&self, index: u64) -> Path<P> {
		let mut path = Vec::new();

		let tree_index = convert_index_to_last_level::<P>(index);

		// Iterate from the leaf up to the root, storing all intermediate hash values.
		let mut current_node = tree_index;
		let mut level = 0;
		while !is_root(current_node) {
			let sibling_node = sibling(current_node).unwrap();

			let empty_hash = &self.empty_hashes[level];

			let current = self
				.tree
				.get(&current_node)
				.cloned()
				.unwrap_or(empty_hash.clone());
			let sibling = self
				.tree
				.get(&sibling_node)
				.cloned()
				.unwrap_or(empty_hash.clone());

			if is_left_child(current_node) {
				path.push((current, sibling));
			} else {
				path.push((sibling, current));
			}
			current_node = parent(current_node).unwrap();
			level += 1;
		}

		Path { path }
	}
}

/// Returns the log2 value of the given number.
#[inline]
fn log2(number: u64) -> u32 {
	ark_std::log2(number as usize)
}

/// Returns the height of the tree, given the size of the tree.
#[inline]
fn tree_height(tree_size: u64) -> u32 {
	log2(tree_size)
}

/// Returns true iff the index represents the root.
#[inline]
fn is_root(index: u64) -> bool {
	index == 0
}

/// Returns the index of the left child, given an index.
#[inline]
fn left_child(index: u64) -> u64 {
	2 * index + 1
}

/// Returns the index of the right child, given an index.
#[inline]
fn right_child(index: u64) -> u64 {
	2 * index + 2
}

/// Returns the index of the sibling, given an index.
#[inline]
fn sibling(index: u64) -> Option<u64> {
	if index == 0 {
		None
	} else if is_left_child(index) {
		Some(index + 1)
	} else {
		Some(index - 1)
	}
}

/// Returns true iff the given index represents a left child.
#[inline]
fn is_left_child(index: u64) -> bool {
	index % 2 == 1
}

/// Returns the index of the parent, given an index.
#[inline]
fn parent(index: u64) -> Option<u64> {
	if index > 0 {
		Some((index - 1) >> 1)
	} else {
		None
	}
}

#[inline]
fn convert_index_to_last_level<P: Config>(index: u64) -> u64 {
	index + (1u64 << P::HEIGHT) - 1
}

/// Returns the output hash, given a left and right hash value.
pub(crate) fn hash_inner_node<H: FixedLengthCRH>(
	parameters: &H::Parameters,
	left: &H::Output,
	right: &H::Output,
) -> Result<H::Output, Error> {
	let bytes = to_bytes![left, right]?;
	H::evaluate(parameters, &bytes)
}

/// Returns the hash of a leaf.
fn hash_leaf<H: FixedLengthCRH, L: ToBytes>(
	parameters: &H::Parameters,
	leaf: &L,
) -> Result<H::Output, Error> {
	H::evaluate(parameters, &to_bytes![leaf]?)
}

fn hash_empty<H: FixedLengthCRH>(parameters: &H::Parameters) -> Result<H::Output, Error> {
	H::evaluate(parameters, &vec![0u8; H::INPUT_SIZE_BITS / 8])
}

fn gen_empty_hashes<P: Config>(parameters: &HParameters<P>) -> Result<Vec<HOutput<P>>, Error> {
	let mut empty_hashes = Vec::with_capacity(P::HEIGHT as usize);

	let mut empty_hash = hash_empty::<P::H>(&parameters)?;
	empty_hashes.push(empty_hash.clone());

	for _ in 1..P::HEIGHT {
		empty_hash = hash_inner_node::<P::H>(&parameters, &empty_hash, &empty_hash)?;
		empty_hashes.push(empty_hash.clone());
	}

	Ok(empty_hashes)
}

#[cfg(test)]
mod test {
	use super::{gen_empty_hashes, hash_inner_node, hash_leaf, Config, SparseMerkleTree};
	use crate::test_data::{get_mds_3, get_rounds_3};
	use ark_ed_on_bn254::Fq;
	use ark_ff::{ToBytes, UniformRand};
	use ark_std::{collections::BTreeMap, test_rng};
	use webb_crypto_primitives::crh::{
		poseidon::{sbox::PoseidonSbox, PoseidonParameters, Rounds, CRH as PoseidonCRH},
		FixedLengthCRH,
	};

	#[derive(Default, Clone)]
	struct PoseidonRounds3;

	impl Rounds for PoseidonRounds3 {
		const FULL_ROUNDS: usize = 8;
		const PARTIAL_ROUNDS: usize = 57;
		const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
		const WIDTH: usize = 3;
	}

	type SMTCRH = PoseidonCRH<Fq, PoseidonRounds3>;

	struct SMTConfig;
	impl Config for SMTConfig {
		type H = SMTCRH;

		const HEIGHT: u8 = 3;
	}

	type SMT = SparseMerkleTree<SMTConfig>;

	fn create_merkle_tree<L: Default + ToBytes + Copy>(
		parameters: &<SMTCRH as FixedLengthCRH>::Parameters,
		leaves: &[L],
	) -> SMT {
		let pairs: BTreeMap<u32, L> = leaves
			.iter()
			.enumerate()
			.map(|(i, l)| (i as u32, *l))
			.collect();
		let smt = SMT::new(parameters, &pairs).unwrap();

		smt
	}

	#[test]
	fn should_create_tree() {
		let rng = &mut test_rng();
		let rounds3 = get_rounds_3::<Fq>();
		let mds3 = get_mds_3::<Fq>();
		let params3 = PoseidonParameters::<Fq>::new(rounds3, mds3);

		let leaves = vec![Fq::rand(rng), Fq::rand(rng), Fq::rand(rng)];
		let smt = create_merkle_tree(&params3, &leaves);

		let root = smt.root();

		let empty_hashes = gen_empty_hashes::<SMTConfig>(&params3).unwrap();
		let hash1 = hash_leaf::<SMTCRH, _>(&params3, &leaves[0]).unwrap();
		let hash2 = hash_leaf::<SMTCRH, _>(&params3, &leaves[1]).unwrap();
		let hash3 = hash_leaf::<SMTCRH, _>(&params3, &leaves[2]).unwrap();

		let hash12 = hash_inner_node::<SMTCRH>(&params3, &hash1, &hash2).unwrap();
		let hash34 = hash_inner_node::<SMTCRH>(&params3, &hash3, &empty_hashes[0]).unwrap();
		let hash1234 = hash_inner_node::<SMTCRH>(&params3, &hash12, &hash34).unwrap();
		let calc_root = hash_inner_node::<SMTCRH>(&params3, &hash1234, &empty_hashes[2]).unwrap();

		assert_eq!(root, calc_root);
	}

	#[test]
	fn should_generate_and_validate_proof() {
		let rng = &mut test_rng();
		let rounds3 = get_rounds_3::<Fq>();
		let mds3 = get_mds_3::<Fq>();
		let params3 = PoseidonParameters::<Fq>::new(rounds3, mds3);

		let leaves = vec![Fq::rand(rng), Fq::rand(rng), Fq::rand(rng)];
		let smt = create_merkle_tree(&params3, &leaves);

		let proof = smt.generate_membership_proof(0);

		let res = proof.verify(&params3, &smt.root(), &leaves[0]).unwrap();
		assert!(res);
	}
}
