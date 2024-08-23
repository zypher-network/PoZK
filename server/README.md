# zytron-miner


## start pozk-miner docker

- modify `config.toml`
  - modify miner to your address
- `docker build -t pozk-miner .`
- `docker run -d -p 9098:9098 -v {your config.toml path, eg: /home/ubuntu/config-file}/data --name pozk-miner pozk-miner:latest /data/config.toml`


## start pozk-miner docker-compose

- copy `/server/cli/config.toml` to where you want
- Change the `miner` to the account address you want to connect to locally. If the front-end switches here, you need to change it and restart it.


```jsonc
ps. The path before the colon is the file path you defined, 
and the path after the colon is the base_path of config.toml. 
It is not recommended to modify it. 
At the same time, put config.toml in the path you defined.


      - /home/cloud/tmp/pozk:/home/ubuntu/pozk
```

