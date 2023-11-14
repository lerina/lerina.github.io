<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

[`<--` Importing non-browser JS ](./003_importing_non-browser_JS.html)

## Working with the char type



## What's next?



Next example: [Importing non-browser JS `-->`](./003_importing_non-browser_JS.html)

  
</main>

<script src="https://lerina.github.io/js/toc.js"></script>
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
<li><a href="../../../../index.html">⇦ home</a></li>
<li><a href="../index.html">hello_world</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
