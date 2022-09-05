import init, {World} from "../pkg/sine_snake.js";

async function run() {
    await init();

    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext('2d');
    const world = new World(600, 600);
    let circle = world.circle;

    function draw_circle() {
      circle = world.circle;
      //DEBUG:console.log("circle.x = " + circle.x);

      ctx.beginPath();
      ctx.arc(circle.x, circle.y, circle.radius, 
              circle.start_angle, circle.end_angle, 
              circle.counter_clockwise);
      ctx.closePath();

      ctx.fillStyle = 'rgba(177, 0, 129, .1)';
      ctx.fill();

      ctx.lineWidth = 3;
      ctx.strokeStyle = '#003300';
      ctx.stroke();

    }

    function draw() {
	    ctx.clearRect(0, 0, world.width, world.height);
	    ctx.fillStyle = '#F6F6F6';
	    ctx.fillRect(0, 0, world.width, world.height);

        draw_circle();
        world.update();
        
	    // call the draw function again!
	    requestAnimationFrame(draw);
    }
requestAnimationFrame(draw);
}

run();
