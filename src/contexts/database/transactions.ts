import { TransactionEntryDto, Wfm } from '$types/index';
import { db } from './database.context'
import { CreateSqlInsert, CreateSqlUpdate } from './helper';

export default class Transaction {
  constructor() {
    // Initialize database
    db.execute(/*sql*/`
        CREATE TABLE if not exists ${Transaction.name} (
          id integer not null primary key autoincrement,
          item_id text not null,
          item_type text not null,
          item_url text not null,
          item_name text not null,
          datetime TEXT,
          transactionType TEXT,
          rank integer not null default 0,
          price REAL not null default 0
        ) STRICT;
      `);
  }
  async list(): Promise<TransactionEntryDto[]> {
    return await db.select<TransactionEntryDto[]>(`SELECT * FROM ${Transaction.name}`);
  }
  async create(item: Wfm.ItemDto, type: "buy" | "sold", item_type: string, _quantity: number, price: number, rank: number) {

    // Create new entry
    const entry: TransactionEntryDto = {
      item_id: item.id,
      item_type: item_type,
      item_url: item.url_name,
      item_name: item.item_name,
      rank,
      price,
      datetime: new Date().toISOString(),
      transactionType: type
    }
    const { sql, values } = CreateSqlInsert(Transaction.name, entry);
    const re = await db.execute(sql, values);
    entry.id = re.lastInsertId;
    return entry;
  }
  async updateById(id: number, input: Partial<TransactionEntryDto>): Promise<TransactionEntryDto> {
    const { sql, values } = CreateSqlUpdate(Transaction.name, input, { id });
    await db.execute(sql, values);
    // Get updated entry
    const [entry] = await db.select<TransactionEntryDto[]>(`SELECT * FROM ${Transaction.name} WHERE id = $1`, [id]);
    if (!entry) throw new Error('Entry not found');
    return entry;

  }
  async delete(id: number) {
    return await db.execute(`DELETE FROM ${Transaction.name} WHERE id = $1`, [id]);
  }
}