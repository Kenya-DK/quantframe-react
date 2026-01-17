export type SortingDirection = 'asc' | 'desc' | null | undefined;
export interface SortingField {
  field: string;
  direction: SortingDirection;
}