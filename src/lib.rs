use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::AccountId;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


type OptionId = String;
type PoolId = String;
type OptionDesc = String;

#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct VotingOption {
    option_id: OptionId,
    option_desc: OptionDesc,
}


#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct PoolInfo{
    creator: AccountId,
    pool_id: PoolId,
    question: String,
}


#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct Pool {
    info:PoolInfo,
    voting_options: Vec<VotingOption>,
}

#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct PoolResult {
    pool_id: PoolId,
    voting_counts: HashMap<OptionId, i32>,
    voted: HashMap<AccountId, OptionId>,
}

#[derive(Serialize, Deserialize)]
pub struct PoolStat {
    pool: PoolInfo,
    result: PoolResult,
}


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pools: HashMap<PoolId, Pool>,
    results: HashMap<PoolId, PoolResult>,
}



#[near_bindgen]
impl Contract {
    pub fn show_pools(&self) -> Vec<PoolInfo>{
        return  self.pools.clone().into_values().map(|pool_opt:Pool| pool_opt.info).collect();
    }

    pub fn show_pool(&self, pool_id: String) -> Option<Pool> {
        match self.pools.get(&pool_id) {
            Some(options) => Some(options.clone()),
            None => {
                env::log(format!("Unknown voting pool {}", pool_id).as_bytes());
                None
            }
        }
    }


    #[payable]
    pub fn create_pool(&mut self, question: String, voting_options: Option<Vec<VotingOption>>) -> PoolId {
        assert_eq!(env::attached_deposit(),10_u128.pow(24),"to create a pool must use at least 1 Near");
        env::log(
            format!(
                "create_pool for {} currently have {} pools",
                question,
                self.pools.len()
            )
            .as_bytes(),
        );
        let creator_account_id = env::signer_account_id();
        let pool_id = bs58::encode(env::sha256(&env::random_seed())).into_string();
        let ret = pool_id.clone();
        let mut voting_options_vec = <Vec<VotingOption>>::new();

        match voting_options{
            Some(voting_options_value) => {
                for option in voting_options_value.iter() {
                    voting_options_vec.push(VotingOption {
                        option_id: option.option_id.to_string(),
                        option_desc: option.option_desc.to_string(),
                    })
                }
            },
            None => ()
        }
        
        self.pools.insert(
            pool_id.clone(),
            Pool {
                info:PoolInfo{
                    creator: creator_account_id,
                    pool_id: pool_id.clone(),
                    question: question,
                },
                voting_options: voting_options_vec,
            },
        );
        self.results.insert(
            pool_id.clone(),
            PoolResult {
                pool_id: pool_id,
                voting_counts: HashMap::new(),
                voted: HashMap::new(),
            },
        );
        return ret;
    }

    #[payable]
    pub fn add_option(&mut self,pool_id:PoolId, voting_options: VotingOption) -> bool{
        assert_eq!(env::attached_deposit(),10_u128.pow(24)/2,"to add a candidate must use at least 0.5 Near");
        env::log(
            format!(
                "add_candidate for pool {} currently have {} cadidates",
                pool_id,
                self.pools.get(&pool_id).unwrap().voting_options.len()
            )
            .as_bytes(),
        );
        match self.pools.get_mut(&pool_id){
            Some(pool_option) => {
                pool_option.voting_options.insert(0,voting_options);
                return true;
            }
            None => ()
        };
        return false
    }



    
    pub fn vote(&mut self, pool_id: PoolId, option_id:OptionId) -> bool {
        let voter_contract = env::signer_account_id();
        let owner_contract = env::current_account_id();
        env::log(
            format!(
                "{} is voting on pool {}, owner is {}",
                voter_contract, pool_id, owner_contract
            )
            .as_bytes(),
        );
        match self.results.get_mut(&pool_id) {
            Some(result) => {
                match result.voted.get(&voter_contract) {
                    Some(_) => {
                        env::log(
                            format!("{} already voted in pool-{}", voter_contract, pool_id).as_bytes(),
                        );
                        return false;
                    }
                    None => {
                            result.voted.insert(voter_contract.clone(), option_id.to_string());
                        
                    }
                }

                match result.voting_counts.get_mut(&option_id) {
                    Some(count) => {
                        *count = *count + 1;
                    }
                    None => {
                        result.voting_counts.insert(option_id.to_string(), 1);
                    }
                }          
                return true;
            }
            None => {
                env::log(format!("no pool known for {}", pool_id).as_bytes());
                return false;
            }
        };
    }

 

    pub fn show_results(&self, pool_id: PoolId) -> Option<PoolStat> {
        match self.pools.get(&pool_id) {
            Some(pool) => match self.results.get(&pool_id) {
                Some(result) => Some(PoolStat {
                    result: result.clone(),
                    pool: pool.info.clone(),
                }),
                None => None,
            },
            None => None,
        }
    }

    pub fn i_love_the_monkey_head(&self) -> String {
        "PONG".to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool,payment:u128) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            epoch_height: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: payment,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn nonexisting_pool() {
        let context = get_context(vec![], false,0);
        testing_env!(context);
        let contract = Contract::default();
        let options = contract.show_pool("default".to_string());
        assert_eq!(true, options.is_none());
    }

    #[test]
    fn create_pool() {
        let context = get_context(vec![], false,10_u128.pow(24));
        testing_env!(context);
        let mut contract = Contract::default();
        let pool_id = contract.create_pool(
            "Are You awesome?".to_string(),
            Some([
                VotingOption{option_id:"Yes".to_string(), option_desc:"https://pros.docs".to_string()},
                VotingOption{option_id:"No".to_string(), option_desc:"https://cons.docs".to_string()},
            ]
            .iter()
            .cloned()
            .collect()),
        );
        let options:Option<Pool> = contract.show_pool(pool_id);
        assert_eq!(false, options.is_none());
        assert_eq!("Are You awesome?".to_string(), options.unwrap().info.question);
    }
}