import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { CSS2DRenderer } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Point3, SetupData, SpaceMetric, TickData } from "~/pkg";
import { Entity } from "~/ts/entity";
import { NodeMarker } from "~/ts/nodeMarker";
import { zip } from "~/ts/utils";

export class Renderer extends Entity<TickData> {
  static MASTER_SCALE = 1 / 1.496e11;

  scene: THREE.Scene;
  camera: THREE.Camera;
  cameraControls: OrbitControls;
  sceneRenderer: THREE.WebGLRenderer;
  labelRenderer: CSS2DRenderer;

  nodeMarkers: NodeMarker[];

  constructor(renderWrapper: HTMLElement, setupData: SetupData) {
    super();
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

    this.nodeMarkers = setupData.nodes.map(
      (node) => new NodeMarker(this.scene, node)
    );

    this.render();
  }

  render() {
    requestAnimationFrame(() => this.render());
    this.sceneRenderer.render(this.scene, this.camera);
    this.labelRenderer.render(this.scene, this.camera);
  }

  update(data: TickData) {
    for (const [marker, node] of zip(this.nodeMarkers, data.nodes)) {
      marker.update(node);
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
