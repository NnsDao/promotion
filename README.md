# promotion

Welcome to your new promotion project and to the internet computer development community. By default, creating a new project adds this README and some template files to your project directory. You can edit these template files to customize your project and to include your own code to speed up the development cycle.

To get started, you might want to explore the project directory structure and the default configuration file. Working with this project in your development environment will not affect any production deployment or identity tokens.

To learn more before you start working with promotion, see the following documentation available online:

- [Quick Start](https://smartcontracts.org/docs/quickstart/quickstart-intro.html)
- [SDK Developer Tools](https://smartcontracts.org/docs/developers-guide/sdk-guide.html)
- [Rust Canister Devlopment Guide](https://smartcontracts.org/docs/rust-guide/rust-intro.html)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://smartcontracts.org/docs/candid-guide/candid-intro.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.app)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd promotion/
dfx help
dfx config --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

## doc
```bash
cargo doc --open 
```

## promotion commands
```
// add promotion 
dfx canister --network ic call promotion add_promotion '(record {nft=vec {record {token="w7o4l-hakor-uwiaa-aaaaa-cqac6-aaqca-aaath-a"; is_saled=false}; record {token="mjkop-xikor-uwiaa-aaaaa-cqac6-aaqca-aaalm-q"; is_saled=false}}; canister_id="ah2fs-fqaaa-aaaak-aalya-cai"; end_time=1750772326000000000; start_time=1650772326000000000; conditions=opt record {canister_id="vgqnj-miaaa-aaaal-qaapa-cai"; limit=100000000000; canister_type=1}; price=100000000})'

// update promotion 
dfx canister --network ic call promotion add_promotion '(1:nat32, record {nft=vec {record {token="w7o4l-hakor-uwiaa-aaaaa-cqac6-aaqca-aaath-a"; is_saled=false}; record {token="mjkop-xikor-uwiaa-aaaaa-cqac6-aaqca-aaalm-q"; is_saled=false}}; canister_id="ah2fs-fqaaa-aaaak-aalya-cai"; end_time=1750772326000000000; start_time=1650772326000000000; conditions=opt record {canister_id="vgqnj-miaaa-aaaal-qaapa-cai"; limit=100000000000; canister_type=1}; price=100000000})'

```

## debug mod code
~~~
// #[update]
// #[candid::candid_method(update)]
// async fn transfer_ndp() -> Result<u128, String> {
//     let user = "d3bd873c05cd9b131b4607e05f27382cfe1a327eda974ffdbcaf6d19d2aec23c".to_owned();
//     let from = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT);
//     // Temporarily set 200
//     let ndp_amount = 100000000 * 200;
//     let ndp_arg = ext::TransferRequest{
//         from: ext::User::address(from.to_string()),
//         to: ext::User::address(user.clone()),
//         token: get_canister_id(CanisterEnmu::Ndp),
//         notify: false,
//         memo: Vec::new(),
//         subaccount: None,
//         amount:ndp_amount as u128,
//     };
//     match CanisterClient::new(get_canister_id(CanisterEnmu::Ndp)).transfer(ndp_arg).await.unwrap().0 {
//         ext::TransferResponse::ok(height) => {
//             let record = ExchangeRecord{
//                 user: user.clone(),
//                 status: true,
//                 amount: ndp_amount,
//                 time: time(),
//             };
//             PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_exchange_record(record));
//             return Ok(height);
//         }
//         ext::TransferResponse::err(_) => {
//             let record = ExchangeRecord{
//                 user: user.clone(),
//                 status: false,
//                 amount: ndp_amount,
//                 time: time(),
//             };
//             PROMOTION_STATE.with(|promotion_service| promotion_service.borrow_mut().set_exchange_record(record));
//             return Err("Purchase failed".to_owned());
//         }
//     }
// }


// #[update]
// #[candid::candid_method(update)]
// async fn transfer() -> ext::TransferResponse {
//     let addr = AccountIdentifier::new(&ic_cdk::api::id(), &ic_ledger_types::DEFAULT_SUBACCOUNT);
//     let arg = ext::TransferRequest{
//         to: ext::User::address("d3bd873c05cd9b131b4607e05f27382cfe1a327eda974ffdbcaf6d19d2aec23c".to_owned()),
//         token: "w7o4l-hakor-uwiaa-aaaaa-cqac6-aaqca-aaath-a".to_owned(),
//         notify: false,
//         from: ext::User::address(addr.to_string()),
//         memo: Vec::new(),
//         subaccount: None,
//         amount: 1,
//     };
//     CanisterClient::new(get_canister_id(CanisterEnmu::Pod)).transfer(arg).await.unwrap()
//     .0
// }

// #[query]
// #[candid::candid_method(query)]
// fn get_time() -> u64 {
//     time()
// }

// #[update]
// #[candid::candid_method]
// async fn get_ndp() -> ext::BalanceResponse {
//     let arg = ext::BalanceRequest {
//         token: "vgqnj-miaaa-aaaal-qaapa-cai".to_owned(),
//         user: ext::User::address(
//             "d3bd873c05cd9b131b4607e05f27382cfe1a327eda974ffdbcaf6d19d2aec23c".to_owned(),
//         ),
//     };
//     let call_result = CanisterClient::new(get_canister_id(CanisterEnmu::Ndp))
//         .balance(arg)
//         .await;
//     call_result.unwrap().0
// }

~~~
