import * as THREE from "three";
import { TickData } from "~/pkg";

export abstract class Component {
  scene: THREE.Scene;

  protected constructor(scene: THREE.Scene) {
    this.scene = scene;
  }

  abstract update(data: TickData);
}
