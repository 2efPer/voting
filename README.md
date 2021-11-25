
# Zero bullshit smart contract for beginners: Build a Vote System and get paid for it!   


This is a baby-sistLet's get things done!  

## 1. Prerequisites
 1.1 [Installing the `Rust` toolchain](https://www.rust-lang.org/tools/install)  
  1.2 [Creating a `NEAR` account](https://docs.near.org/docs/develop/contracts/rust/intro#creating-a-near-account)  
  1.3 [Installing the `near-cli`](https://docs.near.org/docs/develop/contracts/rust/intro#installing-the-near-cli)
   

   
## 2. Creating your own repository  

2.1 use [this template](https://github.com/near-examples/rust-template) to create a Rust repository.  

2.2 Edit `Cargo.toml` as it described, set your `${PROJECT_NAME}` and the crate we use in this project is called [`near-sdk`](https://docs.rs/near-sdk/3.1.0/near_sdk/) , additionally we should change the near-sdk version from `4.0.0-pre.4` to `3.1.0` in this project.  




## 3. Logic of this smart contract  
   Our voting system should meets the following needs of users:  
### 3.1 creating a voting pool with some voting options.  
```rust
fn create_pool(question: String, voting_options: Option<Vec<VotingOption>>) -> PoolId
``` 
the function should receive the question name for voting, and add some voting options optionally,so we wrap it by `Option`. And the function should return a PoolId which the user created.
### 3.2 we should also can add a voting option after the creation of voting pool.  
```rust
fn add_option(pool_id:PoolId, voting_options: VotingOption) -> bool
```
### 3.3 list all existing pools ,or query a pool by its `poolId`
```rust 
fn show_pools() -> Vec<PoolInfo>;  
fn show_pool(pool_id: String) -> Option<Pool>;
```
the different between `PoolInfo` and `Pool` is `PoolInfo` shouldn't  include the voting status. 
### 3.4 VOTE! 
```rust 
fn vote(pool_id: PoolId, option_id:OptionId) -> bool 
```
### 3.5 check the pool results  
```rust
fn show_results(pool_id: PoolId) -> Option<PoolStat>
```

## 4. figuring out our data structure
### 4.1. core datastruce  
Everything we need is either the information about the `Pool` or a statistical result of the `Pool` which we just call `PoolResult`.Then we get our core design of the smart contract.

   ```rust
    use near_sdk::AccountId;
    type PoolId = String;
    type OptionId = String;
    pub struct Contract{
        pools: HashMap<PoolId, Pool>,
        results: HashMap<PoolId, PoolResult>,
    }


    pub struct Pool {
        info:PoolInfo,
        voting_options: Vec<VotingOption>,
    }
    
    pub struct PoolResult {
        pool_id: PoolId,
        voting_counts: HashMap<OptionId, i32>,
        voted: HashMap<AccountId, OptionId>,
    }

    pub struct PoolInfo{
        creator: AccountId,
        pool_id: PoolId,
        question: String,
    }

    pub struct VotingOption {
        option_id: OptionId,
        option_desc: OptionDesc,
    } 
   ``` 
Since this is a Zero-Bullshit tutorial , you may find out [the reason why we use HashMap](https://www.near-sdk.io/contract-structure/collections#in-memory-hashmap-vs-persistent-unorderedmap) or [Vec\<VotingOption>](https://www.near-sdk.io/contract-structure/collections).Or [not](https://discord.gg/teknCYc3m3)?  

You also need to wrap the `Contract` struct in `#[near_bindgen]`  ,it will generates a smart contract compatible with the NEAR blockchain. 

And many structures also need to implement some `Serialize` and `Deserialize` traits to store or transfer data.: 
    
```rust
#[derive(Serialize, Deserialize, Clone, BorshDeserialize, BorshSerialize)]
pub struct PoolResult {
    pool_id: PoolId,
    voting_counts: HashMap<OptionId, i32>,
    voted: HashMap<AccountId, OptionId>,
}
```
### You can read more about [Serialization Protocols](https://www.near-sdk.io/contract-interface/serialization-interface).


## 5. "Add some details"  
### 5.1 Implement those functions which are in part#3, write your code in `lib.rs` of your project repository. [This Repository](https://github.com/2efPer/voting) maybe helpful.
   ```rust
    impl Contract{
        //implement the functions in part#3 
        //or check the following repository: 

        // https://github.com/2efPer/voting
    }
   ```
### 5.2 make it payable!
```rust
    #[payable]
    pub fn my_payable_method(&mut self) {
        //...
    }
```
### 5.3 may be add some unit tests and simulation tests ?    

just check [This Repository](https://github.com/2efPer/voting/blob/master/src/lib.rs) for example .Or read [this document](https://www.near-sdk.io/testing/unit-tests) for more details.

## 6. compile and [deploy to testnet/mainnet](https://docs.near.org/docs/develop/contracts/rust/intro#deploying-the-contract)  

### 6.1 Compile 
```shell
#> cd /path/to/your/project/root/dir
#> mkdir res
#> set -e
#> RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
#> cargo build --target wasm32-unknown-unknown --release
#> cp target/wasm32-unknown-unknown/release/${PROJECT_NAME}.wasm ./res/
```  
Or You can just use the `build.sh` file.   

### 6.2 deployment(Testnet,dev mode)  
```shell
#> cd /path/to/your/project/root/dir
#> near dev-deploy --wasmFile res/${PROJECT_NAME}.wasm  
#> source neardev/dev-account.env 
```  
Then you will get a enviorment variable ${CONTRACT_NAME} .Read more [details](https://docs.near.org/docs/tools/near-cli#near-dev-deploy) about dev-deploy.
### 6.3 deployment(Testnet,use subaccount)  
```shell
#> cd /path/to/your/project/root/dir
#> naer login
#> export ID=${YOUR_NEAR_TESTNET_ACCOUNT_NAME}
#> near create-account vote.$ID --masterAccount $ID --initialBalance 5  
#> near deploy --wasmFile res/${PROJECT_NAME}.wasm --accountId vote.$ID
```

### 7. Usage  
#### 7.1 set shell variable  
```shell
export ID=${YOUR_NEAR_TESTNET_ACCOUNT_NAME};
export CONTRACT_NAME=vote.lagosss.testnet #or change to you deployment address;
```  

### 7.2 create voting pool  
In this case, We make create_pool method payable.You should remove `--amount 1` argument if it is not a payable method.  
```shell
near call $CONTRACT_NAME create_pool '{"question": "What is the best project in near platform?","voting_options":[{"option_id":"Vote-contract","option_desc":"some_url_addr_for_details"},{"option_id":"ref finance","option_desc":""}] }' --accountId $ID --amount 1

```  

### 7.3 listing all voting pools  
```shell
near view $CONTRACT_NAME show_pools
``` 

### 7.4 query a voting pool's basic infomation   

```shell 
export POOL_ID_EXAMPLE=8iq4YSooiWAiUoKVUY8eHtHE7LzswzpoQ1wD11TBjLwh;
near view vote.$ID  show_pool "{\"pool_id\":\"${POOL_ID_EXAMPLE}\"}"
```  

### 7.5 vote  
```shell
near call $CONTRACT_NAME vote "{\"pool_id\":\"$POOL_ID_EXAMPLE\",\"option_id\":\"Vote-contract\"}" --accountId $ID  
```  

### 7.6 show the pool's result  
```shell
 near view $CONTRACT_NAME show_results "{\"pool_id\":\"$POOL_ID_EXAMPLE\"}"
``` 

### 7.7 add candidate  
This is also a payable method.  
```shell
 near call $CONTRACT_NAME add_option  "{\"pool_id\":\"$POOL_ID_EXAMPLE\",\"voting_options\":{\"option_id\":\"new added cadidate\",\"option_desc\":\"just a test\"}}" --accountId $ID --amount 0.5
```
