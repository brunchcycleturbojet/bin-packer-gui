import "./style/PackerResults.css";
import { useAppStore } from "./store";

// Component to display an instance of packing results
export default function PackerResult() {
  const { bin, lastPackedItems, lastUnpackedItems } = useAppStore();

  const renderItemsTable = (items: typeof lastPackedItems, title: string) => {
    return (
      <div className="items-section">
        <h4 className="section-title">{title}</h4>
        <table>
          <tbody>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Size (W × H × D)</th>
              <th>Qty</th>
            </tr>
            {items && items.length > 0 ? (
              items.map((item) => (
                <tr key={item.shape_id}>
                  <td>{item.shape_id}</td>
                  <td>{item.name}</td>
                  <td>{item.width} × {item.height} × {item.depth}</td>
                  <td>{item.quantity}</td>
                </tr>
              ))
            ) : (
              <tr>
                <td colSpan={4} className="centered-cell">
                  No items
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    );
  };

  return (
    <div className="table-container">
      <div className="bin-info">
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

      <div className="item-results">
        {renderItemsTable(lastPackedItems, "Packed")}
        {renderItemsTable(lastUnpackedItems, "Unpacked")}
      </div>

    </div>
  );
}
