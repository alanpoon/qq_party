import { connect,ConnectionOptions } from "nats.ws";
import { unpack, pack } from 'msgpackr';
//var rand = require('rand');
import rand from 'rand'
import {init_chat} from './chat'
export function init_pkg_ws(){
  // document.getElementById("buttonz").onclick(async function(e){
    
  //   await ClientMessageWelcome("l","k")
  // })
  window.web_bevy_events=[]
  window.web_bevy_events_fn = function(){
    var copy = []
    for (var i=0;i<window.web_bevy_events.length;i++){
      copy.push(window.web_bevy_events[i])
    }
    window.web_bevy_events =[]
    return copy;
  }
  var hello_btn = document.getElementById("hello_button");
  hello_btn.onclick = function(event){
    var modal = document.getElementById("myModal");
    modal.style.display = "none";
    var name = document.getElementById("name").value
    window.user = name;
    var e = document.getElementById("country")
    var value = "."+e.value.toLowerCase();
    var d_l = ClientMessageWelcome(name,value)
    window.web_bevy_events.push(d_l)
    var modal = document.getElementById("myModal");
    modal.style.display = "none";
  }
  init_chat()
  window.push_web_bevy_events_fn2 =function(msg){
    var event = JSON.parse(msg);
    if (typeof event["Scores"]!="undefined"){
      var unsorted = [];
      if (typeof event["Scores"]["scoreboard"]!="undefined"){
        
        var scores = event["Scores"]["scoreboard"];
        for (var p=0; p< scores.length;p++){
          var score = scores[p][0];
          var label = scores[p][1];
          var name = label[0]
          var flag = label[1].replace(".","");
          unsorted.push({"name":name,"score":score,"flag":flag});
        }
      }
  
      window.leaderboard_new_data(unsorted)
    }
  }
}
function randomIntFromInterval(min, max) { // min and max included 
  return Math.floor(Math.random() * (max - min + 1) + min)
}
function ClientMessageWelcome(label_0,label_1){
  console.log("ClientMessageWelcome",label_0,label_1)
  // const conn =  await connect(
  //   {
  //     servers: ['ws://localhost:7083/'],
  //   },
  // );
  let ball_id = randomIntFromInterval(10000,99999);
  var dataObj = {
      "Welcome":{
        "game_id":"hello",
        "ball_id":[ball_id,0],
        "ball_label":[label_0,label_1]
      }
  }
  return dataObj
}
