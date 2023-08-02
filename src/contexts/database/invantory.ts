import { InventoryEntryDto, Wfm } from '$types/index';
import { db } from './database.context'
import { CreateSqlInsert, CreateSqlUpdate } from './helper';

export default class Inventory {
  constructor() {
    // Initialize database
    db.execute(/*sql*/`
        CREATE TABLE if not exists ${Inventory.name} (
          id integer not null primary key autoincrement,
          item_id text not null,
          item_url text not null,
          item_name text not null,
          rank integer not null default 0,
          price REAL not null default 0,
          listed_price INT default null,
          owned INT not null default 1
        ) STRICT;
      `);
  }

  async list(): Promise<InventoryEntryDto[]> {
    return await db.select<InventoryEntryDto[]>(`SELECT * FROM ${Inventory.name}`);
  }

  async create(item: Wfm.ItemDto, _quantity: number, price: number, rank: number) {

    // Create new entry
    const entry: InventoryEntryDto = {
      item_id: item.id,
      item_url: item.url_name,
      item_name: item.item_name,
      rank,
      price,
      owned: 1,
    }
    // Check if item already exists
    const exists: InventoryEntryDto[] = await db.select<InventoryEntryDto[]>(`
      SELECT * FROM ${Inventory.name} WHERE item_id = $1`
      , [item.id])

    if (exists.length > 0) {
      const total_owned = exists.reduce((acc, cur) => acc + cur.owned, 0) + 1;
      const total_price = exists.reduce((acc, cur) => acc + cur.owned * cur.price, 0) + price;
      const weighted_average = total_price / total_owned;
      const foundItem = exists[0];
      foundItem.owned = total_owned;
      foundItem.price = weighted_average;
      if (!foundItem.id) throw new Error('Item id is undefined');
      return foundItem;
    } else {
      const { sql, values } = CreateSqlInsert(Inventory.name, entry);
      const re = await db.execute(sql, values);
      entry.id = re.lastInsertId;
    }
    return entry;
  }
  async updateById(id: number, input: Partial<InventoryEntryDto>): Promise<InventoryEntryDto> {
    const { sql, values } = CreateSqlUpdate(Inventory.name, input, { id });
    await db.execute(sql, values);
    // Get updated entry
    const [entry] = await db.select<InventoryEntryDto[]>(`SELECT * FROM ${Inventory.name} WHERE id = $1`, [id]);
    if (!entry) throw new Error('Entry not found');
    return entry;

  }
  async delete(id: number) {
    return await db.execute(`DELETE FROM ${Inventory.name} WHERE id = $1`, [id]);
  }

}