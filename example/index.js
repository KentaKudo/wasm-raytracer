const WIDTH = 400;
const HEIGHT = 225;

const canvas = document.getElementById("raytracer-canvas");
canvas.width = WIDTH;
canvas.height = HEIGHT;

if (window.Worker) {
  const w = new Worker(new URL("./worker.js", import.meta.url));
  w.postMessage([WIDTH, HEIGHT]);
  w.onmessage = function (e) {
    const imageData = new ImageData(e.data, WIDTH);
    canvas.getContext("2d").putImageData(imageData, 0, 0);
  };
}
