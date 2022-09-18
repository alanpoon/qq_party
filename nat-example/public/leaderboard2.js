// var xhReq = new XMLHttpRequest();
// xhReq.open("GET", "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd", false);
// xhReq.send(null);
//var data = JSON.parse(xhReq.responseText); 
window.leaderboard = [{"name":"hello","score":12414,"flag":"cn"},{"name":"tello","score":22,"flag":"cn"}]
// initialization
var cryptocurrencies;
var timerId;
var updateInterval = 30000;


function ascending(a, b) { return a.score > b.score ? 1 : -1; }
function descending(a, b) { return a.score < b.score ? 1 : -1; }
function reposition() {
    var height = $("#cryptocurrencies .cryptocurrency").height();
    var y = height;
    for(var i = 0; i < cryptocurrencies.length; i++) {
        cryptocurrencies[i].$item.css("top", y + "px");
        y += height;			
    }
}
function updateRanks(cryptocurrencies) {
    for(var i = 0; i < cryptocurrencies.length; i++) {
        cryptocurrencies[i].$item.find(".rank").text(i + 1);	
    }
}

function fetchNewData(data,attributeName,name){
    for(var x in data){
        if((data[x].name == name) == true) {
            return data[x][attributeName];
        }
    }
    return null;
}        
function updateBoard() {
    var cryptocurrency = getRandomCoin();	
    cryptocurrency.percent_change_24h += getRandomScoreIncrease();
    cryptocurrency.$item.find(".percent_change_24h").text(cryptocurrency.percent_change_24h);
    cryptocurrencies.sort(descending);
    updateRanks(cryptocurrencies);
    reposition();
}

function getNewData(){
    // get the new data for each coin and change to their new values
    // var newReq = new XMLHttpRequest();
    // newReq.open("GET", "https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd", false);
    // newReq.send(null);
    // var newData = JSON.parse(newReq.responseText); 
    var newData = window.leaderboard;
    for(var i = 0; i < cryptocurrencies.length; i++) {
        var cryptocurrency = cryptocurrencies[i];
        cryptocurrency.score = fetchNewData(newData,'score',cryptocurrency.name);
        cryptocurrency.$item.find(".score").text(cryptocurrency.score);
   
    }
    cryptocurrencies.sort(descending);
    updateRanks(cryptocurrencies);
    reposition();
    console.log('Finished retrieving new data');
    
}
function getRandomScoreIncrease() {
    return getRandomBetween(50, 150);
}
function getRandomBetween(minimum, maximum) {
        return Math.floor(Math.random() * maximum) + minimum;
}
function resetBoard() {
    window.leaderboard = [{"name":"hello","score":12414,"flag":"cn"},{"name":"tello","score":22,"flag":"cn"}]
    var $list = $("#cryptocurrencies");
    $list.find(".cryptocurrency").remove();
    if(timerId !== undefined) {
        clearInterval(timerId);
    }
    cryptocurrencies = [];
    for(let i = 0;i < window.leaderboard.length;i++){
        cryptocurrencies.push(
            {
                name : window.leaderboard[i].name,
                score: window.leaderboard[i].score,
                flag: window.leaderboard[i].flag
            }
        )
    }
    
    for(var i = 0; i < cryptocurrencies.length; i++) {
        var $item = $(
            "<tr class='cryptocurrency'>" + 
                "<th class='rank'>"  + (i + 1) + "</th>" + 
                //"<td class='name f32'>" + '<span class="f32"><img class="flag '+cryptocurrencies[i].flag+'"/></span>'  +cryptocurrencies[i].name + "</td>" + 
                "<td class='name f32'>" + '<img class="flag '+cryptocurrencies[i].flag+'"/>'  +cryptocurrencies[i].name + "</td>" + 
                "<td class='score'>" + cryptocurrencies[i].score + "</td>" + 
            "</tr>"
        );
          console.log("$item",$item,(2 + 1))
        cryptocurrencies[i].$item = $item;
        $list.append($item);
    }
    cryptocurrencies.sort(descending);
    updateRanks(cryptocurrencies);
    reposition();
    
    // fetch new data for the updateInterval
    timerId = setInterval("getNewData();", updateInterval);

}	
resetBoard();