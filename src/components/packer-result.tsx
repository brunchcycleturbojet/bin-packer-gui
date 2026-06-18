import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table";
import { Item } from "../bin-data";

interface PackerResultProps {
  items: Item[];
}

// Table display of a packing result instance
export default function PackerResult(props: PackerResultProps) {
  const items = props.items;

  return (
    <div className="mt-5">
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
}
