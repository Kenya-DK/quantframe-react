import { Box, Button, Card, Checkbox, Divider, Grid, Group, NumberInput, TagsInput, Title } from "@mantine/core";
import { QuantframeApiTypes, TauriTypes } from "$types";
import { useForm } from "@mantine/form";
import { useEffect, useState } from "react";
import dayjs from "dayjs";
import { Round } from "@utils/helper";
import { DataTable } from "mantine-datatable";
import { useTranslatePages } from "@hooks/useTranslate.hook";
import { HasPermission } from "@api/index";
import { PatreonOverlay } from "@components/Shared/PatreonOverlay";
export interface FindInterestingRivensModalProps {
  date: Date;
  rivens: QuantframeApiTypes.RivenPriceDto[];
  onSubmit?: (data: { overrideExistingPrices: boolean; rivens: RivenPriceWithDiscount[]; tags: string[] }) => void;
}

interface RivenPriceWithDiscount extends QuantframeApiTypes.RivenPriceDto {
  discount_price: number;
  potential_profit?: number;
}

// FindInteresting
export function FindInterestingRivensModal({ date, rivens, onSubmit }: FindInterestingRivensModalProps) {
  const [filteredRivens, setFilteredRivens] = useState<RivenPriceWithDiscount[]>([]);
  const [canUse, setCanUse] = useState<boolean>(false);
  const filterForm = useForm({
    initialValues: {
      minPrice: 50,
      minVolume: 10,
      discountPercentage: 50,
      roundToNearest: 5,
    },
    onValuesChange: () => UpdateFilteredRivens(),
  });

  const useTranslate = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslatePages(`trade_messages.tabs.riven.modals.find_interesting_rivens.${key}`, { ...context }, i18Key);
  const useTranslateFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`fields.${key}`, { ...context }, i18Key);
  const useTranslateTitle = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslate(`titles.${key}`, { ...context }, i18Key);
  const exportSettingsForm = useForm({
    initialValues: {
      overrideExistingPrices: true,
      clearExistingEntries: false,
      tags: ["Interesting"] as string[],
    },
  });

  const UpdateFilteredRivens = () => {
    const { minPrice, minVolume, discountPercentage, roundToNearest } = filterForm.values;
    const newFilteredRivens = rivens.filter((riven) => riven.min_price >= minPrice && riven.volume >= minVolume);

    setFilteredRivens(
      newFilteredRivens.map((riven) => {
        const discount_price = riven.min_price * (1 - discountPercentage / 100);
        const rounded_discount_price = Round(discount_price, roundToNearest);
        const potential_profit = riven.min_price - rounded_discount_price;
        return { ...riven, discount_price: rounded_discount_price, potential_profit };
      })
    );
  };

  useEffect(() => {
    UpdateFilteredRivens();
  }, []);
  // Check permissions for export on mount
  useEffect(() => {
    HasPermission(TauriTypes.PermissionsFlags.FIND_INTERESTING_RIVENS).then((res) => setCanUse(res));
  }, []);
  return (
    <Box pos={"relative"}>
      <PatreonOverlay permission={TauriTypes.PermissionsFlags.FIND_INTERESTING_RIVENS} tier="T2+" />
      <Title order={4}>{useTranslate("title", { date: dayjs(date).format("YYYY-MM-DD HH:mm:ss") })}</Title>
      <Grid>
        <Grid.Col span={4}>
          <Card withBorder mt="md" p="sm">
            <Title order={4}>{useTranslateTitle("filtered")}</Title>
            <Group grow>
              <NumberInput disabled={!canUse} min={0} label={useTranslateFields("min_price_label")} {...filterForm.getInputProps("minPrice")} />
              <NumberInput disabled={!canUse} min={0} label={useTranslateFields("min_volume_label")} {...filterForm.getInputProps("minVolume")} />
            </Group>
          </Card>
          <Divider my="md" />
          <Card withBorder mt="md" p="sm">
            <Title order={4}>{useTranslateTitle("discount_settings")}</Title>
            <Group grow>
              <NumberInput
                min={0}
                max={100}
                label={useTranslateFields("discount_percentage_label")}
                {...filterForm.getInputProps("discountPercentage")}
              />
              <NumberInput
                disabled={!canUse}
                min={1}
                label={useTranslateFields("round_to_nearest_label")}
                {...filterForm.getInputProps("roundToNearest")}
              />
            </Group>
          </Card>
          <Card withBorder mt="md" p="sm">
            <Title order={4}>{useTranslateTitle("export_settings")}</Title>
            <Group grow mt={"md"}>
              <Checkbox
                label={useTranslateFields("override_existing_prices_label")}
                {...exportSettingsForm.getInputProps("overrideExistingPrices", { type: "checkbox" })}
              />
            </Group>
            <TagsInput disabled={!canUse} label={useTranslateFields("tags_label")} {...exportSettingsForm.getInputProps("tags")} />
            <Divider my="md" />
            <Button
              fullWidth
              disabled={filteredRivens.length === 0 || !canUse}
              onClick={() => {
                if (!onSubmit) return;
                if (filteredRivens.length === 0) return;
                onSubmit({ ...exportSettingsForm.values, rivens: filteredRivens });
              }}
            >
              {useTranslate("buttons.export_to_trade_messages")}
            </Button>
          </Card>
        </Grid.Col>
        <Grid.Col span={8}>
          <DataTable
            height={"75vh"}
            withColumnBorders
            striped
            highlightOnHover
            columns={[
              { accessor: "name", title: useTranslate("datatable_columns.name") },
              { accessor: "min_price", title: useTranslate("datatable_columns.min_price") },
              { accessor: "discount_price", title: useTranslate("datatable_columns.discount_price") },
              { accessor: "potential_profit", title: useTranslate("datatable_columns.potential_profit") },
              { accessor: "volume", title: useTranslate("datatable_columns.volume") },
            ]}
            records={filteredRivens}
          />
        </Grid.Col>
      </Grid>
    </Box>
  );
}
