(function (){
var cnv = document.getElementById("cnv_all");
var ctx = cnv.getContext("2d");

var rect_x = 10;
var rect_y = 10;
var speed_x = 2;
var speed_y = 2;

function blank(){
    ctx.fillStyle="black"
    ctx.fillRect(0,0,578,150);
}

function animate(){
    blank();
    ctx.fillStyle="Red"
    ctx.fillRect(rect_x,rect_y, 10,10);
    ctx.fill();
    rect_x += speed_x;
    rect_y += speed_y;
    if (rect_x >568 || rect_x <1){ speed_x *= -1;}
    if (rect_y >140 || rect_y <1){ speed_y *= -1;}
    requestAnimationFrame(animate);
}

requestAnimationFrame(animate);

}());

