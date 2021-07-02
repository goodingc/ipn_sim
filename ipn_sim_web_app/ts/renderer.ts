import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { CSS2DRenderer } from "three/examples/jsm/renderers/CSS2DRenderer";

export class Renderer {
  scene: THREE.Scene;
  camera: THREE.Camera;
  cameraControls: OrbitControls;
  sceneRenderer: THREE.WebGLRenderer;
  labelRenderer: CSS2DRenderer;

  constructor(renderWrapper: HTMLElement) {
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

    this.camera.position.y = 5;
    this.cameraControls.update();

    this.scene = new THREE.Scene();

    this.scene.add(new THREE.AxesHelper(1));

    this.render();
  }

  render() {
    requestAnimationFrame(() => this.render());
    this.sceneRenderer.render(this.scene, this.camera);
    this.cameraControls.update();
  }
}
