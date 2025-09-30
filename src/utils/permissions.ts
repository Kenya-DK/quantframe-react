export enum PermissionsFlags {
  ALL = "all",
}

/**
 * @param permissions The permissions bitwise number
 * @param permission  The permission to check
 * @returns True if the permission is set
 */
export const HasPermission = (permissions: Array<string> | undefined | string, permission: PermissionsFlags | string | undefined) => {
  if (!permission) return true;
  if (!permissions) return false;
  if (typeof permissions === "string") permissions = permissions.split(",");
  return permissions.includes(permission) || permissions.includes(PermissionsFlags.ALL);
};
