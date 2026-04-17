export class Bin {
  width = 0;
  height = 0;
  depth = 0;

  constructor(w: number, h: number, d: number) {
    this.width = w;
    this.height = h;
    this.depth = d;
  }
}

export type Item = {
  shape_id: number,     // Unique ID
  name: string,   // Aa-Zz, 0-9 freely customisable name
  x: number,
  y: number,
  z: number,
  width: number,
  height: number,
  depth: number,
  quantity: number,  // Number of items of this size to pack (default: 1)
}

export type FreeSpace = {
  x: number,
  y: number,
  z: number,
  width: number,
  height: number,
  depth: number,
}

// Packing Request Schema
export type PackerInput = {
  bin: {
    width: number,
    height: number,
    depth: number,
  },
  items: Item[],
  unpacked_items?: Item[],
}

// Packing Response Schema
export type PackerOutput = {
  bin: {
    width: number,
    height: number,
    depth: number,
  },
  items: Item[],
  free_spaces: FreeSpace[],
  unpacked_items?: Item[],
}

export type LoadOutput = {
  pack_input: PackerInput,
  pack_result: PackerOutput,
}

/**
 * PackerSchema - Defines the JSON message structure for bin packing operations
 * Handles validation and creation of packing requests and responses
 */
export class PackerSchema {
  /**
   * Creates a packing input payload from bin dimensions and items
   */
  static createInput(bin: { width: number, height: number, depth: number }, items: Item[]): PackerInput {
    return {
      bin,
      items,
      unpacked_items: [],
    };
  }

  /**
   * Validates a packing input payload
   */
  static validateInput(data: unknown): data is PackerInput {
    if (typeof data !== 'object' || data === null) return false;
    
    const obj = data as any;
    
    // Check bin structure
    if (!obj.bin || typeof obj.bin !== 'object') return false;
    if (typeof obj.bin.width !== 'number' || typeof obj.bin.height !== 'number' || typeof obj.bin.depth !== 'number') return false;
    
    // Check items array
    if (!Array.isArray(obj.items)) return false;
    if (!obj.items.every((item: any) => PackerSchema.isValidItem(item))) return false;
    
    return true;
  }

  /**
   * Validates a packing output payload
   */
  static validateOutput(data: unknown): data is PackerOutput {
    if (typeof data !== 'object' || data === null) return false;
    
    const obj = data as any;
    
    // Check bin structure
    if (!obj.bin || typeof obj.bin !== 'object') return false;
    if (typeof obj.bin.width !== 'number' || typeof obj.bin.height !== 'number' || typeof obj.bin.depth !== 'number') return false;
    
    // Check items array
    if (!Array.isArray(obj.items)) return false;
    if (!obj.items.every((item: any) => PackerSchema.isValidItem(item))) return false;
    
    // Check free_spaces array
    if (!Array.isArray(obj.free_spaces)) return false;
    if (!obj.free_spaces.every((space: any) => PackerSchema.isValidFreeSpace(space))) return false;
    
    return true;
  }

  /**
   * Validates an individual item
   */
  private static isValidItem(item: any): item is Item {
    return (
      typeof item === 'object' &&
      item !== null &&
      typeof item.shape_id === 'number' &&
      typeof item.name === 'string' &&
      typeof item.x === 'number' &&
      typeof item.y === 'number' &&
      typeof item.z === 'number' &&
      typeof item.width === 'number' &&
      typeof item.height === 'number' &&
      typeof item.depth === 'number' &&
      typeof item.quantity === 'number' &&
      item.quantity > 0
    );
  }

  /**
   * Validates a free space
   */
  private static isValidFreeSpace(space: any): space is FreeSpace {
    return (
      typeof space === 'object' &&
      space !== null &&
      typeof space.x === 'number' &&
      typeof space.y === 'number' &&
      typeof space.z === 'number' &&
      typeof space.width === 'number' &&
      typeof space.height === 'number' &&
      typeof space.depth === 'number'
    );
  }
}