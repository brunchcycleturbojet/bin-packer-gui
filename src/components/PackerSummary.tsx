import { useAppStore } from "../store";

export default function PackerSummary() {
  const { bin } = useAppStore();

  return (
    <div className="">
      <h4>
        <strong>Bin:</strong> {bin.width} × {bin.height} × {bin.depth}
      </h4>
      <div>
        (TIME OF PACKING START)
      </div>
      <div>
        (TIME OF PACKING END)
      </div>
      <div>
        (TIME TO PACK)
      </div>
      <div>
        (FILL PERCENTAGE)
      </div>
      <div>
        (ITEMS PACKED)
      </div>
      <div>
        (ITEMS NOT PACKED)
      </div>
    </div>
  );
}
