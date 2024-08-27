import { Container, Text } from "@mantine/core";
import { SelectMultipleTradableItems } from "../../components/SelectMultipleTradableItems";



export default function TestPage() {
  return (
    <Container size={"100%"}>
      <SelectMultipleTradableItems
        leftTitle={"Tradable Items"}
        rightTitle={"Blacklisted Items"}
        // onChange={(items) => { form.setFieldValue('stock_item.blacklist', items) }}
        // selectedItems={form.values.stock_item.blacklist || []} />
        onChange={(items) => { console.log(items) }}
        selectedItems={[]} />
    </Container>
  );
}

export type TextTranslateProps = {
  i18nKey: string;
}
export function TextTranslate({ i18nKey }: TextTranslateProps) {
  console.log("Render TextTranslate");
  return (
    <Text >
      {i18nKey}
    </Text>);
}