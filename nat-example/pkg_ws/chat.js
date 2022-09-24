import { connect } from "nats.ws";
import { unpack, pack } from 'msgpackr';
import moment from 'moment';
export async function init_chat(){
    const conn =  await connect(
        {
        servers: ['ws://localhost:7083/'],
        },
    );
    const sub_from_server = conn.subscribe("chat_from_server");
    $( "#chat-input" ).submit(function( event ) {
        var c = {
            user:window.user,
            data:$(this)[0][0].value
        }
        var d = pack(c)
        conn.publish("chat_handler.hello",d)
        $(this)[0][0].value=""
        //event.preventDefault();
        return false;
    });
    (async () => {
        for await (const m of sub_from_server) {
            var d = unpack(m.data)
            var user = d[0];
            var msg = d[1];
           
            $("#chat_text_area").append(user+": "+msg+"&#10;")
            var msg_id = "0"
            var msg_ago = "now"
            var user_id ="0"
            var location="remote"
            if (user==window.user){
              location = "self"
            }
            var current = moment().unix();
            var user_avatar = user.replace("_","+")
            var $item = $(
                "<article class='msg-container msg-"+location+"' id='msg-"+msg_id+"'>" + 
                    "<div class='msg-box'>" +
                      "<img class='user-img' id='user-"+user_id+"' src='//ui-avatars.com/api/?name="+user_avatar+"&background=random' />"+
                      "<div class='flr'>"+
                        "<div class='messages'>"+
                          "<p class='msg' id='msg-"+msg_id+"'>"+
                          msg +
                          "</p>"+
                        "</div>"+
                        "<span class='timestamp'><span class='username'>"+user+"</span>&bull;<span class='posttime' data-time='"+current+"'>"+msg_ago+"</span></span>"+
                      "</div>"+
                    "</div>"+
                  "</article>");
              $("#chat-window").append($item);
              console.log("chat height ",$(".chatbox").height())

              $("#chat-window").animate({ scrollTop:  $("#chat-window").prop("scrollHeight") }, 1000);
        }
    
    console.log("chat from server subscription closed");
    })();
    setInterval(function(){
      // update chat ago
      $(".posttime").each(function(){
        var posttime = $(this).data("time");
        $(this).text(moment.unix(posttime).fromNow());
      })
     
    }, 10000);
    
}