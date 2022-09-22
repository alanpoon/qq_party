$('.chat-input input').keyup(function(e) {
	if ($(this).val() == '')
		$(this).removeAttr('good');
	else
		$(this).attr('good', '');
});

window.chat_append =function(user,user_id,msg,msg_id,msg_ago){
  var $item = $(
    "<article class=;msg-container msg-remote' id='msg-"+msg_id+"'>" + 
        "<div class='msg-box'>" +
          "<img class='user-img' id='user-"+user_id+"' src='//gravatar.com/avatar/00034587632094500000000000000000?d=retro' />"+
          "<div class='flr'>"+
            "<div class='messages'>"+
              "<p class='msg' id='msg-"+msg_id+"'>"+
              msg +
              "</p>"+
            "</div>"+
            "<span class='timestamp'><span class='username'>"+user+"</span>&bull;<span class='posttime'>"+msg_ago+"</span></span>"+
          "</div>"+
        "</div>"+
      "</article>");
  $("#chat_window").append($item)
  
}
var USER_KEY="tello"
// window.chat_send =function(e){
//   var dataObj = {
//     "Chat":{
      
//       "msg":e[0].value,
//       "user_key":USER_KEY
//     }
//   }
//   // window.web_bevy_events.push(dataObj)
//   // console.log("chat_send",e[0].value)
//   e[0].value=""
// }
// window.push_web_bevy_events_fn =function(msg,msg_ago,user){
//   window.chat_append(user,0,msg,0,msg_ago);
// }

