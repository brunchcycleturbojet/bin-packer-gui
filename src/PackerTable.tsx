import "./style/App.css";
import { Bin, Item } from "./BinData";

interface PackerTableProps {
  bin: Bin;
  items: Item[];
}

function PackerTable({ bin: _bin, items }: PackerTableProps) {

  // Load a bin from file

  // Save a bin to file

  // Add new item

  // Delete item

  // Update item(s)

  return (
    <div className="table-container">
      <table>
        <tbody>
          <tr>
            <th></th>
            <th>Name</th>
            <th>Colour</th>
            <th>Size</th>
          </tr>
          {items && items.length > 0 ? (
            items.map((item) => (
              <tr key={item.id}>
                <td><input type="checkbox" name={`item_${item.id}`} value={`${item.id}`} /></td>
                <td>{item.name}</td>
                <td>TODO</td>
                <td>{item.width}x{item.height}x{item.depth}</td>
              </tr>
            ))
          ) : (
            <tr>
              <td colSpan={6}>No items loaded</td>
            </tr>
          )}
        </tbody>

      </table>
    </div>
  );
}

export default PackerTable;