import "../style/Bin3DView.css";
import { useState } from 'react'
import { Canvas } from '@react-three/fiber'
import { Grid, PerspectiveCamera, Text } from '@react-three/drei'
import { EdgesGeometry, LineSegments, LineBasicMaterial, AxesHelper, BoxGeometry, Color, SRGBColorSpace, DoubleSide } from "three";
import { Perf } from 'r3f-perf';

import { Bin, Item, FreeSpace } from "../BinData";
import { CameraControls } from "./CameraControls";
import { DebugButton } from "../DebugButton";

interface Bin3DViewProps {
  bin: Bin;
  items: Item[];
  freeSpaces: FreeSpace[];
}

function Bin3DView({ bin, items, freeSpaces }: Bin3DViewProps) {
  const [showDebug, setShowDebug] = useState(false);
  const [showFreeSpaces, setShowFreeSpaces] = useState(false);

  // Generate a box representing the bin, inside the positive quadrant
  function renderBin() {
    const lineOffset = 0.01; // Enlarge the bin slightly so we don't get z-fighting
    const boxGeom = new BoxGeometry(bin.width + lineOffset, bin.height + lineOffset, bin.depth + lineOffset);
    const edges = new EdgesGeometry(boxGeom);
    
    return (
      <group position={[bin.width/2, bin.height/2, bin.depth/2]}>
        <primitive object={new LineSegments(edges, new LineBasicMaterial({ color: 0x616161 }))} />
      </group>
    );
  }

  // Generate boxes based on items
  // Place sequentially to the side of the bin, in order of size
  function renderPackedBoxes() {
    if (Array.isArray(items) && items.length !== 0) {
      // Calculate min and max volumes for color mapping
      const volumes = items.map(item => item.width * item.height * item.depth);
      const minVolume = Math.min(...volumes);
      const maxVolume = Math.max(...volumes);
      
      return items.map((item) => (
        <ItemBox key={item.id} item={item} minVolume={minVolume} maxVolume={maxVolume} />
      ));
    }

    return null;
  }

  // Generate semi-transparent boxes for free spaces
  function renderFreeSpaces() {
    if (showDebug && showFreeSpaces && Array.isArray(freeSpaces) && freeSpaces.length !== 0) {
      return freeSpaces.map((space, index) => (
        <FreeSpaceBox key={`free-${index}`} space={space} />
      ));
    }

    return null;
  }

  return (
    <div className="bin3DViewContainer">

      <DebugButton showPerf={showDebug} onToggle={() => setShowDebug(!showDebug)} showFreeSpaces={showFreeSpaces} onToggleFreeSpaces={() => setShowFreeSpaces(!showFreeSpaces)} />

      <Canvas id="bin3DView" flat shadows >
        <color attach="background" args={['rgb(255, 255, 255)']} />
        <PerspectiveCamera makeDefault fov={30} position={[10, 10, 10]} zoom={1}/>
        <CameraControls bin={bin} />
        <directionalLight position={[5,10,-2]} intensity={1.0}/>
        <ambientLight intensity={3.0} />

        {showDebug && <>
          <Perf position="top-left" />
          <ScaledAxes bin={bin} />
          {renderFreeSpaces()}
        </>}

        {renderBin()}
        {renderPackedBoxes()}

        <ScaledGrid bin={bin} />
      </Canvas>
    </div>
  );
}

export default Bin3DView;

