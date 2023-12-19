<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

# dev notes

## Fundamentals

- [conditionals_and_error_handling](conditionals_and_error_handling.html)
- [input_output](input_output.html)

## Functional_like

- [loops_and_iteration](loops_and_iteration.html)

## OOP_like

- [impl]
- [traits_and_generics](traits_and_generics.html)

## Meta-programming

- [macros_and_attributes](macros_and_attributes.html)

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
<li><a href="../../../../../index.html">⇦ home</a></li>
<li><a href="../../index.html">code</a></li>
<li><a href="../index.html">Rust / Wasm</a></li>
</ul>
</div>`;
document.getElementById("TOC").prepend(navCrumbs); 
</script>
