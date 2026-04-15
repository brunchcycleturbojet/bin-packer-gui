interface DebugButtonProps {
  showPerf: boolean;
  onToggle: () => void;
  showFreeSpaces?: boolean;
  onToggleFreeSpaces?: () => void;
}

export function DebugButton({ showPerf, onToggle, showFreeSpaces = false, onToggleFreeSpaces }: DebugButtonProps) {
  const buttonClass = `debugButton ${showPerf ? 'debugButton--active' : 'debugButton--inactive'}`;

  return (
    <>
      <button
        onClick={onToggle}
        className={buttonClass}
      >
        Debug
      </button>
      
      {showPerf && onToggleFreeSpaces && (
        <button
          onClick={onToggleFreeSpaces}
          className={`debugButton ${showFreeSpaces ? 'debugButton--active' : 'debugButton--inactive'}`}
          style={{ right: '110px' }}
        >
          Free Spaces
        </button>
      )}
    </>
  );
}
