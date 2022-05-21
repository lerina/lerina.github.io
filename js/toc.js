/* Set the width of the sidebar to 250px and the left margin of the page content to 250px */
function openNav() {
  document.getElementById("TOC").style.width = "60%";
  //document.getElementsByTagName('main')[0].style.marginLeft = "65%";

  //document.getElementsByTagName('main')[0].addEventListener("click", closeNav());
  document.getElementsByTagName('main')[0].onclick = function() {closeNav()};     
}

/* Set the width of the sidebar to 0 and the left margin of the page content to 0 */
function closeNav() {
  document.getElementById("TOC").style.width = "0%";
  //document.getElementsByTagName('main')[0].style.margin = "0 auto";

}


