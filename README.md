# osmosis-txfees-query-contract-example

Try it for yourself, if you want to test this contract out, simply run:

```sh
beaker task run deploy_and_query -- --deploy false
```

Since it has already deploy on testnet. This will run task in `tasks/deploy_and_query.rhai`, which will skip deployment and perform the query.

You can change the contract content and run:

```sh
beaker task run deploy_and_query -- --deploy true
```

If you want to playaround with it.
