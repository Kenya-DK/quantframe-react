import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { WishListItemDetailsModal } from "@components/Modals/WishListItemDetails";
import { useTranslateCommon } from "@hooks/useTranslate.hook";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  useTranslatePrompt: (key: string, context?: { [key: string]: any }) => string;
  updateWishListMutation: {
    mutateAsync: (data: TauriTypes.UpdateWishListItem) => Promise<any>;
  };
  deleteWishListMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  boughtWishListMutation: {
    mutateAsync: (data: TauriTypes.BoughtWishListItem) => Promise<any>;
  };
}

export const useStockModals = ({ useTranslatePrompt, updateWishListMutation, deleteWishListMutation, boughtWishListMutation }: ModalHooks) => {
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
          await updateWishListMutation.mutateAsync({ id, maximum_price });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.WishListItem) => {
    const id = modals.open({
      size: "100%",
      withCloseButton: false,
      children: (
        <WishListItemDetailsModal
          value={item.id}
          onUpdate={async (data) => {
            await updateWishListMutation.mutateAsync(data);
            modals.close(id);
          }}
        />
      ),
    });
  };

  const OpenUpdateModal = (_items: TauriTypes.UpdateStockItem[]) => {
    modals.open({
      title: useTranslatePrompt("update_bulk.title"),
      children: <></>,
    });
  };

  const OpenDeleteModal = (id: number) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_item.title"),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_item.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_item.confirm"), cancel: useTranslateCommon("prompts.delete_item.cancel") },
      onConfirm: async () => await deleteWishListMutation.mutateAsync(id),
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
          await boughtWishListMutation.mutateAsync({ id: stock.id, wfm_url: stock.wfm_url, sub_type: stock.sub_type, price: bought, quantity: 1 });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  return {
    OpenMinimumPriceModal,
    OpenInfoModal,
    OpenUpdateModal,
    OpenDeleteModal,
    OpenBoughtModal,
  };
};
