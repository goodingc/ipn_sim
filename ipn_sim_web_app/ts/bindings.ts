import { Renderer } from "~/ts/renderer";
import { SetupData, TickData } from "~/pkg";
import { Vector3 } from "three";

let renderer: Renderer;

export function setup(data: SetupData) {
  renderer = new Renderer(document.getElementById("renderer-wrapper"), data);
}

export function tick(data: TickData) {
  renderer.update(data);
}

export function getCameraPosition(): Vector3 {
  return renderer.camera.position.clone().divideScalar(Renderer.MASTER_SCALE);
}
