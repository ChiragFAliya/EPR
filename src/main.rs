use solana_sdk::pubkey::{Pubkey, self};

pub struct EggRegistry {
    pub providers: Vec<Pubkey>,
}
impl EggRegistry {
    pub fn new() -> Self {
        EggRegistry {providers: Vec::new() }
        }
       
    pub fn add_provider(&mut self, pubkey: Pubkey) {
        self.providers.push(pubkey);
     }
    
    pub fn fire_provider(&mut self, pubkey: Pubkey) {
        self.providers.retain(|&provider| provider != pubkey);
        println!("Provider with this {} has been fired !",pubkey);
    
    }
    
    pub fn is_provider(&mut self, pubkey: Pubkey) -> bool {
        self.providers.contains(&pubkey)

    }
    
    pub fn num_of_providers(&mut self) -> usize {
        self.providers.len()
    }
    
}


fn main(){

    let mut entry = EggRegistry::new();

    let provider_1 =Pubkey::new_unique();
    let provider_2 = Pubkey::new_unique();
    let provider_3 =Pubkey::new_unique();
    let provider_4 =Pubkey::new_unique();
    let provider_5 =Pubkey::new_unique();
    let provider_6 =Pubkey::new_unique();

    entry.add_provider(provider_1);
    entry.add_provider(provider_2);
    entry.add_provider(provider_3);
    entry.add_provider(provider_4);
    entry.add_provider(provider_5);
    entry.add_provider(provider_6);

    let check_provider = Pubkey::new_unique();
    println!("is {:?} a provider? { }",check_provider, entry.is_provider(check_provider));

    println!("check if {:?} is a provider? { } ", provider_1, entry.is_provider(provider_1));

    entry.fire_provider(provider_1);


    println!("check if {:?} is a provider? { } ", provider_1, entry.is_provider(provider_1));

    println!("numbers of providers: {}",entry.num_of_providers());

    println!("Remaining providers!");

    for provider in entry.providers {
        println!("{}", provider);
    }

}



