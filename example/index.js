import * as Comlink from "comlink";

const WIDTH = 400;
const HEIGHT = 225;

const canvas = document.getElementById("raytracer-canvas");
canvas.width = WIDTH;
canvas.height = HEIGHT;

(async function init() {
  if (window.Worker) {
    const rt = Comlink.wrap(
      new Worker(new URL("./worker.js", import.meta.url))
    );
    const img = await rt.render(WIDTH, HEIGHT);
    const imageData = new ImageData(img, WIDTH);
    canvas.getContext("2d").putImageData(imageData, 0, 0);
  }
})();
