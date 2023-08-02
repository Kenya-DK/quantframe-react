import { Box } from "@mantine/core";
import Database from 'tauri-plugin-sql-api'
import { createContext, useContext, useEffect, useState } from "react";
import { InventoryEntryDto, SQL_LITE_DB_PATH } from '$types/index'
import api from "../../api";

export const db = await Database.load(SQL_LITE_DB_PATH)

const CreateSqlInsert = (table: string, data: { [key: string]: any }) => {
  const keys = Object.keys(data);
  const values = Object.values(data);
  const sql = `INSERT INTO ${table} (${keys.join(', ')}) VALUES (${values.map((_, i) => `$${i + 1}`).join(', ')})`;
  return { sql, values };
}

const CreateSqlUpdate = (table: string, data: { [key: string]: any }, where: { [key: string]: any }) => {
  const keys = Object.keys(data);
  const values = Object.values(data);
  const whereKeys = Object.keys(where);
  const whereValues = Object.values(where);
  const sql = `UPDATE ${table} SET ${keys.map((key, i) => `${key} = $${i + 1}`).join(', ')} WHERE ${whereKeys.map((key, i) => `${key} = $${i + 1 + keys.length}`).join(', ')}`;
  return { sql, values: [...values, ...whereValues] };
}


type DatabaseContextProps = {
  invantory: InventoryEntryDto[];
  createInvantoryEntry: (id: string, quantity: number, price: number, mod_rank: number) => Promise<void>;
  deleteInvantoryEntryById: (id: number) => Promise<void>;
  updateInvantoryById: (id: number, input: Partial<InventoryEntryDto>) => Promise<void>;
}
type DatabaseContextProviderProps = {
  children: React.ReactNode;
}

export const DatabaseContext = createContext<DatabaseContextProps>({
  invantory: [],
  createInvantoryEntry: async () => { },
  deleteInvantoryEntryById: async () => { },
  updateInvantoryById: async () => { }
});

export const useDatabaseContext = () => useContext(DatabaseContext);

export const DatabaseContextProvider = ({ children }: DatabaseContextProviderProps) => {
  // Invantory Functions
  const [invantory, setInvantory] = useState<InventoryEntryDto[]>([]);
  const createInvantoryEntry = async (id: string, _quantity: number, price: number, mod_rank: number) => {
    if (price <= 0) throw new Error("Price must be greater than 0");

    const item = await api.items.findByUrlName(id);
    if (!item) throw new Error(`Item with id ${id} not found`);

    // Check if item already exists
    const exists = await db.select<InventoryEntryDto[]>(`
      SELECT * FROM inventory WHERE item_id = $1`
      , [item.id])
    if (exists.length > 0) {
      const total_owned = exists.reduce((acc, cur) => acc + cur.owned, 0) + 1;
      const total_price = exists.reduce((acc, cur) => acc + cur.owned * cur.price, 0) + price;
      const weighted_average = total_price / total_owned;
      const foundItem = exists[0];
      foundItem.owned = total_owned;
      foundItem.price = weighted_average;
      if (foundItem.id)
        updateInvantoryById(foundItem.id, foundItem);
    } else {
      const asd = CreateSqlInsert('inventory', {
        item_id: item.id,
        item_url: item.url_name,
        item_name: item.item_name,
        mod_rank: mod_rank,
        price: price,
        owned: 1,
      });
      const re = await db.execute(asd.sql, asd.values);
      setInvantory([...invantory, {
        id: re.lastInsertId,
        item_id: item.id,
        item_url: item.url_name,
        item_name: item.item_name,
        mod_rank: mod_rank,
        price: price,
        owned: 1,
      }]);
    }
  };
  const deleteInvantoryEntryById = async (id: number) => {
    const newInvantory = [...invantory];
    const index = newInvantory.findIndex((item) => item.id === id);
    if (index !== -1) {
      newInvantory.splice(index, 1);
      setInvantory(newInvantory);
      await db.execute(`DELETE FROM inventory WHERE id = $1`, [id]);
    }
  };
  const updateInvantoryById = async (id: number, input: Partial<InventoryEntryDto>) => {
    const newInvantory = [...invantory];
    const index = newInvantory.findIndex((item) => item.id === id);
    if (index !== -1) {
      newInvantory[index] = { ...newInvantory[index], ...input };
      setInvantory(newInvantory);
      const { sql, values } = CreateSqlUpdate('inventory', input, { id });
      await db.execute(sql, values);
    }
  };


  useEffect(() => {
    (async () => {
      console.log("Initialize database");

      // Initialize database
      await db.execute(/*sql*/`
      CREATE TABLE if not exists inventory (
        id integer not null primary key autoincrement,
        item_id text not null,
        item_url text not null,
        item_name text not null,
        mod_rank integer not null default 0,
        price REAL not null default 0,
        listed_price INT default null,
        owned INT not null default 1
      ) STRICT;
    `);
      // Load invantory
      const invantory = await db.select<InventoryEntryDto[]>(`SELECT * FROM inventory`);
      setInvantory(invantory);

    })();
  }, [])

  return (
    <DatabaseContext.Provider value={{ invantory, updateInvantoryById, createInvantoryEntry, deleteInvantoryEntryById }}>
      <Box>
        {children}
      </Box>
    </DatabaseContext.Provider>
  )
}