import { BaseDirectory, exists, readTextFile, writeFile, createDir } from '@tauri-apps/api/fs';

export class FileSystem {
  private static readonly _baseDirectory = { dir: BaseDirectory.AppLocalData };

  public static async writeTextFile(path: string, content: string): Promise<void> {
    // Create the directory if it doesn't exist    
    const dir = path.split('/').slice(0, -1).join('/');
    if (dir.length > 0 && !(await exists(dir, this._baseDirectory)))
      createDir(dir, this._baseDirectory);

    return writeFile(path, content, this._baseDirectory);
  }

  public static readJsonFile<T = any>(path: string): Promise<T> {
    try {
      return readTextFile(path, this._baseDirectory).then((content) => JSON.parse(content)) as Promise<T>;
    } catch (error) {
      return Promise.resolve({} as T);
    }
  }

  public static readTextFile(path: string): Promise<string> {
    try {

      return readTextFile(path, this._baseDirectory);
    } catch (error) {
      console.error(error);
      return Promise.resolve('');
    }
  }

  public static exists(path: string): Promise<boolean> {
    try {
      return exists(path, this._baseDirectory);
    } catch (error) {
      console.error(error);
      return Promise.resolve(false);
    }
  }
}