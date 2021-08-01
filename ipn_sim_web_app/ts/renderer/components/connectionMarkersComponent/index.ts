import { Component } from "~/ts/renderer/component";
import { SetupData, TickData } from "~/pkg";
import * as THREE from "three";
import { Renderer } from "~/ts/renderer";
import { ConnectionMarker } from "./connectionMarker";

export class ConnectionMarkersComponent extends Component {
  markers: ConnectionMarker[][];

  constructor(scene: THREE.Scene, setupData: SetupData) {
    super(scene);
    this.markers = [];
  }

  update(data: TickData) {
    for (const markers of this.markers) {
      if (markers != null) {
        for (const marker of markers) {
          if (marker != null) {
            marker.update();
            marker.visible = false;
          }
        }
      }
    }

    for (const [index1, index2] of data.connectableNodeIndices) {
      const marker = this.getMarker(index1, index2);
      marker.visible = true;
      marker.points = [
        Renderer.simPosToScenePos(data.nodes[index1].position),
        Renderer.simPosToScenePos(data.nodes[index2].position),
      ];
      // line.material = ConnectionMarkersComponent.connectingLineMaterial;
    }

    for (const [index1, index2] of data.sendingNodeIndices) {
      const line = this.getMarker(index1, index2);
      line.sending = 1;
      // line.material = ConnectionMarkersComponent.sendingLineMaterial;
    }
  }

  getMarker(index1: number, index2: number): ConnectionMarker {
    let markers = this.markers[index1];
    if (markers == null) {
      markers = this.markers[index1] = [];
    }
    let marker = markers[index2];
    if (marker == null) {
      marker = markers[index2] = new ConnectionMarker(this.scene);
    }
    return marker;
  }
}
