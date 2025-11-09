import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { WishListItemDetailsModal } from "@components/Modals/WishListItemDetails";
import { useTranslateCommon, useTranslateModals } from "@hooks/useTranslate.hook";
import { WishListItemUpdate } from "@components/Forms/WishListItemUpdate";
import { GenerateTradeMessageModal, GenerateTradeMessageModalProps } from "@components/Modals/GenerateTradeMessage";

interface ModalHooks {
  updateMutation: {
    mutateAsync: (data: TauriTypes.UpdateWishListItem) => Promise<any>;
  };
  updateMultipleMutation: {
    mutateAsync: (data: { ids: number[]; input: TauriTypes.UpdateWishListItem }) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  deleteMultipleMutation: {
    mutateAsync: (ids: number[]) => Promise<any>;
  };
  boughtMutation: {
    mutateAsync: (data: TauriTypes.BoughtWishListItem) => Promise<any>;
  };
}

export const useStockModals = ({ updateMutation, deleteMutation, boughtMutation, updateMultipleMutation, deleteMultipleMutation }: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, maximum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.maximum_price.title"),
      innerProps: {
        fields: [
          {
            name: "maximum_price",
            label: useTranslateCommon("prompts.maximum_price.fields.maximum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateCommon("prompts.maximum_price.fields.maximum_price.description"),
            },
            value: maximum_price,
            type: "number",
          },
        ],
        onConfirm: async (data: { maximum_price: number }) => {
          if (!id) return;
          const { maximum_price } = data;
          await updateMutation.mutateAsync({ id, maximum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.WishListItem) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <WishListItemDetailsModal value={item.id} />,
    });
  };

  const OpenUpdateMultipleModal = (ids: number[]) => {
    let id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <WishListItemUpdate
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
  const OpenWTBModal = (input: GenerateTradeMessageModalProps) => {
    modals.open({
      size: "100%",
      title: useTranslateModals("generate_trade_message.title", { count: input.items.length }),
      withCloseButton: false,
      children: <GenerateTradeMessageModal {...input} />,
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
  const OpenBoughtModal = (stock: TauriTypes.WishListItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateCommon("prompts.bought_manual.title"),
      innerProps: {
        fields: [
          {
            name: "bought",
            label: useTranslateCommon("prompts.bought_manual.fields.bought.label"),
            attributes: {
              min: 0,
            },
            value: 0,
            type: "number",
          },
        ],
        onConfirm: async (data: { bought: number }) => {
          if (!stock) return;
          const { bought } = data;
          await boughtMutation.mutateAsync({ id: stock.id, wfm_url: stock.wfm_url, sub_type: stock.sub_type, price: bought, quantity: 1 });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  return {
    OpenMinimumPriceModal,
    OpenInfoModal,
    OpenUpdateMultipleModal,
    OpenDeleteModal,
    OpenDeleteMultipleModal,
    OpenBoughtModal,
    OpenWTBModal,
  };
};
