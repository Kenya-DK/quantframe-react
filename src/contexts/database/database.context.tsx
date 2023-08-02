import { Box } from "@mantine/core";
import Database from 'tauri-plugin-sql-api'
import { createContext, useContext, useEffect, useState } from "react";
import { InventoryEntryDto, TransactionEntryDto, SQL_LITE_DB_PATH } from '$types/index'
import api from "../../api";
import Inventory from './invantory';
import Transaction from './transactions';
export const db = await Database.load(SQL_LITE_DB_PATH)
const tableInvantory = new Inventory();
const tableTransaction = new Transaction();

type DatabaseContextProps = {
  invantory: InventoryEntryDto[];
  createInvantoryEntry: (id: string, quantity: number, price: number, mod_rank: number) => Promise<void>;
  deleteInvantoryEntryById: (id: number) => Promise<void>;
  updateInvantoryById: (id: number, input: Partial<InventoryEntryDto>) => Promise<void>;
  transactions: TransactionEntryDto[];
}
type DatabaseContextProviderProps = {
  children: React.ReactNode;
}

export const DatabaseContext = createContext<DatabaseContextProps>({
  invantory: [],
  transactions: [],
  createInvantoryEntry: async () => { },
  deleteInvantoryEntryById: async () => { },
  updateInvantoryById: async () => { }
});

export const useDatabaseContext = () => useContext(DatabaseContext);

export const DatabaseContextProvider = ({ children }: DatabaseContextProviderProps) => {
  // Invantory Functions
  const [invantory, setInvantory] = useState<InventoryEntryDto[]>([]);
  const getInvantoryById = async (id: number): Promise<[index: number, item: InventoryEntryDto | undefined, items: InventoryEntryDto[]]> => {
    const newInvantory = [...invantory];
    const index = newInvantory.findIndex((item) => item.id === id);
    if (index !== -1)
      return [index, newInvantory[index], newInvantory];
    return [index, undefined, newInvantory];
  };
  const createInvantoryEntry = async (id: string, quantity: number, price: number, rank: number) => {
    if (price <= 0) throw new Error("Price must be greater than 0");

    const item = await api.items.findByUrlName(id);
    if (!item) throw new Error(`Item with id ${id} not found`);

    // Update database and get new entry
    const entry = await tableInvantory.create(item, quantity, price, rank);

    // Check if item already exists
    if (entry.id) updateInvantoryById(entry.id, entry);
    else setInvantory([...invantory, entry]);
  };
  const deleteInvantoryEntryById = async (id: number) => {
    const newInvantory = [...invantory];
    const index = newInvantory.findIndex((item) => item.id === id);
    if (index !== -1) {
      newInvantory.splice(index, 1);
      setInvantory(newInvantory);
      await tableInvantory.delete(id);
    }
  };
  const updateInvantoryById = async (id: number, input: Partial<InventoryEntryDto>) => {
    const [index, item, newInvantory] = await getInvantoryById(id);
    if (!item) throw new Error(`Item with id ${id} not found`);
    if (input.price && input.price <= 0) throw new Error("Price must be greater than 0");
    // Update database and get new entry
    const entry = await tableInvantory.updateById(id, input);
    newInvantory[index] = { ...newInvantory[index], ...entry };
    setInvantory(newInvantory);
  };
  // Transaction Functions
  const [transactions, setTransactions] = useState<TransactionEntryDto[]>([]);

  useEffect(() => {
    (async () => {
      // Load invantory
      setInvantory(await tableInvantory.list());
      // Load transactions
      setTransactions(await tableTransaction.list());
    })();
  }, [])

  return (
    <DatabaseContext.Provider value={{ invantory, updateInvantoryById, createInvantoryEntry, deleteInvantoryEntryById, transactions }}>
      <Box>
        {children}
      </Box>
    </DatabaseContext.Provider>
  )
}