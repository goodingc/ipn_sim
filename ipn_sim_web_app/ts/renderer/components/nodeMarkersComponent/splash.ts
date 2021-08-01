import { CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import * as THREE from "three";

export class Splash {
  cssObject: CSS2DObject;
  life = 1;
  htmlElement: HTMLDivElement;
  alive = true;

  constructor(
    public mesh: THREE.Mesh,
    public color: THREE.Color,
    public expanding = true
  ) {
    this.htmlElement = document.createElement("div");
    this.htmlElement.style.borderRadius = "50%";
    this.htmlElement.style.borderStyle = "solid";
    this.htmlElement.style.zIndex = "1";
    this.cssObject = new CSS2DObject(this.htmlElement);
    this.updateStyle();

    this.mesh.add(this.cssObject);
  }

  updateStyle() {
    this.htmlElement.style.borderColor = `rgba(${this.color.r * 255}, ${
      this.color.g * 255
    }, ${this.color.b * 255}, ${this.life * 2})`;
    const size = 100 * (this.expanding ? 1 - this.life : this.life);
    this.htmlElement.style.width = size + "px";
    this.htmlElement.style.height = size + "px";
    this.htmlElement.style.borderWidth =
      10 * (this.expanding ? this.life : 1 - this.life) + "px";
  }

  update() {
    this.life *= 0.9;
    this.updateStyle();
    if (this.life < 0.01) {
      this.alive = false;
      this.mesh.remove(this.cssObject);
    }
  }
}
