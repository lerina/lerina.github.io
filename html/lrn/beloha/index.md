<div class="bg_blh"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>


</main>
<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://razafy.com/img/github32px.png){.link .glow}
  </a>
</footer>

<script src="https://razafy.com/js/toc.js"></script>
<script>
let anchor= document.createElement('a');
anchor.href="javascript:closeNav()"; //void(0)"; //anchor[0].onclick = closeNav();
anchor.className = "closebtn";  
anchor.innerHTML="&times;";
document.getElementById("TOC").prepend(anchor);

let navCrumbs= document.createElement('div');
navCrumbs.className = "hover-nav";
navCrumbs.innerHTML = `
<div class="hover-nav">
<ul>
<li><a href="../../../index.html">⇦ home</a></li>
<li><a href="../index.html">lerina</a></li>
<li><a href="./index.html">beloha: a webnovel</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>