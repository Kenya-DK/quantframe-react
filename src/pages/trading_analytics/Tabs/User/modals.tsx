import { modals } from "@mantine/modals";
import { Text } from "@mantine/core";
import { TauriTypes } from "$types";
import { useTranslateCommon } from "@hooks/useTranslate.hook";
import { UpdateTransaction } from "@components/Forms/UpdateTransaction";

interface ModalHooks {
  useTranslateBasePrompt: (key: string, context?: { [key: string]: any }) => string;
  useTranslatePrompt: (key: string, context?: { [key: string]: any }) => string;
  refetchQueries: () => void;
  setLoadingRows: (callback: (prev: string[]) => string[]) => void;
  updateMutation: {
    mutateAsync: (data: TauriTypes.UpdateTransaction) => Promise<any>;
  };
  deleteMutation: {
    mutateAsync: (id: number) => Promise<any>;
  };
  deleteBulkMutation: {
    mutateAsync: (ids: number[]) => Promise<any>;
  };
}

export const useModals = ({ useTranslatePrompt, setLoadingRows, updateMutation, deleteMutation, deleteBulkMutation }: ModalHooks) => {
  const OpenUpdateModal = (item: TauriTypes.TransactionDto) => {
    modals.open({
      title: useTranslatePrompt("update_title"),
      children: (
        <UpdateTransaction
          value={item}
          onSubmit={async (data) => {
            await updateMutation.mutateAsync(data);
            modals.closeAll();
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

  const OpenDeleteBulkModal = (ids: number[]) => {
    modals.openConfirmModal({
      title: useTranslateCommon("prompts.delete_bulk_item.title", { count: ids.length }),
      children: <Text size="sm">{useTranslateCommon("prompts.delete_bulk_item.message", { count: ids.length })}</Text>,
      labels: { confirm: useTranslateCommon("prompts.delete_bulk_item.confirm"), cancel: useTranslateCommon("prompts.delete_bulk_item.cancel") },
      onConfirm: async () => {
        await deleteBulkMutation.mutateAsync(ids);
        setLoadingRows(() => []);
      },
    });
  };

  return {
    OpenUpdateModal,
    OpenDeleteModal,
    OpenDeleteBulkModal,
  };
};
