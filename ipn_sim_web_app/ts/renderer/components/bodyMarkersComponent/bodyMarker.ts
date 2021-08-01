import * as THREE from "three";
import { CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Body } from "~/pkg";
import { Renderer } from "~/ts/renderer";
import { getBodyMesh } from "./bodyMesh";

export class BodyMarker {
  static BODY_SCALE = 1;

  mesh: THREE.Mesh;
  label: CSS2DObject;

  constructor(scene: THREE.Scene, bodyData: Body) {
    this.mesh = getBodyMesh(bodyData);
    scene.add(this.mesh);

    const labelDiv = document.createElement("div");
    labelDiv.className = "text-light";
    labelDiv.textContent = bodyData.name;
    this.label = new CSS2DObject(labelDiv);
    this.label.position.set(0, 0, 0);
    this.mesh.add(this.label);
  }

  update(data: Body) {
    this.mesh.position.copy(Renderer.simPosToScenePos(data.position));
  }
}