// Render a box with edge outlines
function ItemBox({ item, minVolume, maxVolume }: { item: Item; minVolume: number; maxVolume: number }) {

  // Define the distance move objects 'up', so they don't clip with the floor.
  // Use to put 'things' on the floor, example: Ypos = Y scale/2 + offset
  const YOffsetFromFloor = 0.001;
  const shrinkMultiplier = 0.999; // Multiplier to shrink the box, to prevent z-fighting

  // Set colour by interpolating through hues, to visualise volume
  const volume = item.width * item.height * item.depth;
  const volumeNormalized = maxVolume === minVolume ? 0.0 : (volume - minVolume) / (maxVolume - minVolume);

  const lowestVolumeColour = new Color().setHSL(27 / 360, 0.96, 0.55, SRGBColorSpace);   // hsl(27, 96%, 55%)
  const highestVolumeColour = new Color().setHSL(215 / 360, 0.65, 0.53, SRGBColorSpace); // hsl(215, 65%, 53%)
  const boxColor = new Color().copy(lowestVolumeColour).lerpHSL(highestVolumeColour, volumeNormalized);

  const lowestVolumeWireColour = new Color().setHSL(0 / 360, 0.7, 0.3, SRGBColorSpace);     // hsl(0, 70%, 30%)
  const highestVolumeWireColour = new Color().setHSL(244 / 360, 0.7, 0.4, SRGBColorSpace);  // hsl(244, 70%, 40%)
  const wireframeColor = new Color().copy(lowestVolumeWireColour).lerpHSL(highestVolumeWireColour, volumeNormalized);
  
  return (
    <group key={item.id} position={[item.x + item.width / 2, item.y + item.height / 2 + YOffsetFromFloor, item.z + item.depth / 2]}>
      <mesh>
        {/* Box (slightly smaller to prevent z-fighting on outline) */}
        <boxGeometry args={[item.width*shrinkMultiplier, item.height*shrinkMultiplier, item.depth*shrinkMultiplier]} />
        <meshPhongMaterial color={boxColor} wireframe={false}/>
      </mesh>
      <mesh>
        {/* Wireframe outline */}
        <boxGeometry args={[item.width, item.height, item.depth]} />
        <meshStandardMaterial color={wireframeColor} wireframe={true} wireframeLinejoin="bevel"/>
      </mesh>
    </group>

  );
}

// Render a free space as a semi-transparent box
function FreeSpaceBox({ space }: { space: FreeSpace }) {
  const YOffsetFromFloor = 0.001;
  const shrinkMultiplier = 0.999; // Multiplier to shrink the box, to prevent z-fighting
  const freeSpaceColor = new Color(0x4a9eff).convertLinearToSRGB(); // Light blue color for free spaces
  
  return (
    <group position={[space.x + space.width / 2, space.y + space.height / 2 + YOffsetFromFloor, space.z + space.depth / 2]}>
      <mesh>
        <boxGeometry args={[space.width*shrinkMultiplier, space.height*shrinkMultiplier, space.depth*shrinkMultiplier]} />
        <meshPhongMaterial color={freeSpaceColor} wireframe={false} transparent opacity={0.15} />
      </mesh>
      <mesh>
        {/* Wireframe outline */}
        <boxGeometry args={[space.width*shrinkMultiplier, space.height*shrinkMultiplier, space.depth*shrinkMultiplier]} />
        <meshStandardMaterial color={freeSpaceColor} wireframe={true} wireframeLinejoin="bevel" transparent opacity={0.4} />
      </mesh>
    </group>
  );
}

// Grid scaled to bin size
function ScaledGrid({ bin }: { bin: Bin }) {
  // Scale grid based on bin dimensions (orders of 10)
  const maxDim = Math.max(bin.width, bin.height, bin.depth);
  const scale = Math.pow(10, Math.floor(Math.log10(maxDim)));

  return (
    <>
    <Grid 
      renderOrder={1} 
      position={[0, 0, 0]} 
      infiniteGrid 
      cellSize={1 * scale}
      cellColor={"rgb(68, 68, 68)"}
      cellThickness={0.6 } 
      sectionSize={5 * scale} 
      sectionThickness={1.5 } 
      sectionColor={"rgb(253, 169, 12)"} 
      fadeFrom={0}
      fadeDistance={60*((bin.height+bin.width+bin.depth) / 10)}
      fadeStrength={8}
      side={DoubleSide}
    />
    </>

  );
}

function ScaledAxes({ bin }: { bin: Bin }) {
  // Scale based on bin dimensions (orders of 10)
  const maxDim = Math.max(bin.width, bin.height, bin.depth);
  const scale = Math.pow(10, Math.floor(Math.log10(maxDim)));
  return (
    <>
      <primitive object={new AxesHelper(60*scale)} />
      <Text position={[40*scale, 8*scale, 0]} rotation={[0, Math.PI/2, 0]} fontSize={20*scale} color="red" anchorX="center" anchorY="middle">X</Text>
      <Text position={[0, 40*scale, 0]} rotation={[Math.PI/2, 0, 0]} fontSize={20*scale} color="green" anchorX="center" anchorY="middle">Y</Text>
      <Text position={[0, 8*scale, 40*scale]} rotation={[0, Math.PI, 0]} fontSize={20*scale} color="blue" anchorX="center" anchorY="middle">Z</Text>
    </>
  );
}