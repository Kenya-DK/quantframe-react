import { Group, Pagination, Select, Text } from "@mantine/core";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

export type PaginationFooterProps = {
  page: number;
  limit: number;
  total: number;
  limitOptions?: string[];
  onPageChange: (page: number) => void;
  onLimitChange: (limit: number) => void;
};

export function PaginationFooter({
  page,
  limit,
  total,
  limitOptions = ["25", "50", "100", "200"],
  onPageChange,
  onLimitChange,
}: PaginationFooterProps) {
  const start = (page - 1) * limit + 1;
  const end = Math.min(page * limit, total);

  return (
    <Group grow mt={"md"}>
      <Text>
        {useTranslateCommon("pagination_total_items", {
          start,
          end,
          total,
        })}
      </Text>
      <Group justify="flex-end">
        <Select
          size="xs"
          w={90}
          allowDeselect={false}
          value={String(limit)}
          data={limitOptions}
          onChange={(value) => {
            if (!value) return;
            onLimitChange(Number(value));
          }}
        />
        <Pagination value={page} onChange={onPageChange} total={Math.ceil(total / limit)} />
      </Group>
    </Group>
  );
}
