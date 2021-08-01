import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { CSS2DRenderer } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Point3, SetupData, SpaceMetric, TickData } from "~/pkg";
import { Component } from "~/ts/renderer/component";
import { NodeMarkersComponent } from "~/ts/renderer/components/nodeMarkersComponent";
import { ConnectionMarkersComponent } from "~/ts/renderer/components/connectionMarkersComponent";
import { BodyMarkersComponent } from "~/ts/renderer/components/bodyMarkersComponent";
export class Renderer {
  static MASTER_SCALE = 1e5 / 1.496e11;

  scene: THREE.Scene;
  camera: THREE.Camera;
  cameraControls: OrbitControls;
  sceneRenderer: THREE.WebGLRenderer;
  labelRenderer: CSS2DRenderer;

  components: Component[];

  constructor(renderWrapper: HTMLElement, setupData: SetupData) {
    this.sceneRenderer = new THREE.WebGLRenderer();
    this.sceneRenderer.setSize(
      renderWrapper.clientWidth,
      renderWrapper.clientHeight
    );
    renderWrapper.appendChild(this.sceneRenderer.domElement);

    this.labelRenderer = new CSS2DRenderer();
    this.labelRenderer.setSize(
      renderWrapper.clientWidth,
      renderWrapper.clientHeight
    );
    this.labelRenderer.domElement.style.position = "absolute";
    this.labelRenderer.domElement.style.top = "0px";
    renderWrapper.appendChild(this.labelRenderer.domElement);

    this.camera = new THREE.PerspectiveCamera(
      75,
      renderWrapper.clientWidth / renderWrapper.clientHeight
    );
    this.cameraControls = new OrbitControls(
      this.camera,
      this.labelRenderer.domElement
    );

    this.camera.position.y = 7;
    this.cameraControls.update();

    this.scene = new THREE.Scene();

    this.scene.add(new THREE.AxesHelper(1));

    this.components = [
      new NodeMarkersComponent(this.scene, setupData),
      new ConnectionMarkersComponent(this.scene, setupData),
      new BodyMarkersComponent(this.scene, setupData),
    ];

    this.render();
  }

  render() {
    requestAnimationFrame(() => this.render());
    this.sceneRenderer.render(this.scene, this.camera);
    this.labelRenderer.render(this.scene, this.camera);
  }

  update(data: TickData) {
    for (const component of this.components) {
      component.update(data);
    }
  }

  static simPosToScenePos(statePosition: Point3<SpaceMetric>): THREE.Vector3 {
    return new THREE.Vector3(
      statePosition.x * Renderer.MASTER_SCALE,
      statePosition.y * Renderer.MASTER_SCALE,
      statePosition.z * Renderer.MASTER_SCALE
    );
  }
}
