import { Renderer } from "~/ts/renderer";
import { SetupData, TickData } from "~/pkg";

let renderer: Renderer;

export function setup(data: SetupData) {
  renderer = new Renderer(document.getElementById("renderer-wrapper"), data);
}

export function tick(data: TickData) {
  renderer.update(data);
}
