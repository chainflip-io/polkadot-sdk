// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use crate as xcmp_queue;
use core::marker::PhantomData;
use cumulus_pallet_parachain_system::AnyRelayNumber;
use cumulus_primitives_core::{IsSystem, ParaId};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Everything, Nothing, OriginTrait},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use xcm::prelude::*;
use xcm_builder::{CurrencyAdapter, FixedWeightBounds, IsConcrete, NativeAsset, ParentIsPreset};
use xcm_executor::traits::ConvertOrigin;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ParachainSystem: cumulus_pallet_parachain_system::{
			Pallet, Call, Config<T>, Storage, Inherent, Event<T>, ValidateUnsigned,
		},
		XcmpQueue: xcmp_queue::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

type AccountId = u64;

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Test>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 5;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

impl cumulus_pallet_parachain_system::Config for Test {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = ();
	type OutboundXcmpMessageSource = XcmpQueue;
	// Ignore all DMP messages:
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<(), sp_core::ConstU8<0>>;
	type ReservedDmpWeight = ();
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ();
	type CheckAssociatedRelayNumber = AnyRelayNumber;
	type ConsensusHook = cumulus_pallet_parachain_system::consensus_hook::ExpectParentIncluded;
}

parameter_types! {
	pub const RelayChain: MultiLocation = MultiLocation::parent();
	pub UniversalLocation: InteriorMultiLocation = X1(Parachain(1u32));
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000, 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<RelayChain>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

pub type LocationToAccountId = (ParentIsPreset<AccountId>,);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = ();
	type IsReserve = NativeAsset;
	type IsTeleporter = NativeAsset;
	type UniversalLocation = UniversalLocation;
	type Barrier = ();
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = ();
	type ResponseHandler = ();
	type AssetTrap = ();
	type AssetClaims = ();
	type SubscriptionService = ();
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
}

pub type XcmRouter = (
	// XCMP to communicate with the sibling chains.
	XcmpQueue,
);

pub struct SystemParachainAsSuperuser<RuntimeOrigin>(PhantomData<RuntimeOrigin>);
impl<RuntimeOrigin: OriginTrait> ConvertOrigin<RuntimeOrigin>
	for SystemParachainAsSuperuser<RuntimeOrigin>
{
	fn convert_origin(
		origin: impl Into<MultiLocation>,
		kind: OriginKind,
	) -> Result<RuntimeOrigin, MultiLocation> {
		let origin = origin.into();
		if kind == OriginKind::Superuser &&
			matches!(
				origin,
				MultiLocation {
					parents: 1,
					interior: X1(Parachain(id)),
				} if ParaId::from(id).is_system(),
			) {
			Ok(RuntimeOrigin::root())
		} else {
			Err(origin)
		}
	}
}

parameter_types! {
	pub storage EnqueuedMessages: Vec<(ParaId, Vec<u8>)> = Default::default();
}

use frame_support::{traits::Footprint, BoundedSlice};

pub struct EnqueueToLocalStorage<T>(PhantomData<T>);

pub fn enqueued_messages(origin: ParaId) -> Vec<Vec<u8>> {
	EnqueuedMessages::get()
		.into_iter()
		.filter(|(o, _)| *o == origin)
		.map(|(_, m)| m)
		.collect()
}

impl<T: OnQueueChanged<ParaId>> EnqueueMessage<ParaId> for EnqueueToLocalStorage<T> {
	type MaxMessageLen = sp_core::ConstU32<65_536>;

	fn enqueue_message(message: BoundedSlice<u8, Self::MaxMessageLen>, origin: ParaId) {
		let mut msgs = EnqueuedMessages::get();
		msgs.push((origin, message.to_vec()));
		EnqueuedMessages::set(&msgs);
		T::on_queue_changed(origin, Self::footprint(origin));
	}

	fn enqueue_messages<'a>(
		iter: impl Iterator<Item = BoundedSlice<'a, u8, Self::MaxMessageLen>>,
		origin: ParaId,
	) {
		let mut msgs = EnqueuedMessages::get();
		for message in iter {
			msgs.push((origin, message.to_vec()));
		}
		EnqueuedMessages::set(&msgs);
		T::on_queue_changed(origin, Self::footprint(origin));
	}

	fn sweep_queue(origin: ParaId) {
		let mut msgs = EnqueuedMessages::get();
		msgs.retain(|(o, _)| o != &origin);
		EnqueuedMessages::set(&msgs);
		T::on_queue_changed(origin, Self::footprint(origin));
	}

	fn footprint(origin: ParaId) -> Footprint {
		let msgs = EnqueuedMessages::get();
		let mut footprint = Footprint::default();
		for (o, m) in msgs {
			if o == origin {
				footprint.count += 1;
				footprint.size += m.len() as u64;
			}
		}
		footprint.pages = footprint.size as u32 / 16; // Number does not matter
		footprint
	}
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ChannelInfo = MockedChannelInfo;
	type VersionWrapper = ();
	type XcmpQueue = EnqueueToLocalStorage<Pallet<Test>>;
	type MaxInboundSuspended = sp_core::ConstU32<1_000>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = SystemParachainAsSuperuser<RuntimeOrigin>;
	type WeightInfo = ();
	type PriceForSiblingDelivery = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

/// A para that we have a HRMP channel with.
pub const MAGIC_PARA_ID: u32 = 7777;

pub struct MockedChannelInfo;
impl GetChannelInfo for MockedChannelInfo {
	fn get_channel_status(id: ParaId) -> ChannelStatus {
		if id == MAGIC_PARA_ID.into() {
			return ChannelStatus::Ready(128, usize::MAX)
		}

		ParachainSystem::get_channel_status(id)
	}

	fn get_channel_max(id: ParaId) -> Option<usize> {
		if id == MAGIC_PARA_ID.into() {
			return Some(128)
		}

		ParachainSystem::get_channel_max(id)
	}
}

pub(crate) fn mk_page() -> Vec<u8> {
	let mut page = Vec::<u8>::new();

	for i in 0..100 {
		page.extend(match i % 5 {
			0 => v2_xcm().encode(),
			1 => v3_xcm().encode(),
			// We cannot push an undecodable XCM here since it would break the decode stream.
			// This is expected and the whole reason to introduce `MaybeDoubleEncodedVersionedXcm`
			// instead.
			2 => MaybeDoubleEncodedVersionedXcm(v2_xcm()).encode(),
			3 => MaybeDoubleEncodedVersionedXcm(v3_xcm()).encode(),
			4 => MaybeDoubleEncodedVersionedXcm(undecodable_xcm()).encode(),
			_ => unreachable!(),
		});
	}

	page
}

pub(crate) fn v2_xcm() -> VersionedXcm<()> {
	let instr = xcm::v2::Instruction::<()>::ClearOrigin;
	VersionedXcm::V2(xcm::v2::Xcm::<()>(vec![instr; 3]))
}

pub(crate) fn v3_xcm() -> VersionedXcm<()> {
	let instr = xcm::v3::Instruction::<()>::Trap(1);
	VersionedXcm::V3(xcm::v3::Xcm::<()>(vec![instr; 3]))
}

/// A too deeply nested XCM.
pub(crate) fn undecodable_xcm() -> VersionedXcm<()> {
	let mut xcm = Xcm(vec![ClearOrigin]);
	for _ in 0..MAX_XCM_DECODE_DEPTH {
		xcm = Xcm(vec![SetAppendix(xcm)]);
	}
	let xcm = VersionedXcm::V3(xcm);
	assert!(validate_xcm_nesting(&xcm).is_err());
	xcm
}
