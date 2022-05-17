<div class="bg_rnd"></div>
<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>

<main>

> "You can retreat but never quit" ...  
> 
> External doubt is the essence of science,  
> internal doubt the nuisance of persistence.  
> Suvival then, like success,  
> is the fine art of progress
> thought obstacles and duress  
>                                                   _ rindra   

## [linkedin](https://ca.linkedin.com/in/rindra-razafy-b77509217)
## [github](https://github.com/razafy-rindra)

</main>


<script>
let anchor= document.createElement('a');
anchor.href="javascript:closeNav()"; //void(0)"; //anchor[0].onclick = closeNav();
anchor.className = "closebtn";  
anchor.innerHTML="&times;";
document.getElementById("TOC").prepend(anchor);

/* Set the width of the sidebar to 250px and the left margin of the page content to 250px */
function openNav() {
  document.getElementById("TOC").style.width = "60%";
  document.getElementsByTagName("MAIN").style.marginLeft = "250px";
}

/* Set the width of the sidebar to 0 and the left margin of the page content to 0 */
function closeNav() {
  document.getElementById("TOC").style.width = "0%";
  document.getElementsByTagName("MAIN").style.marginLeft = "0px";
}

let navCrumbs= document.createElement('div');
navCrumbs.className = "hover-nav";
navCrumbs.innerHTML = `
<div class="hover-nav">
<ul>
<li><a href="../../index.html">⇦ home</a></li>
<li><a href="./index.html">rindra</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
