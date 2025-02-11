export interface PaginatedWithInclude<T, I> {
  total: number;
  limit: number;
  page: number;
  results: T[];
  include: I;
}
