import { connect,ConnectionOptions,StringCodec } from "nats.ws";

export async function init_chat(){
    const conn =  await connect(
        {
        servers: ['ws://localhost:7083/'],
        },
    );
    const sc = StringCodec();
    const sub = conn.subscribe("chat");
    const sub_from_server = conn.subscribe("chat_from_server");
    $( "#chat-input" ).submit(function( event ) {
        const sc = StringCodec();
        console.log("text",$(this)[0][0].value);
        conn.publish("chat",sc.encode($(this)[0][0].value))
        conn.publish("chat_handler",sc.encode($(this)[0][0].value))
        //event.preventDefault();
        return false;
    });
    (async () => {
    for await (const m of sub) {
        console.log(`chat [${sub.getProcessed()}]: ${sc.decode(m.data)}`);
    }
    
    console.log("chat subscription closed");
    })();
    (async () => {
        for await (const m of sub_from_server) {
            console.log(`chat from server[${sub_from_server.getProcessed()}]: ${sc.decode(m.data)}`);
        }
    
    console.log("chat from server subscription closed");
    })();
}

// window.chat_send = function(e){
//     conn.publish("chat",sc(e[0].value))
//     e[0].value=""
// }