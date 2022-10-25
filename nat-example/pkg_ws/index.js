import { connect,ConnectionOptions } from "nats.ws";
import { unpack, pack } from 'msgpackr';
//var rand = require('rand');
import rand from 'rand'
import {init_chat} from './chat'
export function init_pkg_ws(){
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
    var name = document.getElementById("name").value
    $('input').blur(function(event) {
       console.log(event.target.checkValidity());
    }).bind('invalid', function(event) {
        setTimeout(function() { $(event.target).focus();}, 50);
    });
    // var form = document.querySelector('form')
    // console.log(" form.reportValidity()", form.reportValidity())
    if (name.length <5){
    //  alert("Enter name than 5 characters")
      return
    }
    window.user = name;
    var user_type = $("input:radio[name ='user']:checked").val();
    var ball_id_sprite_enum = 0
    if (user_type=="github"){
      $.get("https://api.github.com/repos/alanpoon/qq_party/stargazers",function(data){
       for(var i=0;i<=data.length;i++){
        if (data[i].login==name){
          ball_id_sprite_enum =1
          break;
        }
       }
       var d_l = ClientMessageWelcome(ball_id_sprite_enum,name)
        window.web_bevy_events.push(d_l)
        var modal = document.getElementById("myModal");
        modal.style.display = "none";
      })
    }else{
      var d_l = ClientMessageWelcome(ball_id_sprite_enum,name)
      window.web_bevy_events.push(d_l)
      var modal = document.getElementById("myModal");
      modal.style.display = "none";
    }
  }
  init_chat()
  window.push_web_bevy_events_fn2 =function(msg){
    var event = JSON.parse(msg);
    if (typeof event["Scores"]!="undefined"){
      var unsorted = [];
      if (typeof event["Scores"]["scoreboard"]!="undefined"){
        
        var scores = event["Scores"]["scoreboard"];
        for (var p=0; p< scores.length;p++){
          var ball_id = scores[p][0];
          var score = scores[p][1];
          var label = scores[p][2];
          var name = label[0];
          var flag = label[1].replace(".","");
          unsorted.push({"name":name,"score":score,"flag":flag});
        }
      }
      window.leaderboard_new_data(unsorted)
    }else if (typeof event["StormRings"]!="undefined"){
      console.log("nn",event["StormRings"]);
      window.storm_rings = event["StormRings"]["storm_rings"];
    }else if (typeof event["Ball"]!="undefined"){
      window.local_ball = event["Ball"];
    }else if (typeof event["StateChange"]!="undefined"){
      console.log("StateChange",event["StateChange"]["state"]);
      if ( typeof event["StateChange"]["state"] !="undefined"){
        switch (event["StateChange"]["state"]){
          case "Running":
            break
          case "Stop":
            var x = document.getElementById("myWinners");
            x.style.display = "block";
            break
          default:
            break
        }
       
      }
      var scores = event["StateChange"]["scoreboard"];
      for (var p=0; p< scores.length;p++){
        var ball_id = scores[p][0];
        var score = scores[p][1];
        var label = scores[p][2];
        var name = label[0];
        $("#winner_"+(p+1).toString()).text(name);
        var flag = label[1].replace(".","");
        $("#winner_"+(p+1).toString()+"_flag").removeClass();
        $("#winner_"+(p+1).toString()+"_flag").addClass("flag");
        $("#winner_"+(p+1).toString()+"_flag").addClass(flag);
      }
    }else if (typeof event["StateNotification"]!="undefined"){
      if ( typeof event["StateNotification"]["countdown"] !="undefined"){
        console.log("StateNotification",event["StateNotification"]);
        $("#alert_placeholder").after(
          '<div class="alert alert-success alert-dismissable">'+
              '<button type="button" class="close" ' + 
                      'data-dismiss="alert" aria-hidden="true">' + 
                  '&times;' + 
              '</button>' + 
              event["StateNotification"]["text"] + (Math.floor(event["StateNotification"]["countdown"]/1000)).toString() +" secs"+
           '</div>');
      }
    }
  }
}
function randomIntFromInterval(min, max) { // min and max included 
  return Math.floor(Math.random() * (max - min + 1) + min)
}
function ClientMessageWelcome(ball_id_sprite_enum,label_0){
  console.log("ball_id_sprite_enum",ball_id_sprite_enum);
  var e = document.getElementById("country")
  var value = "."+e.value.toLowerCase();
  var label_1 = value

  let ball_id = randomIntFromInterval(10000,99999);
  var dataObj = {
      "Welcome":{
        "game_id":"hello",
        "ball_id":[ball_id,ball_id_sprite_enum],
        "ball_label":[label_0,label_1]
      }
  }
  return dataObj
}
