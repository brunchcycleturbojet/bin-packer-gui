import CatIcon from "../assets/cat-icon";
import { useAppStore } from "../store";

export default function PackerSummary() {
  const { bin, lastPackedItems, lastUnpackedItems } = useAppStore();

  if(lastPackedItems.length === 0 && lastUnpackedItems.length === 0) {
    return (
      <div className="h-full w-full flex flex-col justify-center items-center" >
        <CatIcon className="h-1/4 w-1/4 fill-border" />
        <div className="text-border font-medium text-lg ">No items</div>
      </div>
    );
  } else {
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

}
