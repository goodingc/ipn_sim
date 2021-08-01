import { Component } from "~/ts/renderer/component";
import { SetupData, TickData } from "~/pkg";
import * as THREE from "three";
import { NodeMarker } from "~/ts/renderer/components/nodeMarkersComponent/nodeMarker";
import { zip } from "~/ts/utils";
import { BodyMarker } from "~/ts/renderer/components/bodyMarkersComponent/bodyMarker";

export class BodyMarkersComponent extends Component {
  markers: BodyMarker[];

  constructor(scene: THREE.Scene, setupData: SetupData) {
    super(scene);

    this.markers = setupData.bodies.map(
      (body) => new BodyMarker(this.scene, body)
    );
  }

  update(data: TickData) {
    for (const [marker, body] of zip(this.markers, data.bodies)) {
      marker.update(body);
    }
  }
}
