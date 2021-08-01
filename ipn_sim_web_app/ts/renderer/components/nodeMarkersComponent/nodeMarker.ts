import * as THREE from "three";
import { Node, Point3, SpaceMetric } from "~/pkg";
import { CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Renderer } from "~/ts/renderer";
import { Splash } from "~/ts/renderer/components/nodeMarkersComponent/splash";

export class NodeMarker {
  mesh: THREE.Mesh;

  labelDiv: HTMLDivElement;
  label: CSS2DObject;

  path: THREE.Line;
  pathPoints: THREE.Vector3[] = [];

  splashes: Splash[] = [];

  occupancyBar: HTMLDivElement;

  updateIndex = 0;

  constructor(scene: THREE.Scene, nodeData: Node) {
    this.mesh = new THREE.Mesh();
    scene.add(this.mesh);

    this.path = new THREE.Line(
      new THREE.BufferGeometry(),
      new THREE.LineBasicMaterial({ color: 0xff0000 })
    );

    scene.add(this.path);

    // scene.add(
    //   new THREE.Line(
    //     new THREE.BufferGeometry().setFromPoints(
    //       path.map(Renderer.simPosToScenePos)
    //     ),
    //     new THREE.LineBasicMaterial({ color: 0xff0000 })
    //   )
    // );

    this.labelDiv = document.createElement("div");

    this.labelDiv.innerHTML = `
<div class="col" style="position: relative; width: 100px; height: 1.5em;">
    <div class="progress w-100 h-100 position-absolute top-0 start-0" >
        <div id="${nodeData.name}-occupancy-bar" class="progress-bar bg-primary" style="z-index: inherit"></div>
    </div>
    <div class="w-100 text-center position-absolute top-50 start-50 translate-middle">${nodeData.name}</div>
</div>
`;

    this.label = new CSS2DObject(this.labelDiv);
    this.label.position.set(0, 0, 0);
    this.mesh.add(this.label);
  }

  addSplash(color: THREE.Color, expanding = true) {
    this.splashes.push(new Splash(this.mesh, color, expanding));
  }

  update(data: Node, bufferOccupancy: number) {
    const position = Renderer.simPosToScenePos(data.position);
    this.mesh.position.copy(position);

    if (this.updateIndex % 10 == 0) {
      this.pathPoints.push(position);
      if (this.pathPoints.length > 1000) {
        this.pathPoints.shift();
      }
    }
    this.path.geometry.setFromPoints(this.pathPoints);

    for (const splash of this.splashes) {
      splash.update();
    }

    this.splashes = this.splashes.filter((splash) => splash.alive);

    this.occluded = false;

    if (this.occupancyBar == null) {
      this.occupancyBar = document.getElementById(
        `${data.name}-occupancy-bar`
      ) as HTMLDivElement;
    }
    this.occupancyBar.style.width = bufferOccupancy * 100 + "%";
    this.updateIndex++;
  }

  set occluded(occluded: boolean) {
    if (occluded) {
      this.labelDiv.style.setProperty("opacity", "0.5");
      if (this.occupancyBar != null) {
        this.occupancyBar.className = "progress-bar bg-secondary";
      }
    } else {
      this.labelDiv.style.setProperty("opacity", "1");
      if (this.occupancyBar != null) {
        this.occupancyBar.className = "progress-bar";
      }
    }
  }
}
