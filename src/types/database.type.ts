export interface InventoryEntryDto {
  id?: number;
  item_id: string;
  item_url: string;
  item_name: string;
  mod_rank: number;
  price: number;
  listed_price?: number | null;
  owned: number;
}