use crate::{
	core::trusted_call::TrustedCall,
	primitives::{
		address::Address32,
		identity::{Identity, ValidationData},
		network::Web3Network,
		types::KeyPair,
	},
	utils::{enclave::mrenclave_to_bs58, hex::ToHexPrefixed, identity::ValidationDataBuilder},
	Creek, WorkerPublicApis, WorkerTxApi,
};
use sp_core::{sr25519, Pair};

use super::service::DiRequest;

impl WorkerTxApi for Creek {
	fn link_identity(&self) {
		let alice = sr25519::Pair::from_string("//Alice", None).unwrap();
		let alice_identity_a = Address32::from(alice.public());
		let alice_identity = Identity::Substrate(alice_identity_a);

		// let address1 =
		// Address32::from_hex("0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"
		// ).unwrap(); let identity1 = Identity::Substrate(address1);

		let bob = sr25519::Pair::from_string("//Bob", None).unwrap();
		let bob_identity = Address32::from(bob.public());
		let bob_identity = Identity::Substrate(bob_identity);
		let networks = vec![Web3Network::Litentry];

		let shard = self.author_get_shard().unwrap();
		let mrenclave = self.state_get_mrenclave().unwrap();
		let sidechain_nonce = self
			.author_get_next_nonce(
				mrenclave_to_bs58(&shard.to_fixed_bytes()),
				alice_identity_a.to_hex(),
			)
			.unwrap();

		let vdata = ValidationData::build_vdata_substrate(
			&alice,
			&alice_identity,
			&bob_identity,
			sidechain_nonce,
		)
		.unwrap();

		let call = TrustedCall::link_identity(
			alice_identity.clone(),
			alice_identity,
			bob_identity,
			vdata,
			networks,
			None,
			Default::default(),
		);
		let call_signed =
			call.sign(&KeyPair::Sr25519(Box::new(alice)), sidechain_nonce, &mrenclave, &shard);
		let operation_call = call_signed.into_trusted_operation(true);

		let tee_shielding_key = self.author_get_shielding_key().unwrap();
		let jsonresp = self.client().di_request(shard, tee_shielding_key, &operation_call).unwrap();
		println!("[LINK IDENTITY]: {:?}", jsonresp);
	}
}
