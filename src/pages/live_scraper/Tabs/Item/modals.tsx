import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { StockItemDetailsModal } from "@components/Modals/StockItemDetails";
import { StockItemUpdate } from "@components/Forms/StockItemUpdate";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { GenerateTradeMessageModal, GenerateTradeMessageModalProps } from "@components/Modals/GenerateTradeMessage";

interface ModalHooks {
  updateMutation: {
    mutateAsync: (data: TauriTypes.UpdateStockItem) => Promise<any>;
  };
  updateMultipleMutation: {
    mutateAsync: (data: { ids: number[]; input: TauriTypes.UpdateStockItem }) => Promise<any>;
  };
  sellStockMutation: {
    mutateAsync: (data: TauriTypes.SellStockItem) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  deleteMultipleMutation: {
    mutateAsync: (ids: number[]) => Promise<any>;
  };
}

export const useModals = ({ updateMutation, updateMultipleMutation, sellStockMutation, deleteMutation, deleteMultipleMutation }: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modalKey: "prompt",
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
          await updateMutation.mutateAsync({ id, minimum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenWTSModal = (input: GenerateTradeMessageModalProps) => {
    modals.open({
      size: "100%",
      title: useTranslateModals("generate_trade_message.title", { count: input.items.length }),
      withCloseButton: false,
      children: <GenerateTradeMessageModal {...input} />,
    });
  };

  const OpenSellModal = (stock: TauriTypes.StockItem) => {
    modals.openContextModal({
      modalKey: "prompt",
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
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <StockItemDetailsModal value={item.id} />,
    });
  };

  const OpenUpdateMultipleModal = (ids: number[]) => {
    let id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <StockItemUpdate
          values={ids}
          onUpdate={async (input) => {
            if (ids.length == 1) await updateMutation.mutateAsync(input);
            else await updateMultipleMutation.mutateAsync({ ids, input: { ...input, id: -1 } });
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
      onConfirm: async () => await deleteMutation.mutateAsync(id),
    });
  };

  const OpenDeleteMultipleModal = (ids: number[]) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_multiple_items.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_multiple_items.message", { count: ids.length })}</Text>,
      labels: {
        confirm: useTranslateCommon("prompts.delete_multiple_items.confirm"),
        cancel: useTranslateCommon("prompts.delete_multiple_items.cancel"),
      },
      onConfirm: async () => await deleteMultipleMutation.mutateAsync(ids),
    });
  };

  return {
    OpenMinimumPriceModal,
    OpenSellModal,
    OpenInfoModal,
    OpenUpdateMultipleModal,
    OpenDeleteModal,
    OpenDeleteMultipleModal,
    OpenWTSModal,
  };
};
