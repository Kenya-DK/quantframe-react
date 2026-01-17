export interface Query {
  [key: string]: any;
  $or?: Query[]; // Support for $or
  $match?: Query; // Support for $match
}
// Helper function to get nested property values using dot notation
const getNestedValue = (obj: any, path: string): any => {
  return path
    .split(".")
    .reduce(
      (value, key) =>
        value && value[key] !== undefined ? value[key] : undefined,
      obj
    );
};

// Updated search function with combined property support
export const searchProperties = <T>(
  properties: T[],
  query: Query,
  isCaseSensitive: boolean = true
): T[] => {
  const compareValues = (
    propertyValue: any,
    comparisonValue: any,
    operator: string
  ): boolean => {
    if (operator === "$eq") return propertyValue === comparisonValue;
    if (operator === "$ne") return propertyValue !== comparisonValue;
    if (operator === "$gt") return propertyValue > comparisonValue;
    if (operator === "$lt") return propertyValue < comparisonValue;
    if (operator === "$gte") return propertyValue >= comparisonValue;
    if (operator === "$lte") return propertyValue <= comparisonValue;
    if (operator === "$contains") {
      if (
        typeof propertyValue === "string" &&
        typeof comparisonValue === "string"
      ) {
        // Handle case-sensitive or case-insensitive contains
        if (!isCaseSensitive) {
          propertyValue = propertyValue.toLowerCase();
          comparisonValue = comparisonValue.toLowerCase();
        }
        return propertyValue.includes(comparisonValue);
      }
      if (Array.isArray(propertyValue)) {
        return propertyValue.includes(comparisonValue);
      }
    }
    return false;
  };

  const matchCriteria = (property: T, criteria: Query): boolean => {
    // Handle $match condition (always check fields in $match)
    if (criteria.$match) {
      for (const key in criteria.$match) {
        const requiredValue = criteria.$match[key];
        const propertyValue = getNestedValue(property, key);

        if (propertyValue !== requiredValue) {
          return false; // If any $match field doesn't match, return false early
        }
      }
    }
    if (criteria.$or && criteria.$or.length > 0) {
      // If $or is present, at least one of the sub-queries must match
      return criteria.$or.some((subQuery: Query) =>
        matchCriteria(property, subQuery)
      );
    }

    // Handle combined properties
    if (criteria.$combined && criteria.value) {
      const combinedValue = criteria.$combined
        .map((field: any) => getNestedValue(property, field) || "") // Get the values of combined fields
        .join(" "); // Combine values with a space
      const comparisonValue = criteria.value;

      // Perform case-sensitive or case-insensitive comparison
      return isCaseSensitive
        ? combinedValue === comparisonValue
        : combinedValue.toLowerCase() === comparisonValue.toLowerCase();
    }

    for (const key in criteria) {
      if (
        key === "$or" ||
        key === "$combined" ||
        key === "value" ||
        key === "$match"
      )
        continue; // Skip special keys

      const condition = criteria[key];
      let propertyValue = getNestedValue(property, key); // Get nested value

      if (typeof condition === "object" && condition !== null) {
        // Handle operators like $gt, $lt, $contains, etc.
        for (const operator in condition) {
          const comparisonValue = condition[operator];

          if (!compareValues(propertyValue, comparisonValue, operator)) {
            return false;
          }
        }
      } else {
        // Simple equality check
        if (propertyValue !== condition) return false;
      }
    }
    return true;
  };

  return properties.filter((property) => matchCriteria(property, query));
};
