---
title: Invader -  Demo
subtitle: A Rust/Wasm code & Demo
author:
- lerina j-y Razafy
toc-depth: 3
keywords: [Rust, Wasm, Webassembly]
---

<style>canvas {box-shadow: black 20px 10px 50px}</style>

<main>

<h1> Invaders</h1>
<div id="Level">Level:<span id="level"></span></div>
<div id="scoring">Score:<span id="score"></span></div>
<canvas id="canvas" style="outline: none" tabindex="0" height="600" width="600">
Your browser does not support the Canvas.
</canvas>

</main>

# <a href="../index.html">⇦ home</a> {.collapse}

<script type="module">
import init from "./pkg/wasm_shooting_game.js";

  async function run() {
    await init();
  }

  run();
</script>
