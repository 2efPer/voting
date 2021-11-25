#### 1 set shell variable  
```shell
#> export ID=${YOUR_NEAR_TESTNET_ACCOUNT_NAME}
#> export CONTRACT_NAME=vote.lagosss.testnet #or change to you deployment address
```  
 Tx address:  https://explorer.testnet.near.org/transactions/AZFNeRQoFCMM9XhegbUydHAtNdECHStKYz6FKLVMTs4q
### 2 create voting pool  
In this case, We make create_pool method payable.You should remove `--amount 1` argument if it is not a payable method.  
```shell
#>near call $CONTRACT_NAME create_pool '{"question": "What is the best project in near platform?","voting_options":[{"option_id":"Vote-contract","option_desc":"some_url_addr_for_details"},{"option_id":"ref finance","option_desc":""}] }' --accountId $ID --amount 1

```    
 Tx address: https://explorer.testnet.near.org/transactions/ExMD3FiuDBcupMeDJuz8pRqsuvVZ16Mg1i7oQv6tW3dY

### 3 listing all voting pools  
```shell
#>near view $CONTRACT_NAME show_pools
``` 

### 4 query a voting pool's basic infomation   

```shell 
#>export POOL_ID_EXAMPLE=8iq4YSooiWAiUoKVUY8eHtHE7LzswzpoQ1wD11TBjLwh
#>near view vote.$ID  show_pool "{\"pool_id\":\"${POOL_ID_EXAMPLE}\"}"
```  

### 5 vote  
```shell
$>near call $CONTRACT_NAME vote "{\"pool_id\":\"$POOL_ID_EXAMPLE\",\"option_id\":\"Vote-contract\"}" --accountId $ID  
```  
 Tx address: https://explorer.testnet.near.org/transactions/GwwsgReKtUtZfF9k1eeNj16XYG12q9DyWFK3bQjLZeYw  


### 6 show the pool's result  
```shell
#> near view $CONTRACT_NAME show_results "{\"pool_id\":\"$POOL_ID_EXAMPLE\"}"
``` 

### 7 add candidate  
This is also a payable method.  
```shell
#> near call $CONTRACT_NAME add_option  "{\"pool_id\":\"$POOL_ID_EXAMPLE\",\"voting_options\":{\"option_id\":\"new added cadidate\",\"option_desc\":\"just a test\"}}" --accountId $ID --amount 0.5
```  
 Tx address: https://explorer.testnet.near.org/transactions/6CBqeLA9oRsgXYUDsiidYegddYoJpBKYaKBxBLM7FEDQ
