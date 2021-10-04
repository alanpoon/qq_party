import React from 'react';
import { Grid } from '@material-ui/core';
import { connect,StringCodec } from "./../nats";

import Peer from 'peerjs';
const sc = StringCodec();
const myPeer = new Peer(undefined, {
  host: '/',
  port: '9000'
})
const myVideo = document.createElement('video')
myVideo.muted = true
const peers = {}
function onBeforeUnload(e) {
  if (thereAreUnsavedChanges()) {
      e.preventDefault();
      e.returnValue = '';
      return;
  }

  delete e['returnValue'];
}

window.addEventListener('beforeunload', onBeforeUnload);

function addVideoStream(video, stream) {
  const videoGrid = document.getElementById('video-grid')
  video.srcObject = stream
  video.addEventListener('loadedmetadata', () => {
    video.play()
  })
  videoGrid.append(video)
}
function connectToNewUser(userId, stream) {
  const call = myPeer.call(userId, stream)
  const video = document.createElement('video')
  call.on('stream', userVideoStream => {
    addVideoStream(video, userVideoStream)
  })
  call.on('close', () => {
    video.remove()
  })
  peers[userId] = call
}


export default class AutomationTab extends React.Component {
  state = {
  }
  async run() {
    // create a connection
    
    const nc = await connect({ servers: "localhost:9222"
    , user: "client",
    pass: "client", 
    });
    const sc = StringCodec();
    const sub = nc.subscribe("hello");
    var ROOM_ID = "";
    (async () => {
      for await (const m of sub) {
        console.log("m.data",m.data)
        ROOM_ID = sc.decode(m.data)
        console.log(`something new[${sub.getProcessed()}]: ${sc.decode(m.data)}`);
      }
      console.log("subscription closed");
    })();
    
    
    nc.publish("hello", sc.encode("world"));
    nc.publish("hello", sc.encode("world"));
    //nc.publish("zello", sc.encode("world"));
    const urlParams = new URLSearchParams(window.location.search);
    if (urlParams.get("host")=="true"){
      var data = {"room_number_s":"23","description":"hello"}
      nc.publish("ws_gateway.room_mgr2.host_create_room",sc.encode(JSON.stringify(data)))
      myPeer.on('open', id => {
        //socket.emit('join-room', ROOM_ID, id)
        console.log("someone joined",id)
        
      })
      const subz = nc.subscribe("public.connect_to_host.23");
      var ROOM_ID = "";
      (async () => {
        for await (const m of subz) {
          var user_data = JSON.parse(sc.decode(m.data))
          console.log("public.connect_to_host.23",user_data)
          this.join_room("23",user_data)
        }
        console.log("subscription closed");
      })();
    }else{
      myPeer.on('open', id => {
        //socket.emit('join-room', ROOM_ID, id)
        var data = {"user_id_s":Math.random().toString(),"peer_id":id}
        nc.publish("public.connect_to_host.23",sc.encode(JSON.stringify(data)))
      })
      
    }
    
 
    navigator.mediaDevices.getUserMedia({
      video: true,
      audio: true
    }).then(stream => {
      addVideoStream(myVideo, stream)
      myPeer.on('call', call => {
        call.answer(stream)
    
        const video = document.createElement('video')
        call.on('stream', userVideoStream => {
          addVideoStream(video, userVideoStream)
        })
      })
      const sub_user_connected = nc.subscribe("public.user-connected.23");
      (async () => {
        for await (const m of sub_user_connected) {
          var peerid = sc.decode(m.data)
          console.log("sub_user_connected",peerid)
          connectToNewUser(peerid,stream)
        }
      })();
    })
  }
  
  componentDidMount(){
    this.run()
  }
  async join_room(room_number,user_data){
    const nc = await connect({ servers: "localhost:9222"
    , user: "client",
    pass: "client", 
    });
    var data ={
      "room_number_s":room_number,
      "user":user_data
    }
    nc.publish("ws_gateway.room_mgr2.join_room", sc.encode(JSON.stringify(data)));
    
    nc.publish("public.user-connected.23",sc.encode(user_data["peer_id"]))
  }
  
  render(){
    var __this = this
    return (
      <div style={{width:"100%"}}>
        Mock commands:
        <Grid container spacing={1}>
          <Grid item xs={6}>
            <div>Call_rpc</div>
            <div id="video-grid"></div>
            
          </Grid>
          <Grid item xs={6}>
            <div>Set_Mock</div>
          </Grid>
        </Grid>
      </div>
    )
  }
}