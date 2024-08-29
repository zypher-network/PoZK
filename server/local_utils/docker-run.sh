docker run -d \
  --name miner-pozk2 \
  -p 9098:9098 \
  -v /home/cloud/tmp/pozk2:/home/ubuntu/pozk \
  -v /var/run/docker.sock:/var/run/docker.sock \
  miner-pozk:latest