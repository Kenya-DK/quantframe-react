import { Container, Text } from "@mantine/core";
import { useQuery } from "@tanstack/react-query";
import api from "../../api";
import { DataGrid } from "@components/DataGrid";
import { Loading } from "../../components/Loading";

export default function TestPage() {

  const { data } = useQuery({
    queryKey: ['cache_items'],
    queryFn: () => api.cache.getTradableItems(),
  })
  return (
    <Container size={"100%"}>
      <DataGrid
        height={500}
        customLoader={<Loading />}
        columns={[
          {
            accessor: 'wfm_id',
          },
          {
            accessor: 'wfm_url_name',
          },
          {
            accessor: 'trade_tax',
          },
          {
            accessor: 'mr_requirement',
            render: (value) => <TextTranslate i18nKey={value.image_url} />
          }
        ]}
        fetching={true}
        records={data || []}
      />
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