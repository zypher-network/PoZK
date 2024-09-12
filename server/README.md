# zytron-miner

## start pozk-miner docker-compose

- copy `/deploy/config.toml` to where you want
- Change the `miner` to the account address you want to connect to locally. If the front-end switches here, you need to change it and restart it.

## Dev in local
- in cli, `cargo run -- --miner 0x000_mineraccount_000 --endpoint https://linea-testnet-zytron.zypher.game --network testnet --host-base-path ./ --docker-base-path ./`


```jsonc
ps. The path before the colon is the file path you defined,
and the path after the colon is the base_path of config.toml.
It is not recommended to modify it.
At the same time, put config.toml in the path you defined.


      - /home/cloud/tmp/pozk:/home/ubuntu/pozk
```

