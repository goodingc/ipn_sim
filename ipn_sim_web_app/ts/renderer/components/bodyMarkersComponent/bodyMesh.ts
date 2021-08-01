import * as THREE from "three";
import { Renderer } from "~/ts/renderer";
import { BodyMarker } from "./bodyMarker";
import { Body } from "~/pkg";

const defaultMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const textureLoader = new THREE.TextureLoader();

export function getBodyMesh(body: Body): THREE.Mesh {
  const name = body.name.toLowerCase();
  const materialFactory = materialFactories[name];
  return new THREE.Mesh(
    getSphereGeometry(body.radius),
    materialFactory ? materialFactory(name) : defaultMaterial
  );
}

function getSphereGeometry(radius: number) {
  return new THREE.SphereGeometry(
    radius * Renderer.MASTER_SCALE * BodyMarker.BODY_SCALE,
    64,
    32
  );
}

const materialFactories: {
  [name: string]: (name: string) => THREE.Material | THREE.Material[];
} = {
  mercury: getColorMaterial,
  venus: getColorMaterial,
  earth: getColorMaterial,
  // earth: () => ([
  //   getColorMaterial('earth'),
  //   getColorMaterial('clouds', true),
  // ]),
  mars: getColorMaterial,
  jupiter: getColorMaterial,
  saturn: () => [
    getColorMaterial("saturn"),
    getColorMaterial("saturn_ring", false, THREE.DoubleSide),
  ],
  uranus: getColorMaterial,
  neptune: getColorMaterial,
};

function getColorMaterial(
  name: string,
  alphaMap = false,
  side = THREE.FrontSide
) {
  return new THREE.MeshBasicMaterial({
    map: textureLoader.load(`/textures/${name}_color.jpg`),
    alphaMap: alphaMap
      ? textureLoader.load(`/textures/${name}_alpha.jpg`)
      : null,
    transparent: alphaMap,
    side,
  });
}
