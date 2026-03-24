import "./style/App.css";
import { Bin, Item } from "./BinData";

interface PackerTableProps {
  bin: Bin;
  items: Item[];
}

function PackerTable({ bin, items }: PackerTableProps) {

  // Show current bin dimensions
  const binDescription = `${bin.id} (${bin.width}×${bin.height}×${bin.depth})`;

  // Load a bin from file

  // Save a bin to file

  // Add new item

  // Delete item

  // Update item(s)

  return (
    <>
      <div className="bin-info">{binDescription}</div>
      <table>
        <tbody>
          <tr>
            <th></th>
            <th>Name</th>
            <th>Colour</th>
            <th>Height</th>
            <th>Width</th>
            <th>Depth</th>
          </tr>
          {items && items.length > 0 ? (
            items.map((item) => (
              <tr key={item.id}>
                <td><input type="checkbox" name={`item_${item.id}`} value={`${item.id}`} /></td>
                <td>{item.name}</td>
                <td>TODO</td>
                <td>{item.height}</td>
                <td>{item.width}</td>
                <td>{item.depth}</td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={6}>No items loaded</td>
            </tr>
          )}
        </tbody>

      </table>
    </>
  );
}

export default PackerTable;