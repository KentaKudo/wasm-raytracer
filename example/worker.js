import * as Comlink from "comlink";

const raytracer = {
  async render(width, height) {
    const { render } = await import("wasm-raytracer");
    const rendered = render(width, height);
    return Comlink.transfer(rendered, [rendered.buffer]);
  },
};

Comlink.expose(raytracer);
