#!/bin/bash
#sudo apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin
wget -c https://github.com/nats-io/nats-server/releases/download/v2.8.4/nats-server-v2.8.4-linux-amd64.tar.gz -O - | tar -xz
sudo cp nats-server-v2.8.4-linux-amd64/nats-server /usr/local/bin
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo curl -L "https://github.com/docker/compose/releases/download/v2.10.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
mkdir ops && cd ops && wget https://raw.githubusercontent.com/alanpoon/qq_party/release/ops/websocket2.conf && wget https://raw.githubusercontent.com/alanpoon/qq_party/release/ops/docker-compose.yml
sudo scp -r /Users/alan.poon/srv/ ubuntu@ec2-13-212-53-5.ap-southeast-1.compute.amazonaws.com:~
mkdir game_client
sudo scp -r nat-example/public ubuntu@ec2-13-212-53-5.ap-southeast-1.compute.amazonaws.com:~/game_client
sed -i 's/Users/home/g' ops/websocket2.conf
sed -i 's/alan.poon/ubuntu/g' ops/websocket2.conf
nats-server -c ops/websocket2.conf -js
nohup nats-server -c ops/websocket2.conf -V -D -js >> nohup_nats.out 2>&1 &
sudo apt install nginx
docker compose -f ops/docker-compose.yml up -d
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install cmake
cargo install basic-http-server
nohup basic-http-server game_client -a 127.0.0.1:4001 >> nohup_web_server.out 2>&1 &
nohup python3 -m http.server --directory game_client/public 4001 >> nohup_web_server.out 2>&1 &
WASMCLOUD_CTL_HOST=13.212.53.51 REMOTE=13.212.53.51 WASMCLOUD_RPC_HOST=13.212.53.51 make build_aws
# sudo systemctl restart docker
#docker exec -it 3e99f752bc0f bash
nohup python3 -m http.server --directory game_client 4001 >> nohup_web_server.out 2>&1 &
echo "" > $(docker inspect --format='{{.LogPath}}' c78165653107)