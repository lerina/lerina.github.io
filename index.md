<!-- should be in head 
<link rel="prefetch" href=" http://lerina.github.io/hire_me ">
-->
<canvas id="cnv_all" width="578" height="150"></canvas>
<script src="./js/cnv01.js"></script>

<div class="container">

<!--
<nav>
<canvas id="cnv_left" width="192" height="150" style="border:solid 1 red"></canvas>
<canvas id="cnv_center" width="192" height="150"></canvas>
<canvas id="cnv_right" width="192" height="150"></canvas>
</nav>
-->

<nav> 

-   [Home](/)
-   [Programming](/prog)
-   [Blog](/blog)
-   [About](/about)
-   [Faq](/faq)

</nav>

<div>

Hello, ... friend?
If you believe programmers and designers ought to partner up to make 
digital projects such as websites, web applications, and mobile apps that look and work great, you've reach the right place

Tell me about your **P.A.I.N.** <span id="PAIN"></span>

- **P**roject or **P**roblem (to solve) <a href="./#Pain"> more ...</a>
- **A**genda/Ambition or **A**ngst/Anger (to fix) <a href="./#pAin"> more ...</a>
- **I**nspiration or **I**rritation  (to heal) <a href="./#paIn"> more ...</a>
- **N**iche and **N**eeds (to get started) <a href="./#paiN"> more ...</a>

Don't butcher your way through your IT Project. everybody can write code. But not everybody can write good code.
Like wise anybody can talk about design, but not everybody can actually deliver a good design. Talent is where creativity and practice meet repetitively. Very often in the IT world, design Talents and coding Talents need to meets repetitively 

Tell me about your P.A.I.N.
=============================

<span id="Pain">**P**roject or **P**roblem (to solve)</span>

<a href="./#PAIN">back ...</a> 

<span id="pAin">**A**genda/Ambition or **A**ngst/Anger (to fix)</span>

<a href="./#PAIN">back ...</a> 

<span id="PaIn">**I**nspiration or **I**rritation  (to heal)</span>

<a href="./#PAIN">back ...</a> 

<span id="PaiN">**N**iche and **N**eeds (to get started)</span>

<a href="./#PAIN">back ...</a> 


If you are a content writer, designer or entrepreneur you've got `P.A.I.N.` and that is great. 
You want to get thing out in the world. Perhaps we may work together, until that <img src="./logo_lerina_16x16.png" align="bottom" /> `l`ast `R`elease.



</div>

## Some Code

```rust
fn main() {
    let mut days_left: u8 = 10;
     
    println!("{} days to startup!", days_left);
}
```

“Everybody in this country should learn how to program a computer … because it teaches you how to think.”   
~ Steve Jobs

“Everybody in this country should learn how to think … programming a computer is one option.”   
~ lerina
</div>

<nav>
<div style="float:left">
- Programming

    - Python
    - Rust
    - JavaScript
</div><div style="float:left;border-right:solid 1px #ddd; padding-right:3px;">
- Web / Mobile

    - Django
    - Webassembly
    - React


</div><div style="float:left; padding-left:3px;">

- &nbsp;<img src="./logo_lerina_32x32.png" align="top" /> Hire Me

    - Availability
    - Rates
    - Legal

I enjoy working with others to produce a better outcome.
I try to find the best fitting technology for a given problem. 
While my professional background is mostly Python, thanks to Webassembly I have reconnected
with JavaScript and fallen in love with Rust.

</div>
</nav>


<footer style="clear:both">


-   [zoom]()
-   [email](mailto:lerina.razafy@gmail.com)
-   [github.com/lerina](https://github.com/lerina)

<div id="copy">
<em>&#xa9;</em> 2018  &nbsp; <a href="http://razafy.com" target="_blank"> <span class="le">le</span><span class="ri">ri</span><span class="na">na</span>  ^_^ </a></div>

</footer>

<script>
/*
  var cnv_left = document.getElementById('cnv_left');
  var ctx_left = cnv_left.getContext('2d');

      ctx_left.fillStyle = 'black';
      ctx_left.fillRect(0, 0, 200, 150);
      ctx_left.fill();
 
  var cnv_center = document.getElementById('cnv_center');
      var ctx_center = cnv_center.getContext('2d');

      ctx_center.fillStyle = 'black';
      ctx_center.fillRect(0, 0, 200, 150);
      ctx_center.fill();

      ctx_center.strokeStyle = 'white';
      ctx_center.beginPath();
      ctx_center.moveTo(0, 0);
      ctx_center.lineTo(200, 150);
      ctx_center.stroke();
 
  var cnv_right = document.getElementById('cnv_right');
  var ctx_right = cnv_right.getContext('2d');

      ctx_right.fillStyle = 'black';
      ctx_right.fillRect(0, 0, 200, 150);
      ctx_right.fill();
*/

  var cnv_all = document.getElementById('cnv_all');
  var ctx_all = cnv_all.getContext('2d');

      ctx_all.fillStyle = 'black';
      ctx_all.fillRect(0, 0, 600, 150);
      ctx_all.fill();

cnv_all.style.width = window.innerWidth - (window.innerWidth *30/100) + 'px';

// cnv_all.style.display="none"; // it hides the entire element
// visibility:hidden means that the contents of the element will be invisible, but the element stays in its original position and size.

window.addEventListener("resize", OnResizeCalled, false);

function OnResizeCalled() {
    var newsize = window.innerWidth - (window.innerWidth *30/100);
    cnv_all.style.width = newsize + 'px';
    cnv_all.style.height = '150px';

    //cnv_all.style.height = window.innerHeight + 'px';

cnv_all.style.display="block";

/*
cnv_left.style.display="none";
cnv_center.style.display="none";
cnv_right.style.display="none";
*/
} 
</script>
