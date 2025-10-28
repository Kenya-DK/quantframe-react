import { Group, Box, Button, Collapse, Switch, Grid, NumberInput, Stack } from "@mantine/core";
import { useTranslateForms } from "@hooks/useTranslate.hook";
import { useForm } from "@mantine/form";
import { TauriTypes } from "$types";
import { MinMax } from "@components/Forms/MinMax";
import { RivenFilterAttribute } from "@components/Forms/RivenFilterAttribute";

export type RivenFilterProps = {
  value: TauriTypes.StockRivenFilter;
  onSubmit: (values: TauriTypes.StockRivenFilter) => void;
};

export function RivenFilter({ value, onSubmit }: RivenFilterProps) {
  // Translate general
  const useTranslateForm = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForms(`riven_filter.${key}`, { ...context }, i18Key);
  const useTranslateFormFields = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`fields.${key}`, { ...context }, i18Key);
  const useTranslateButtons = (key: string, context?: { [key: string]: any }, i18Key?: boolean) =>
    useTranslateForm(`buttons.${key}`, { ...context }, i18Key);

  // User form
  const form = useForm({
    initialValues: {
      ...value,
      attributes: value.attributes ?? [],
    },
    validate: {},
  });
  return (
    <Box w={"100%"} p={"sm"}>
      <form
        onSubmit={form.onSubmit((data) => {
          onSubmit(data);
        })}
      >
        <Group gap="md">
          <Switch
            label={useTranslateFormFields("enabled.label")}
            checked={form.values.enabled}
            onChange={(event) => form.setFieldValue("enabled", event.currentTarget.checked)}
          />
        </Group>
        <Collapse in={form.values.enabled}>
          <Grid>
            <Grid.Col span={6}>
              <MinMax
                label={useTranslateFormFields("re_rolls.label")}
                value={[form.values.re_rolls?.min || 0, form.values.re_rolls?.max || 0]}
                onChange={(re_rolls) => {
                  form.setFieldValue("re_rolls", { min: re_rolls?.[0] || 0, max: re_rolls?.[1] || 0 });
                }}
              />
              <MinMax
                label={useTranslateFormFields("rank.label")}
                value={[form.values.rank?.min || 0, form.values.rank?.max || 0]}
                onChange={(rank) => {
                  form.setFieldValue("rank", { min: rank?.[0] || 0, max: rank?.[1] || 0 });
                }}
              />
              <MinMax
                label={useTranslateFormFields("mastery_rank.label")}
                value={[form.values.mastery_rank?.min || 0, form.values.mastery_rank?.max || 0]}
                onChange={(mastery_rank) => {
                  form.setFieldValue("mastery_rank", { min: mastery_rank?.[0] || 0, max: mastery_rank?.[1] || 0 });
                }}
              />
            </Grid.Col>
            <Grid.Col span={6}>
              <Group gap="md">
                <Switch
                  label={useTranslateFormFields("required_negative.label")}
                  checked={form.values.required_negative}
                  onChange={(event) => form.setFieldValue("required_negative", event.currentTarget.checked)}
                />
              </Group>
              <NumberInput
                label={useTranslateFormFields("similarity.label")}
                placeholder="0"
                value={form.values.similarity || 0}
                min={0}
                max={100}
                onChange={(event) => {
                  const value = Number(event);
                  if (value == 0) form.setFieldValue("similarity", undefined);
                  else form.setFieldValue("similarity", value);
                }}
                suffix=" %"
                mt="md"
              />
              <Stack mt="md" gap="md">
                {(form.values.attributes ?? []).map((attribute, index) => (
                  <RivenFilterAttribute
                    key={index}
                    value={attribute}
                    onChange={(value) => {
                      const attributes = [...(form.values.attributes ?? [])];
                      attributes[index] = value;
                      form.setFieldValue("attributes", attributes);
                    }}
                  />
                ))}
              </Stack>
            </Grid.Col>
          </Grid>
        </Collapse>
        {form.isDirty() && (
          <Button mt={15} type="submit" color="blue" radius="md" fullWidth>
            {useTranslateButtons("save.label")}
          </Button>
        )}
      </form>
    </Box>
  );
}
