import { Button } from "#components/ui/button";
import { RiBugLine } from "@remixicon/react";

interface DebugButtonProps {
  showPerf: boolean;
  onToggle: () => void;
  showFreeSpaces?: boolean;
  onToggleFreeSpaces?: () => void;
}

export function DebugButton({ showPerf, onToggle, showFreeSpaces = false, onToggleFreeSpaces }: DebugButtonProps) {
  return (
    <>
      <Button
        variant="secondary"
        onClick={onToggle}
        className="absolute z-100 right-1.5 top-1.5">
        <RiBugLine />
      </Button>
      
      {showPerf && onToggleFreeSpaces && (
        <button
          onClick={onToggleFreeSpaces}
          className={`debugButton ${showFreeSpaces ? 'debugButton--active' : 'debugButton--inactive'}`}
          style={{ right: '110px' }}
        >
          Toggle Free Spaces
        </button>
      )}
    </>
  );
}
