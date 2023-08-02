export const CreateSqlInsert = (table: string, data: { [key: string]: any }) => {
  const keys = Object.keys(data);
  const values = Object.values(data);
  const sql = `INSERT INTO ${table} (${keys.join(', ')}) VALUES (${values.map((_, i) => `$${i + 1}`).join(', ')})`;
  return { sql, values };
}

export const CreateSqlUpdate = (table: string, data: { [key: string]: any }, where: { [key: string]: any }) => {
  const keys = Object.keys(data);
  const values = Object.values(data);
  const whereKeys = Object.keys(where);
  const whereValues = Object.values(where);
  const sql = `UPDATE ${table} SET ${keys.map((key, i) => `${key} = $${i + 1}`).join(', ')} WHERE ${whereKeys.map((key, i) => `${key} = $${i + 1 + keys.length}`).join(', ')}`;
  return { sql, values: [...values, ...whereValues] };
}