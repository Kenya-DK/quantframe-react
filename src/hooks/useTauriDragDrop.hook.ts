import { useEffect, useRef } from "react";
import { getCurrentWebview } from "@tauri-apps/api/webview";

type UseTauriDragDropOptions = {
  onDrop?: (path: string) => Promise<void> | void;
  onDropMultiple?: (paths: string[]) => Promise<void> | void;
  onEnter?: () => void;
  onOver?: () => void;
  onLeave?: () => void;
  onCancel?: () => void;
};

export function useTauriDragDrop({ onDrop, onDropMultiple, onEnter, onOver, onLeave, onCancel }: UseTauriDragDropOptions) {
  const onDropRef = useRef(onDrop);
  const onDropMultipleRef = useRef(onDropMultiple);
  const onEnterRef = useRef(onEnter);
  const onOverRef = useRef(onOver);
  const onLeaveRef = useRef(onLeave);
  const onCancelRef = useRef(onCancel);

  useEffect(() => {
    onDropRef.current = onDrop;
    onDropMultipleRef.current = onDropMultiple;
    onEnterRef.current = onEnter;
    onOverRef.current = onOver;
    onLeaveRef.current = onLeave;
    onCancelRef.current = onCancel;
  });

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setup = async () => {
      unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
        const { payload } = event;

        switch (payload.type) {
          case "enter":
            onEnterRef.current?.();
            break;

          case "over":
            onOverRef.current?.();
            break;

          case "leave":
            onLeaveRef.current?.();
            break;

          case "drop":
            if (!payload.paths?.length) {
              onCancelRef.current?.();
              return;
            }

            if (payload.paths.length > 1) {
              await onDropMultipleRef.current?.(payload.paths);
            } else {
              await onDropRef.current?.(payload.paths[0]);
            }
            break;
        }
      });
    };

    setup();

    return () => {
      unlisten?.();
    };
  }, []);
}
