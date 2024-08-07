# zytron-miner


## docker

- modify `config.toml`
  - modify miner to your address
- `docker build -t pozk-miner .`
- `docker run -d -p 9098:9098 -v {your config.toml path, eg: /home/ubuntu/config-file}/data --name pozk-miner pozk-miner:latest /data/config.toml`
