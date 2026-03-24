import { OrbitControls } from "@react-three/drei";
import { Vector3, MOUSE } from "three";
import { Bin } from "../BinData";
import { useEffect, useRef, useState } from "react";
import { useThree } from "@react-three/fiber";

interface CameraControlsProps {
  bin: Bin;
}

export function CameraControls({ bin }: CameraControlsProps) {
  const [controlsOrbitPoint, setControlsOrbitPoint] = useState<[number, number, number]>([0, 0, 0]);
  const controlsRef = useRef<any>(null);
  const { camera } = useThree();

  // Update camera when bin changes, to keep it in view
  useEffect(() => {
    setControlsOrbitPoint([bin.width / 2, bin.height / 2, bin.depth / 2]);

    const maxDimension = Math.max(bin.width, bin.height, bin.depth);
    const distance = maxDimension * 2.0;
    camera.position.set(distance, distance, distance);

  }, [bin.width, bin.height, bin.depth]);

  // Restrict panning to vertical only
  const handleControlsChange = () => {
    if (controlsRef.current) {
      const target = controlsRef.current.target;

      const maxPan = controlsOrbitPoint[1] + bin.height * 0.8;
      const minPan = controlsOrbitPoint[1] - bin.height * 0.8;
      const clampedY = Math.min(Math.max(target.y, minPan), maxPan);

      // Keep only the Y component, reset X and Z to center
      target.set(controlsOrbitPoint[0], clampedY, controlsOrbitPoint[2]);
    }
  };

    return (
    <>
        <OrbitControls
        ref={controlsRef}
        makeDefault 
        target={new Vector3(controlsOrbitPoint[0], controlsOrbitPoint[1], controlsOrbitPoint[2])}
        mouseButtons={{
            LEFT: MOUSE.ROTATE, 
            MIDDLE: MOUSE.DOLLY,
            RIGHT: MOUSE.PAN,
        }}
        enableDamping={true}
        dampingFactor={0.05}
        onChange={handleControlsChange}
        />
    </>
    );
}