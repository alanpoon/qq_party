start server
cd runner & make

send nats command
cd runner & cargo run --bin msg

https://docs.nats.io/nats-tools/nsc/basics

bash ops/compile.sh ws_gateway,ws_server,wasm_user


nats-account-server -c ops/nas_not.conf
docker run -d -p 5000:5000 --name registry registry:2.7
nats-server -c ops/websocket_lite.conf -js -V -D

WASMCLOUD_HOST_SEED=SNAKDMBLB7TPIL4K3YXDGLUDXYFEEB2UAUXSAJYFBUKAWXBT6VPSTSE34Y WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000 WASMCLOUD_CLUSTER_SEED=SCALV3N2M5JRUOCSLTRTAIQYJSBKNZ3XMGME2XI3L3OSR2RRSYUTMHAN7E wasmcloud_host start
sudo systemctl start docker

docker run -d -p 5000:5000 --restart always --name registry registry

docker run -d -p 6379:6379 --restart always --name redis redis:6.2

https://opengameart.org/content/wooden-stick

https://lashkar.in/how-to-make-an-egg-shape

storm https://raw.githubusercontent.com/tiandaye/phaser_projects/master/example-games/legendofwolf/assets/sprites/storm.png

thunder JonathanShaw https://opengameart.org/content/thunder-very-close-rain-01

BG TeFox opengameart
Cracking sound Hansjorg Malthaner

https://v6p9d9t4.ssl.hwcdn.net/html/6421126/index.html