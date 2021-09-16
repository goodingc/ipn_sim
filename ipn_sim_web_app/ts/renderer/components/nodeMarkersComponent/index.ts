import { Component } from "~/ts/renderer/component";
import { SetupData, TickData } from "~/pkg";
import * as THREE from "three";
import { NodeMarker } from "~/ts/renderer/components/nodeMarkersComponent/nodeMarker";
import { zip } from "~/ts/utils";

export class NodeMarkersComponent extends Component {
  markers: NodeMarker[];

  constructor(scene: THREE.Scene, setupData: SetupData) {
    super(scene);

    this.markers = zip(setupData.nodes).map(
      ([node, path]) => new NodeMarker(this.scene, node)
    );
  }

  update(data: TickData) {
    for (const [marker, node, messageBufferOccupancy] of zip(
      this.markers,
      data.nodes,
      data.messageBufferOccupancies
    )) {
      marker.update(node, messageBufferOccupancy);
      marker.occluded = false;
      marker.highlighted = false;
    }

    for (const creatingNodeIndex of data.creatingNodeIndices) {
      this.markers[creatingNodeIndex].addSplash(new THREE.Color("green"));
    }

    for (const creatingNodeIndex of data.deliveringNodeIndices) {
      this.markers[creatingNodeIndex].addSplash(
        new THREE.Color("green"),
        false
      );
    }

    for (const occludedNodeIndex of data.occludedNodeIndices) {
      this.markers[occludedNodeIndex].occluded = true;
    }

    if (data.highlightedNodeIndex != null) {
      this.markers[data.highlightedNodeIndex].highlighted = true
    }
  }
}
