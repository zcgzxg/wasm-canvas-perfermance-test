const iteration = Number(
  new URL(location.href).searchParams.get("count") || 1000
);

import("./pkg")
  .then((wasm) => {
    draw();

    wasm.draw(iteration, () => {
      console.log(`iteration: ${iteration}
        
        wasm init: ${performance.getEntriesByName("wasm init")[0].duration}
        js init: ${performance.getEntriesByName("js init")[0].duration}

        wasm draw: ${performance.getEntriesByName("wasm draw")[0].duration}
        js draw: ${performance.getEntriesByName("js draw")[0].duration}
      `);
    });
  })
  .catch(console.error);

function draw() {
  let perf = performance;
  perf.mark("start-init");
  /** @type {HTMLCanvasElement} */
  let canvas = document.getElementById("canvas2");
  let ctx = canvas.getContext("2d");
  perf.measure("js init", "start-init");

  perf.mark("start-draw");
  for (let i = 0; i < iteration; i++) {
    ctx.clearRect(0.0, 0.0, canvas.width, canvas.height);
    ctx.fillStyle = "#000000";
    ctx.fillRect(0.0, 0.0, canvas.width, canvas.height);

    ctx.strokeStyle = "#feccaa";
    ctx.lineWidth = 3;
    ctx.strokeRect(i, 30.0, i, 60.0);
    ctx.arc(i, i, i * 20, 0.0, 6.28);
    ctx.stroke();
  }
  perf.measure("js draw", "start-draw");
}
