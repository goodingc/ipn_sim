import * as THREE from "three";

export class ConnectionMarker {
  static connectingLineColor = new THREE.Color(0x0000ff);
  static sendingLineColor = new THREE.Color(0x00ff00);

  line: THREE.Line;
  lineMaterial: THREE.LineBasicMaterial;
  sending: number;

  constructor(scene: THREE.Scene) {
    this.lineMaterial = new THREE.LineBasicMaterial({
      color: ConnectionMarker.connectingLineColor.clone(),
    });
    this.line = new THREE.Line(undefined, this.lineMaterial);
    scene.add(this.line);
    this.sending = 0;
  }

  set points(points: THREE.Vector3[]) {
    this.line.geometry.setFromPoints(points);
  }

  set visible(visible: boolean) {
    this.line.visible = visible;
  }

  update() {
    this.sending *= 0.9;
    this.lineMaterial.color = ConnectionMarker.connectingLineColor
      .clone()
      .lerp(ConnectionMarker.sendingLineColor, this.sending);
  }
}
