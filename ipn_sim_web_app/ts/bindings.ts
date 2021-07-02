import { Renderer } from "~/ts/renderer";

let renderer: Renderer;

export function setup() {
  renderer = new Renderer(document.getElementById("renderer-wrapper"));
}
