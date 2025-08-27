import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { CreateRiven } from "@components/Forms/CreateRiven";
import { RivenFilter } from "@components/Forms/RivenFilter";
import { StockRivenDetailsModal } from "@components/Modals/StockRivenDetails";
interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  useTranslatePrompt: (key: string, context?: { [key: string]: any }) => string;
  updateStockMutation: {
    mutateAsync: (data: TauriTypes.UpdateStockRiven) => Promise<any>;
  };
  sellStockMutation: {
    mutateAsync: (data: TauriTypes.SellStockRiven) => Promise<any>;
  };
  deleteStockMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  createStockMutation: {
    mutateAsync: (data: TauriTypes.CreateStockRiven) => Promise<any>;
  };
}

export const useStockModals = ({
  useTranslateBasePrompt,
  useTranslatePrompt,
  updateStockMutation,
  sellStockMutation,
  deleteStockMutation,
  createStockMutation,
}: ModalHooks) => {
  const OpenMinimumPriceModal = (id: number, minimum_price: number) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("minimum_price.title"),
      innerProps: {
        fields: [
          {
            name: "minimum_price",
            label: useTranslateBasePrompt("minimum_price.fields.minimum_price.label"),
            attributes: {
              min: 0,
              description: useTranslateBasePrompt("minimum_price.fields.minimum_price.description"),
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

  const OpenSellModal = (stock: TauriTypes.SellStockRiven) => {
    modals.openContextModal({
      modal: "prompt",
      title: useTranslateBasePrompt("sell.title"),
      innerProps: {
        fields: [
          {
            name: "sell",
            label: useTranslateBasePrompt("sell.fields.sell.label"),
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
          await sellStockMutation.mutateAsync({ ...stock, price: sell });
        },
        onCancel: (id: string) => modals.close(id),
      },
    });
  };

  const OpenInfoModal = (item: TauriTypes.StockRiven) => {
    modals.open({
      size: "100%",
      withCloseButton: false,
      children: <StockRivenDetailsModal value={item.id} />,
    });
  };

  const OpenCreateRiven = () => {
    modals.open({
      title: useTranslatePrompt("create_riven.title"),
      size: "950px",
      children: (
        <CreateRiven
          onSubmit={async (data) => {
            await createStockMutation.mutateAsync({
              ...data,
              raw: data.wfm_weapon_url,
              rank: data.sub_type?.rank || 0,
            });
            modals.closeAll();
          }}
        />
      ),
    });
  };

  const OpenUpdateModal = (_items: TauriTypes.UpdateStockRiven[]) => {
    modals.open({
      title: useTranslatePrompt("update.title"),
      children: <></>,
    });
  };

  const OpenDeleteModal = (id: number) => {
    modals.openConfirmModal({
      title: useTranslateBasePrompt("delete.title"),
      children: <Text size="sm">{useTranslateBasePrompt("delete.message", { count: 1 })}</Text>,
      labels: { confirm: useTranslateBasePrompt("delete.confirm"), cancel: useTranslateBasePrompt("delete.cancel") },
      onConfirm: async () => await deleteStockMutation.mutateAsync(id),
    });
  };

  const OpenFilterModal = (item: TauriTypes.StockRiven) => {
    const filter = item.filter || { enabled: false, attributes: [] };
    if (!filter.attributes) filter.attributes = item.attributes.map((x) => ({ positive: x.positive, url_name: x.url_name, is_required: false }));

    modals.open({
      title: useTranslateBasePrompt("update_filter.title"),
      size: "75vw",
      children: (
        <RivenFilter
          value={filter}
          onSubmit={async (data) => {
            await updateStockMutation.mutateAsync({ id: item.id, filter: data });
            modals.closeAll();
          }}
        />
      ),
    });
  };
  return {
    OpenMinimumPriceModal,
    OpenSellModal,
    OpenFilterModal,
    OpenInfoModal,
    OpenCreateRiven,
    OpenUpdateModal,
    OpenDeleteModal,
  };
};
