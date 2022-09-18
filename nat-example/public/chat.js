$('.chat-input input').keyup(function(e) {
	if ($(this).val() == '')
		$(this).removeAttr('good');
	else
		$(this).attr('good', '');
});

window.append_chat =function(user,msg){
  var $item = $(
    "<article class=;msg-container msg-remote' id='msg-"+msg.id+"'>" + 
        "<div class='msg-box'>" +
          "<img class='user-img' id='user-"+user.id+"' src='//gravatar.com/avatar/00034587632094500000000000000000?d=retro' />"+
          "<div class='flr'>"+
            "<div class='messages'>"+
              "<p class='msg' id='msg-"+msg.id+"'>"+
              msg.data +
              "</p>"+
            "</div>"+
            "<span class='timestamp'><span class='username'>"+user.name+"</span>&bull;<span class='posttime'>"+msg.ago+"</span></span>"+
          "</div>"+
        "</div>"+
      "</article>");
  $("#chat_window").append($item)
  
}