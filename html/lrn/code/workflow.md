
<div class="bg_lerina"></div><div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>
## My Workflow

There is a fake dichotomy about Top-down vs Bottom-up approach 
In reality most practices integrate elements of both.
We alternate between "do the right job" awarness and "do the job right" eagerness.
This is Reflective equilibrium applied to software development.

As professionals, design decisions flow from a process of deliberative discovery 
and mutual adjustment among general stakeholder desires and particular developers judgements.

We naturaly seek a state of balance or coherence  
among these set of design decisions and particular developers judgements.

### Process Dynamics

#### Top-down vision

- What do we want?
- What do I have?
- What has to happen to get from what I have to what I want?
- What do I need to make it happen?
- verify and confirm impact.

#### Bottom-up implementation

- Given what I have which fundamental block can I tackle
- What are the clear steps involved
- What's my immediate action. (Spike, test/code loop, verify)

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
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
