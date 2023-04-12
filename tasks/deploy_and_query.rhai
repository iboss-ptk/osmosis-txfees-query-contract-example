let cli_args = match_args(["deploy"]);

let default_args = #{
    signer_account: "test1",
    contract_name: "txfees-query",
    network: "testnet"
};

if cli_args.deploy == "true" {
    let contract = wasm::deploy(merge( default_args, #{  no_wasm_opt: true, msg: #{} }));
}

wasm::query(merge( default_args, #{  msg: #{ query_fee_tokens: #{} } }));
wasm::query(merge( default_args, #{  msg: #{ query_denom_pool_id: #{ denom: "uion" } } }));

