pub async fn subnet_raw_rand() -> Result<u64, String> {
    let management_canister = ic_cdk::export::Principal::management_canister();
    let rnd_buffer: (Vec<u8>,) = match ic_cdk::call(management_canister, "raw_rand", ()).await {
        Ok(result) => result,
        Err(err) => {
            return Err(err.1);
        }
    };

    Ok(rnd_buffer.0[0] as u64)
}