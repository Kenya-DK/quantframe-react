import { ResponseError } from "../types";

export class AppError {
  constructor(public error: ResponseError<{ operations: string[] }>) {}

  hasOperation(operation: string) {
    return this.error.properties?.operations?.includes(operation) ?? false;
  }
}
