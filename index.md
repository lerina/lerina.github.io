
<canvas id="cnv_all" width="578" height="150"></canvas>

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
Don't butcher your way through your IT Project. everybody can write code. But not everybody can write good code.
Like wise anybody can talk about design, but not everybody can actually deliver a good design. Talent is where creativity and practice meet repetitively. Very often in the IT world, design Talents and coding Talents need to meets repetitively 

Tell me about your P.A.I.N.
=============================

All programmers and designers of the world Unite! 
I help programmers and designers partner up to make 
digital projects such as websites, web applications, and mobile apps that look and work great.

Tell me about your **P**.A.I.N.

- **P**roject or **P**roblem (to solve)

Tell me about your P.**A**.I.N.

- **A**genda/Ambition or **A**ngst/Anger (to fix)

Tell me about your P.A.**I**.N.

- **I**nspiration or **I**rritation  (to heal)

Tell me about your P.A.I.**N**.

- **N**iche and **N**eeds (to get started)


If you are a  content writers, designer or entrepreneurs you've got `P.A.I.N.` and that is great. You want to get thing out in the world. Let me help you until the <img src="./logo_lerina_16x16.png" align="bottom" /> `l`ast `R`elease.



</div>

## Some Code

```rust
fn main() {
    let mut days_left: u8 = 10;
     
    println!("{} days to startup!", days_left);
}
```

</div>

<nav>
<div style="float:left">
- Programming

    - Python
    - Rust
    - JavaScript
</div><div style="float:left;border-right:solid 1px #ddd; ">
- Web / Mobile

    - Django
    - Webassembly
    - React


</div><div style="float:right; marging-right:1px;">

- &nbsp;<img src="./logo_lerina_32x32.png" align="top" /> Hire Me

    - Availability
    - Rates
    - Legal
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
