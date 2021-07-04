import * as THREE from "three";
import { Node } from "~/pkg";
import { CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Entity } from "~/ts/entity";
import { Renderer } from "~/ts/renderer";

export class NodeMarker extends Entity<Node> {
  mesh: THREE.Mesh;
  label: CSS2DObject;

  constructor(scene: THREE.Scene, nodeData: Node) {
    super();
    this.mesh = new THREE.Mesh();
    scene.add(this.mesh);

    const labelDiv = document.createElement("div");
    labelDiv.className = "label";
    labelDiv.textContent = nodeData.name;
    this.label = new CSS2DObject(labelDiv);
    this.label.position.set(0, 0, 0);
    this.mesh.add(this.label);
  }

  update(data: Node) {
    this.mesh.position.copy(Renderer.simPosToScenePos(data.position));
  }
}
