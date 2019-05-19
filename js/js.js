(function (){
let banner = document.getElementById("banner");
  banner.style.marginLeft = "auto";
  banner.style.marginRight = "auto";
  banner.style.display = "block";
  OnResizeCalled();
  
window.addEventListener("resize", OnResizeCalled, false);

function OnResizeCalled() {
    let newWidth  = window.innerWidth - (window.innerWidth *30/100);
    let newHeight = window.innerHeight - (window.innerHeight *85/100);
    banner.style.width = newWidth + 'px';
    banner.style.height = newHeight + 'px';
    
} 

}());

