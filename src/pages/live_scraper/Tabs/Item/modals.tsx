import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { StockItemDetailsModal } from "@components/Modals/StockItemDetails";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface ModalHooks {
  updateStockMutation: {
    mutateAsync: (data: TauriTypes.UpdateStockItem) => Promise<any>;
  };
  sellStockMutation: {
    mutateAsync: (data: TauriTypes.SellStockItem) => Promise<any>;
  };
  deleteStockMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
}

export const useStockModals = ({ updateStockMutation, sellStockMutation, deleteStockMutation }: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.minimum_price.title"),
      innerProps: {
        fields: [
          {
            name: "minimum_price",
            label: useTranslateCommon("prompts.minimum_price.fields.minimum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateCommon("prompts.minimum_price.fields.minimum_price.description"),
            },
            value: minimum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { minimum_price: number }) => {
          if (!id) return;
          const { minimum_price } = data;
          await updateStockMutation.mutateAsync({ id, minimum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenSellModal = (stock: TauriTypes.StockItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.sell_manual.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateCommon("prompts.sell_manual.fields.sell.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { sell: number }) => {
          if (!stock) return;
          const { sell } = data;
          await sellStockMutation.mutateAsync({ id: stock.id, wfm_url: stock.wfm_url, sub_type: stock.sub_type, price: sell, quantity: 1 });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.StockItem) => {
    let id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <StockItemDetailsModal
          value={item.id}
          onUpdate={async (data) => {
            await updateStockMutation.mutateAsync(data);
            modals.close(id);
          }}
        />
      ),
    });
  };

  const OpenDeleteModal = (id: number) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteStockMutation.mutateAsync(id),
    });
  };

  return {
    OpenMinimumPriceModal,
    OpenSellModal,
    OpenInfoModal,
    OpenDeleteModal,
  };
};
