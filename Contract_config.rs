pub trait Config: { 
    type Time: Time; 
    type Randomness: Randomness<Self::Hash, Self::BlockNumber>; 
    type Currency: Currency<Self::AccountID>;  
    type Event: From<Event<Self>> + IsType<Self::Event>; 
    type RentPayment: OnUnbalanced<<Self::Currency as Currency<Self::AccountId>>::NegativeImbalance>; 
    type WeightPrice: Convert<Weight, <Self::Currency as Currency<Self::AccountID>>::Balance>; 
    type WeightInfo: WeightInfo; 
    type ChainExtension: ChainExtension<Self>; 
    type Schedule: Get<Schedule<Self>>: 
    type SignedClaimHandicap: Get<Self::BlockNumber>; 
    type TombstoneDeposit: Get<<Self::Currency as Currency<Self::AccountId>>::Balance>; 
    type DepositPerContract: Get<<Self::Currency as Currency<<Self::AccountId>>::Balance>; 
    type DepositPerStorageByte: Get<<Self::Currency as Currency<<Self::AccountId>>::Balance>; 
    type DespoitPerStorageItem: Get<<Self::Currency as Currency<<Self::AccountId>>::Balance>; 
    type RentFraction: Get<Perbill>; 
    type SurchargeReward: Get<<Self::Currency as Currency<Self::AccountId>>::Balance>; 
    type CallStack: Array<Item = Frame<Self>>; 
    type DeletionQueueDepth: Get<u32>; 
    type DeletionWeightLimit: Get<Weight>; 
}  