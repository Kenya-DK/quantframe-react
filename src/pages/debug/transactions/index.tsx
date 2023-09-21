import { Text, Card, Group, Grid, Tooltip, ActionIcon } from "@mantine/core";
import { useWarframeMarketContextContext } from "../../../contexts";
import { DataTable } from "mantine-datatable";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCopy } from "@fortawesome/free-solid-svg-icons";

export const Transactions = () => {
  const { transactions } = useWarframeMarketContextContext();
  return (
    <Card>
      <Group position="apart" mb="xs">
        <Text weight={500}>Transactions</Text>
      </Group>
      <Grid>
        <DataTable
          sx={{ marginTop: "20px" }}
          striped
          records={transactions}
          // define columns
          columns={[
            {
              accessor: 'item_name',
              title: "Name",
              width: 250,
            },
            {
              accessor: 'price',
              title: "Price",
              width: 250,
            },
            {
              accessor: 'quantity',
              title: "Quantity",
              width: 250,
            },
            {
              accessor: 'datetime',
              title: "Date",
              width: 250,
            },
            {
              accessor: 'transaction_type',
              title: "Type",
              width: 250,
            },
            {
              accessor: 'actions',
              width: 100,
              title: "Actions",
              render: ({ item_url, datetime, transaction_type, price }) =>
                <Group position="center" >
                  <Tooltip label="">
                    <ActionIcon variant="filled" onClick={async () => {
                      // Set the text to be copied
                      await navigator.clipboard.writeText(`INSERT INTO transactions (name, datetime, transactionType, price) VALUES ('${item_url}', '${datetime}', '${transaction_type}', ${price})`);

                    }} >
                      <FontAwesomeIcon icon={faCopy} />
                    </ActionIcon>
                  </Tooltip>
                </Group>
            },
          ]}
        />
      </Grid>
    </Card>
  );
}