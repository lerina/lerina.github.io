<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
## Notes
### [Onja: Minimal Programming Methodology](./the_process/SIMPLE.html)
### [0x80 Process: A Personal Productivity system](./the_process/0x80_productivity_system.html)
### [Stacks and Heaps: The heart of 0x80 Process](./the_process/stack_heap.html)
### [We all suffer from acronym driven development](./dev_notes/sect.html) 
### [Problem-Solving Methodology](./dev_notes/D.M.Etter.html)

## Tips
### Vim
### Git
### Cinnamon

## The process

</main>
<footer>
  <a href="https://github.com/lerina" target="_blank" title="github">![github](https://lerina.github.io/img/github32px.png){.link .glow}
  </a>
</footer>

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
<li><a href="../../index.html">⇦ home</a></li>
<li><a href="../index.html">lerina</a></li>
<li><a href="./index.html">Dev notes</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>

