wget -c https://github.com/nats-io/nats-server/releases/download/v2.8.4/nats-server-v2.8.4-linux-amd64.tar.gz -O - | tar -xz
sudo curl -L "https://github.com/docker/compose/releases/download/v2.10.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

cd wasmcloud && wget -c https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.50.2/x86_64-linux.tar.gz -O - | tar -xz
wget https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.52.0/x86_64-core-linux.tar.gz -O - | tar -xz

wget http://mirror.centos.org/centos/7/updates/x86_64/Packages/zlib-1.2.7-20.el7_9.x86_64.rpm
WASMCLOUD_HOST_SEED=SNAKDMBLB7TPIL4K3YXDGLUDXYFEEB2UAUXSAJYFBUKAWXBT6VPSTSE34Y WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000 RUST_LOG=debug ./bin/host_core console