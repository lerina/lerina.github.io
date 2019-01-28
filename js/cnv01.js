(function (){
var cnv = document.getElementById("cnv");
  //cnv.style.width = window.innerWidth - (window.innerWidth *30/100) + 'px';
  //cnv.style.height = window.innerHeight  - (window.innerHeight *15/100) + 'px';
  OnResizeCalled();
  cnv.style.marginLeft = "auto";
  cnv.style.marginRight = "auto";
  cnv.style.display = "block";
    
  // cnv.style.display="none"; // it hides the entire element
  // visibility:hidden means that the contents of the element will be invisible, but the element stays in its original position and size.

var ctx = cnv.getContext("2d");


var rect_x = 10;
var rect_y = 10;
var speed_x = 2;
var speed_y = 2;



window.addEventListener("resize", OnResizeCalled, false);

function OnResizeCalled() {
    let newWidth  = window.innerWidth - (window.innerWidth *30/100);
    let newHeight = window.innerHeight - (window.innerHeight *85/100);
    cnv.style.width = newWidth + 'px';
    cnv.style.height = newHeight + 'px';
    
} 


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

