export enum PermissionsFlags {
  ALL = "all",
  RIVEN_PRICES_SEARCH = "riven_prices_search",
  WFM_USER_ACTIVE_HISTORY = "wfm_user_active_history",
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
