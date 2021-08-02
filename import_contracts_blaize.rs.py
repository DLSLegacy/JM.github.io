impl pallet_contracts::Trait for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveflip;
    type Currency = Balances;
    type Event = Event;
    type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminer<Runtime>;
    type TriedGenerator = pallet_contracts::TrieIdFromParentCounter<Runtime>;
    type RentPayment = ();
    type SignedClaimHandicap = pallet_contracts::DefaultSignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type StorageSizeOffset = pallet_contracts::DefaultStorageSizeOffset;
    type RentByteFee = RentByteFee;
    type RentDepositOffset = RentDepositOffset;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = pallet_contracts::DefualtMaxDepth;
    type MaxValueSize = pallet_contracts::DefaultMaxValueSize;
    type WeightPrice = pallet_transaction_payment::Module<Self>;
}



















