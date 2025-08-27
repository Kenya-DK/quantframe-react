import { useEffect, useCallback } from "react";
import { OnTauriEvent, OffTauriEvent } from "@api/index";
import { TauriTypes } from "$types";

/**
 * Custom hook for handling Tauri events with automatic cleanup
 * @param event - The Tauri event to listen to
 * @param handler - The callback function to handle the event
 * @param deps - Dependencies array for the handler (optional)
 */
export function useTauriEvent<T = any>(event: TauriTypes.Events, handler: (data: T) => void, deps: React.DependencyList = []) {
  // Memoize the handler to ensure stable reference
  const memoizedHandler = useCallback(handler, deps);

  useEffect(() => {
    OnTauriEvent<T>(event, memoizedHandler);

    return () => {
      OffTauriEvent<T>(event, memoizedHandler);
    };
  }, [event, memoizedHandler]);
}

/**
 * Hook for multiple Tauri events with automatic cleanup
 * @param events - Array of event configurations
 */
export function useTauriEvents(
  events: Array<{
    event: TauriTypes.Events;
    handler: (data: any) => void;
    deps?: React.DependencyList;
  }>
) {
  useEffect(() => {
    const cleanupFunctions: Array<() => void> = [];

    events.forEach(({ event, handler }) => {
      OnTauriEvent(event, handler);
      cleanupFunctions.push(() => OffTauriEvent(event, handler));
    });

    return () => {
      cleanupFunctions.forEach((cleanup) => cleanup());
    };
  }, []); // Empty deps since we want to register once and cleanup on unmount
}

/**
 * Alternative approach: Hook that returns a cleanup function
 * Useful when you need conditional event registration
 */
export function useTauriEventWithCleanup<T = any>(event: TauriTypes.Events, handler: (data: T) => void): () => void {
  useEffect(() => {
    OnTauriEvent<T>(event, handler);

    return () => {
      OffTauriEvent<T>(event, handler);
    };
  }, [event, handler]);

  return useCallback(() => {
    OffTauriEvent<T>(event, handler);
  }, [event, handler]);
}
