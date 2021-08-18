use ink_lang as ink; 

#[ink::contract] 
mod MyContract {  


    #[ink(storage)] 
    pub struct MyContract { 
        my_account: AccountId, 
        my_balance: Balance, 
    } 
    
}
    