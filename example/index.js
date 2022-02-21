import { render } from "wasm-raytracer";

const WIDTH = 1200;
const HEIGHT = 775;

const canvas = document.getElementById("raytracer-canvas");
canvas.width = WIDTH;
canvas.height = HEIGHT;

const imageData = new ImageData(render(WIDTH, HEIGHT), WIDTH);
canvas.getContext("2d").putImageData(imageData, 0, 0);
