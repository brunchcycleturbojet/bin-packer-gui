import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table";
import { Item } from "../bin-data";
import CatIcon from "../assets/cat-icon"
import { ScrollArea } from "./ui/scroll-area";

interface PackerResultProps {
  items: Item[];
  emptyMessage: string;
}

// Table display of a packing result instance
export default function PackerResult(props: PackerResultProps) {
  const {items, emptyMessage} = props;
  return (
    <div className="relative h-full flex">
      {items && items.length <= 0 ? (
      <div className="absolute h-full w-full flex flex-col justify-center items-center" >
        <CatIcon className="h-1/4 w-1/4 fill-border" />
        <div className="text-border font-medium text-lg ">{emptyMessage}</div>
      </div>
      ): <></>}
      <ScrollArea className="grow min-h-0">
        <Table>
          <TableHeader className="sticky top-0 bg-secondary">
            <TableRow>
              <TableHead className="text-left pl-10">Name</TableHead>
              <TableHead className="text-center">Size (W × H × D)</TableHead>
              <TableHead className="text-center">Qty</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {items.map((item) => (
                <TableRow key={item.shape_id}>
                  <TableCell className="pl-10">{item.name}</TableCell>
                  <TableCell className="text-center">{item.width} × {item.height} × {item.depth}</TableCell>
                  <TableCell className="text-center">{item.quantity}</TableCell>
                </TableRow>
              ))}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  );
}
