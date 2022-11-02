import { connect,ConnectionOptions } from "nats.ws";
import { unpack, pack } from 'msgpackr';
//var rand = require('rand');
import rand from 'rand'
import {init_chat} from './chat'
function state_is_not_running(){
  $(".alert_placeholder").html(
    '<div class="alert alert-success alert-dismissable">'+
        'Please wait, game has not started.' +      
     '</div>');
     var timeinterval;
     timeinterval = setInterval(() => {
    
        $(".alert_placeholder").empty();
        clearInterval(timeinterval);
      
    },2000);
}
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
          ball_id_sprite_enum =1;
          $("#fire_img").attr('src','assets/2d/shadow/stick.png');
          $("#fire_img").css("transform","scale(1.5)");
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
    var docElm = document.documentElement;
    if (docElm.requestFullscreen) {
      docElm.requestFullscreen();
    } else if (docElm.msRequestFullscreen) {
      docElm.msRequestFullscreen();
    } else if (docElm.mozRequestFullScreen) {
      docElm.mozRequestFullScreen();
    } else if (docElm.webkitRequestFullScreen) {
      docElm.webkitRequestFullScreen();
    }
  }
  init_chat()
  var timeinterval;
  var countdown;
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
      window.storm_rings = event["StormRings"]["storm_rings"];
    }else if (typeof event["Ball"]!="undefined"){
      window.local_ball = event["Ball"];
    }else if (typeof event["StateChange"]!="undefined"){
      if ( typeof event["StateChange"]["state"] !="undefined"){
        switch (event["StateChange"]["state"]){
          case "Running":
            var x = document.getElementById("myWinners");
            x.style.display = "none";
            window.qq_state = "Running";
            break
          case "Stop":
            var x = document.getElementById("myWinners");
            //commented out
            //x.style.display = "block";
            
            window.qq_state = "Stop";
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
        $("#winner_"+(p+1).toString()+"_score").text("score: "+score.toString());
        var flag = label[1].replace(".","");
        $("#winner_"+(p+1).toString()+"_flag").removeClass();
        $("#winner_"+(p+1).toString()+"_flag").addClass("flag");
        $("#winner_"+(p+1).toString()+"_flag").addClass(flag);
      }
    }else if (typeof event["StateNotification"]!="undefined"){
      if ( typeof event["StateNotification"]["countdown"] !="undefined"){
        $(".alert_placeholder").html(
          '<div class="alert alert-success alert-dismissable">'+
              // '<button type="button" class="close" ' + 
              //         'data-dismiss="alert" aria-hidden="true">' + 
              //     '&times;' + 
              // '</button>' + 
              event["StateNotification"]["text"] + '<span class="notification_counter">'+(Math.floor(event["StateNotification"]["countdown"]/1000)).toString() +'</span>' +" secs"+
           '</div>');
           countdown = Math.floor(event["StateNotification"]["countdown"]/1000)
           timeinterval = setInterval(() => {
            $(".notification_counter").text(countdown.toString());
            countdown-=1
            if (countdown < 0) {
              $(".alert_placeholder").empty();
              clearInterval(timeinterval);
            }
          },1000);
      }
    }else if (typeof event["Welcome"]!="undefined"){
      //only if ball_id is same as local_user_info
      if ( typeof event["Welcome"]["qq_state"]!="undefined"){
        var x = document.getElementById("myWinners");
        //commented out
        //x.style.display = "block";
        state_is_not_running();
      }
    }else if (typeof event["DisplayUI"]!="undefined"){
      var to_show = event["DisplayUI"];
      var elem;
      if (to_show=="fire") {
        elem = "amdfc-simple-button-3"
      }else if (to_show=="dash"){
        elem = "amdfc-simple-button-1"
      }
      var x = document.getElementById(elem);
      x.style.display = "inline-block"
      //x.style['pointer-events'] = "auto"
    }else if (typeof event["HideUI"]!="undefined"){
      var to_show = event["HideUI"];
      var elem;
      if (to_show=="fire") {
        elem = "amdfc-simple-button-3"
      }else if (to_show=="dash"){
        elem = "amdfc-simple-button-1"
      }
      var x = document.getElementById(elem);
      //x.style['pointer-events'] = "none"
      x.style.display = "none"
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
