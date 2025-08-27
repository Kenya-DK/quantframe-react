import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { WishListItemDetailsModal } from "@components/Modals/WishListItemDetails";

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

export const useStockModals = ({
  useTranslateBasePrompt,
  useTranslatePrompt,
  updateWishListMutation,
  deleteWishListMutation,
  boughtWishListMutation,
}: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, maximum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("maximum_price.title"),
      innerProps: {
        fields: [
          {
            name: "maximum_price",
            label: useTranslateBasePrompt("maximum_price.fields.maximum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateBasePrompt("maximum_price.fields.maximum_price.description"),
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
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <WishListItemDetailsModal value={item.id} />,
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
      title: useTranslateBasePrompt("delete.title"),
      children: <Text size="sm">{useTranslateBasePrompt("delete.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateBasePrompt("delete.confirm"), cancel: useTranslateBasePrompt("delete.cancel") },
      onConfirm: async () => await deleteWishListMutation.mutateAsync(id),
    });
  };

  const OpenBoughtModal = (stock: TauriTypes.WishListItem) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("bought.title"),
      innerProps: {
        fields: [
          {
            name: "bought",
            label: useTranslateBasePrompt("bought.fields.bought.label"),
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
