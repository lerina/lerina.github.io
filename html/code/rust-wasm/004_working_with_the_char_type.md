<div class="navbar"><a class="openbtn" onclick="openNav()">&#9776;</a></div>
<main>

[`<--` Importing non-browser JS ](./003_importing_non-browser_JS.html)

## Working with the char type


1. Make the file structure

```
cargo new char --lib
cd char
mkdir -p www/html www/js
```

2. Edit Cargo.toml, add crate-type and wasm-bindgen dependency

```toml
[package]
name = "char"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wasm-bindgen = "0.2.88"

```

3. cut and paste the import-js example from github [src/lib.rs](https://github.com/rustwasm/wasm-bindgen/blob/main/examples/char/src/lib.rs)

or the rust code in 

[wasm-bindgen: char](https://rustwasm.github.io/wasm-bindgen/examples/char.html)


```rust
// src/lib.rs

use wasm_bindgen::prelude::*;

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Counter {
    key: char,
    count: i32,
}

#[wasm_bindgen]
impl Counter {
    pub fn new(key: char, count: i32) -> Counter {
        log(&format!("Counter::new({}, {})", key, count));
        Counter { key, count }
    }

    pub fn key(&self) -> char {
        log("Counter.key()");
        self.key
    }

    pub fn count(&self) -> i32 {
        log("Counter.count");
        self.count
    }

    pub fn increment(&mut self) {
        log("Counter.increment");
        self.count += 1;
    }

    pub fn update_key(&mut self, key: char) {
        self.key = key;
    }
}

```

4. create the index file at `www/html/index.html`:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Wasm no NPM no Webpack</title>
</head>
<body>


  <script type="module" src="../js/index.js"></script>
</body>
</html>
```

5. Create the initial `index.js`


```javascript
// www/js/index.js

import init from "../pkg/char.js";

async function run() {
    const wasm = await init();
}

run();
```


Get the `char-list.js` file

```js
export let chars = [
    '!','#','$','%','&','\'','(',')','*','+',',',
    '-','.','/','0','1','2','3','4','5','6','7',
    '8','9',':',';','<','=','>','?','@','A','B',
    'C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X',
    'Y','Z','[',']','^','_','`','a','b','c',
    'd','e','f','g','h','i','j','k','l','m','n',
    'o','p','q','r','s','t','u','v','w','x','y',
    'z','{','|','}','~',' ','¡','¢','£','¤','¥',
    '¦','§','¨','©','ª','«','¬','®','¯','°',
    '±','²','³','´','µ','¶','·','¸','¹','º','»',
    '¼','½','¾','¿','À','Á','Â','Ã','Ä','Å','Æ',
    'Ç','È','É','Ê','Ë','Ì','Í','Î','Ï','Ð','Ñ',
    'Ò','Ó','Ô','Õ','Ö','×','Ø','Ù','Ú','Û','Ü',
    'Ý','Þ','ß','à','á','â','ã','ä','å','æ','ç',
    'è','é','ê','ë','ì','í','î','ï','ð','ñ','ò',
    'ó','ô','õ','ö','÷','ø','ù','ú','û','ü','ý',
    'þ','ÿ','Ā','ā','Ă','ă','Ą','ą','Ć','ć','Ĉ',
    'ĉ','Ċ','ċ','Č','č','Ď','ď','Đ','đ','Ē','ē',
    'Ĕ','ĕ','Ė','ė','Ę','ę','Ě','ě','Ĝ','ĝ','Ğ',
    'ğ','Ġ','ġ','Ģ','ģ','Ĥ','ĥ','Ħ','ħ','Ĩ','ĩ',
    'Ī','ī','Ĭ','ĭ','Į','į','İ','ı','Ĳ','ĳ','Ĵ',
    'ĵ','Ķ','ķ','ĸ','Ĺ','ĺ','Ļ','ļ','Ľ','ľ','Ŀ',
    'ŀ','Ł','ł','Ń','ń','Ņ','ņ','Ň','ň','ŉ','Ŋ',
    'ŋ','Ō','ō','Ŏ','ŏ','Ő','ő','Œ','œ','Ŕ','ŕ',
    'Ŗ','ŗ','Ř','ř','Ś','ś','Ŝ','ŝ','Ş','ş','Š',
    'š','Ţ','ţ','Ť','ť','Ŧ','ŧ','Ũ','ũ','Ū','ū',
    'Ŭ','ŭ','Ů','ů','Ű','ű','Ų','ų','Ŵ','ŵ','Ŷ',
    'ŷ','Ÿ','Ź','ź','Ż','ż','Ž','ž','ſ','ƀ','Ɓ',
    'Ƃ','ƃ','Ƅ','ƅ','Ɔ','Ƈ','ƈ','Ɖ','Ɗ','Ƌ','ƌ',
    'ƍ','Ǝ','Ə','Ɛ','Ƒ','ƒ','Ɠ','Ɣ','ƕ','Ɩ','Ɨ',
    'Ƙ','ƙ','ƚ','ƛ','Ɯ','Ɲ','ƞ','Ɵ','Ơ','ơ','Ƣ',
    'ƣ','Ƥ','ƥ','Ʀ','Ƨ','ƨ','Ʃ','ƪ','ƫ','Ƭ','ƭ',
    'Ʈ','Ư','ư','Ʊ','Ʋ','Ƴ','ƴ','Ƶ','ƶ','Ʒ','Ƹ',
    'ƹ','ƺ','ƻ','Ƽ','ƽ','ƾ','ƿ','ǀ','ǁ','ǂ','ǃ',
    'Ǆ','ǅ','ǆ','Ǉ','ǈ','ǉ','Ǌ','ǋ','ǌ','Ǎ','ǎ',
    'Ǐ','ǐ','Ǒ','ǒ','Ǔ','ǔ','Ǖ','ǖ','Ǘ','ǘ','Ǚ',
    'ǚ','Ǜ','ǜ','ǝ','Ǟ','ǟ','Ǡ','ǡ','Ǣ','ǣ','Ǥ',
    'ǥ','Ǧ','ǧ','Ǩ','ǩ','Ǫ','ǫ','Ǭ','ǭ','Ǯ','ǯ',
    'ǰ','Ǳ','ǲ','ǳ','Ǵ','ǵ','Ƕ','Ƿ','Ǹ','ǹ','Ǻ',
    'ǻ','Ǽ','ǽ','Ǿ','ǿ','Ȁ','ȁ','Ȃ','ȃ','Ȅ','ȅ',
    'Ȇ','ȇ','Ȉ','ȉ','Ȋ','ȋ','Ȍ','ȍ','Ȏ','ȏ','Ȑ',
    'ȑ','Ȓ','ȓ','Ȕ','ȕ','Ȗ','ȗ','Ș','ș','Ț','ț',
    'Ȝ','ȝ','Ȟ','ȟ','Ƞ','ȡ','Ȣ','ȣ','Ȥ','ȥ','Ȧ',
    'ȧ','Ȩ','ȩ','Ȫ','ȫ','Ȭ','ȭ','Ȯ','ȯ','Ȱ','ȱ',
    'Ȳ','ȳ','ȴ','ȵ','ȶ','ȷ','ȸ','ȹ','Ⱥ','Ȼ','ȼ',
    'Ƚ','Ⱦ','ȿ','ɀ','Ɂ','ɂ','Ƀ','Ʉ','Ʌ','Ɇ','ɇ',
    'Ɉ','ɉ','Ɋ','ɋ','Ɍ','ɍ','Ɏ','ɏ','ɐ','ɑ','ɒ',
    'ɓ','ɔ','ɕ','ɖ','ɗ','ɘ','ə','ɚ','ɛ','ɜ','ɝ',
    'ɞ','ɟ','ɠ','ɡ','ɢ','ɣ','ɤ','ɥ','ɦ','ɧ','ɨ',
    'ɩ','ɪ','ɫ','ɬ','ɭ','ɮ','ɯ','ɰ','ɱ','ɲ','ɳ',
    'ɴ','ɵ','ɶ','ɷ','ɸ','ɹ','ɺ','ɻ','ɼ','ɽ','ɾ',
    'ɿ','ʀ','ʁ','ʂ','ʃ','ʄ','ʅ','ʆ','ʇ','ʈ','ʉ',
    'ʊ','ʋ','ʌ','ʍ','ʎ','ʏ','ʐ','ʑ','ʒ','ʓ','ʔ',
    'ʕ','ʖ','ʗ','ʘ','ʙ','ʚ','ʛ','ʜ','ʝ','ʞ','ʟ',
    'ʠ','ʡ','ʢ','ʣ','ʤ','ʥ','ʦ','ʧ','ʨ','ʩ','ʪ',
    'ʫ','ʬ','ʭ','ʮ','ʯ','Ͳ','ͳ','ʹ','͵','Ͷ','ͷ',
    'ͺ','ͻ','ͼ','ͽ',';','Ϳ','΄','΅','Ά','·','Έ','Ή',
    'Ί','Ό','Ύ','Ώ',
    'ΐ','Α','Β','Γ','Δ','Ε','Ζ','Η','Θ','Ι','Κ',
    'Λ','Μ','Ν','Ξ','Ο','Π','Ρ','Σ','Τ','Υ','Φ',
    'Χ','Ψ','Ω','Ϊ','Ϋ','ά','έ','ή','ί','ΰ','α',
    'β','γ','δ','ε','ζ','η','θ','ι','κ','λ','μ',
    'ν','ξ','ο','π','ρ','ς','σ','τ','υ','φ','χ',
    'ψ','ω','ϊ','ϋ','ό','ύ','ώ','Ϗ','ϐ','ϑ','ϒ',
    'ϓ','ϔ','ϕ','ϖ','ϗ','Ϙ','ϙ','Ϛ','ϛ','Ϝ','ϝ',
    'Ϟ','ϟ','Ϡ','ϡ','Ϣ','ϣ','Ϥ','ϥ','Ϧ','ϧ','Ϩ',
    'ϩ','Ϫ','ϫ','Ϭ','ϭ','Ϯ','ϯ','ϰ','ϱ','ϲ','ϳ',
    'Օ','Ֆ','🕧','🕨','🕩','🕪','🕫','🕬','🕭','🕮',
    '🕯','🕰','🕱','🕲','🕳','🕴','🕵','🕶','🕷','🕸',
    '🕹','🕺','🕻','🕼','🕽','🕾','🕿','🖀','🖁','🖂',
    '🖃','🖄','🖅','🖆','🖇','🖈','🖉','🖊','🖋',
    '🖌','🖍','🖎','🖏','🖐','🖑','🖒','🖓','🖔','🖕',
    '🖖','🖗','🖘','🖙','🖚','🖛','🖜','🖝','🖞','🖟',
    '🖠','🖡','🖢','🖣','🖤','🖥','🖦','🖧','🖨','🖩',
    '🖪','🖫','🖬','🖭','🖮','🖯','🖰','🖱','🖲','🖳',
    '🖴','🖵','🖶','🖷','🖸','🖹','🖺','🖻','🖼','🖽',
    '🖾','🖿','🗀','🗁','🗂','🗃','🗄','🗅','🗆','🗇',
    '🗈','🗉','🗊','🗋','🗌','🗍','🗎','🗏','🗐','🗑','🗒',
    '🗓','🗔','🗕','🗖','🗗','🗘','🗙','🗚','🗛','🗜',
    '🗝','🗞','🗟','🗠','🗡','🗢','🗣','🗤','🗥','🗦',
    '🗧','🗨','🗩','🗪','🗫','🗬','🗭','🗮','🗯','🗰',
    '🗱','🗲','🗳','🗴','🗵','🗶','🗷','🗸','🗹','🗺',
    '🗻','🗼','🗽','🗾','🗿','😀'];


```

Next modify the code that goes in `index.js`

```js
import init, { Counter } from "../pkg/char.js";
import { chars } from './chars-list.js';

let counters = [];
async function run() {
    const wasm = await init();

    addCounter();
    let b = document.getElementById('add-counter');
    alert("b");
    if (!b) throw new Error('Unable to find #add-counter');
    b.addEventListener('click', ev => addCounter());
}

run();

//let imp = import('./pkg');
//let mod;


function addCounter() {
    let ctr = Counter.new(randomChar(), 0);
    counters.push(ctr);
    update();
    console.log("toto");
}

function update() {
    let container = document.getElementById('container');
    if (!container) throw new Error('Unable to find #container in dom');
    while (container.hasChildNodes()) {
        if (container.lastChild.id == 'add-counter') break;
        container.removeChild(container.lastChild);
    }
    for (var i = 0; i < counters.length; i++) {
        let counter = counters[i];
        container.appendChild(newCounter(counter.key(), counter.count(), ev => {
            counter.increment();
            update();
        }));
    }
}

function randomChar() {
    console.log('randomChar');
    let idx = Math.floor(Math.random() * (chars.length - 1));
    console.log('index', idx);
    let ret = chars.splice(idx, 1)[0];
    console.log('char', ret);
    return ret;
}

function newCounter(key, value, cb) {
    let container = document.createElement('div');
    container.setAttribute('class', 'counter');
    let title = document.createElement('h1');
    title.appendChild(document.createTextNode('Counter ' + key));
    container.appendChild(title);
    container.appendChild(newField('Count', value));
    let plus = document.createElement('button');
    plus.setAttribute('type', 'button');
    plus.setAttribute('class', 'plus-button');
    plus.appendChild(document.createTextNode('+'));
    plus.addEventListener('click', cb);
    container.appendChild(plus);
    return container;
}

function newField(key, value) {
    let ret = document.createElement('div');
    ret.setAttribute('class', 'field');
    let name = document.createElement('span');
    name.setAttribute('class', 'name');
    name.appendChild(document.createTextNode(key));
    ret.appendChild(name);
    let val = document.createElement('span');
    val.setAttribute('class', 'value');
    val.appendChild(document.createTextNode(value));
    ret.appendChild(val);
    return ret;
}

```

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
