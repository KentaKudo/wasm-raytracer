onmessage = async function (e) {
  const { render } = await import("wasm-raytracer");
  const rendered = render(e.data[0], e.data[1]);
  postMessage(rendered);
};
