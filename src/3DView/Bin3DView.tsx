import "../style/Bin3DView.css";
import { useState, useRef, useLayoutEffect, useEffect } from 'react'
import { Canvas, useFrame, useThree } from '@react-three/fiber'
import { Grid, PerspectiveCamera } from '@react-three/drei'
import { DoubleSide, EdgesGeometry, LineSegments, LineBasicMaterial, Mesh } from "three";

import { Bin, Item } from "../BinData";
import { CameraControls } from "./CameraControls";

interface Bin3DViewProps {
  bin: Bin;
  items: Item[];
}

function Bin3DView({ bin, items }: Bin3DViewProps) {
  const [gridOrigin, setGridOrigin] = useState<[number, number, number]>([0, 0, 0]);

  // Center grid to bin position whenever it changes
  useEffect(() => {
    setGridOrigin([bin.width / 2, 0, bin.depth / 2])
  }, [bin.width, bin.height, bin.depth]);

  // Generate a box representing the bin, inside the positive quadrant
  function renderBin() {
    const lineOffset = 0.01; // Enlarge the bin slightly so we don't get z-fighting
    return (
      <mesh position={[bin.width/2, bin.height/2, bin.depth/2]}>
        <boxGeometry args={[bin.width + lineOffset, bin.height + lineOffset, bin.depth + lineOffset]} />
        <meshBasicMaterial color="rgb(97, 97, 97)" wireframe={true} transparent opacity={0.25} side={DoubleSide} />
      </mesh>
    );
  }

  // Generate boxes based on items
  // Place sequentially to the side of the bin, in order of size
  function renderUnpackedBoxes() {
    if (Array.isArray(items) && items.length !== 0) {
      return items.map((item) => (
        <ItemBox key={item.id} item={item} />
      ));
    }

    return null;
  }

  return (
    <Canvas id="bin3DView" flat shadows >
      <color attach="background" args={['rgb(235, 232, 232)']} />

      <PerspectiveCamera makeDefault fov={30} position={[10, 10, 10]} zoom={1}/>
      <CameraControls bin={bin} />

      <directionalLight position={[5,10,-2]} intensity={1.0}/>
      <ambientLight intensity={3.0} />

      {renderBin()}
      {renderUnpackedBoxes()}

      <Grid 
        renderOrder={-1} 
        position={gridOrigin} 
        infiniteGrid 
        cellSize={0.6} 
        cellColor={"rgb(68, 68, 68)"}
        cellThickness={0.6} 
        sectionSize={3.3} 
        sectionThickness={1.5} 
        sectionColor={"rgb(253, 169, 12)"} 
        fadeFrom={0} // 0: fade from origin, 1: fade from camera
        fadeDistance={60}
        fadeStrength={8}
        side={DoubleSide}
      />

      {/* Fake reflection (hidden when camera is below floor) */}
      <FloorMask>
        <group scale={[1, -1, 1]} position={[0, -0.02, 0]}>
          {renderBin()}
          {renderUnpackedBoxes()}
        </group>
      </FloorMask>
    </Canvas>
  );
}

export default Bin3DView;

// Component to render a box with edge outlines
function ItemBox({ item }: { item: Item }) {
  const meshRef = useRef<Mesh>(null);

  useLayoutEffect(() => {
    if (meshRef.current) {
      const geometry = meshRef.current.geometry;
      const edges = new EdgesGeometry(geometry);
      const wireframe = new LineSegments(edges, new LineBasicMaterial({ color: "rgb(119, 61, 61)", linewidth: 2 }));
      meshRef.current.add(wireframe);
    }
  }, []);

  // Define the distance move objects 'up', so they don't clip with the floor.
  // Use to put 'things' on the floor, example: Ypos = Y scale/2 + offset
  const YOffsetFromFloor = 0.001;
  return (
    <mesh 
      ref={meshRef}
      key={item.id} 
      position={[item.x + item.width / 2, item.y + item.height / 2 + YOffsetFromFloor, item.z + item.depth / 2]}
    >
      <boxGeometry args={[item.width, item.height, item.depth]} />
      <meshStandardMaterial color="rgb(231, 136, 58)" />
    </mesh>
  );
}

// Component to control rendering of children, only when above the floor (xz plane)
function FloorMask({ children }: { children: React.ReactNode }) {
  const { camera } = useThree();
  const [visible, setVisible] = useState(true);

  useFrame(() => {
    const shouldShow = camera.position.y >= 0;
    if (visible !== shouldShow) {
      setVisible(shouldShow);
    }
  });

  if (!visible) return null;
  return (
    <>{children}</>
  );
}