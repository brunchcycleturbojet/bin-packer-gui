import { useAppStore } from "./store";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./components/ui/table";

// Component to display an instance of packing results
export default function PackerResult() {
  const { bin, lastPackedItems, lastUnpackedItems } = useAppStore();

  const renderItemsTable = (items: typeof lastPackedItems, title: string) => {
    return (
      <div className="">
        <h4 className="text-center">{title}</h4>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead className="text-center">ID</TableHead>
              <TableHead className="text-center">Name</TableHead>
              <TableHead className="text-center">Size (W × H × D)</TableHead>
              <TableHead className="text-center">Qty</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {items && items.length > 0 ? (
              items.map((item) => (
                <TableRow key={item.shape_id}>
                  <TableCell className="text-center">{item.shape_id}</TableCell>
                  <TableCell >{item.name}</TableCell>
                  <TableCell className="text-center">{item.width} × {item.height} × {item.depth}</TableCell>
                  <TableCell className="text-center">{item.quantity}</TableCell>
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={4} className="text-center">
                  No items
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
    );
  };

  return (
    <div className="">
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

      <div className="mt-5">
        {renderItemsTable(lastPackedItems, "Packed")}
        {renderItemsTable(lastUnpackedItems, "Unpacked")}
      </div>

    </div>
  );
}
