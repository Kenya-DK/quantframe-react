import { useEffect, useState } from "react";
import { merge } from 'lodash'
import { store } from "@store/index";

export function useStorage<T>(key: string, defaultValue: T): [T, boolean, (value: T) => void] {
  const [value, setValue] = useState(defaultValue);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    (async () => {
      setLoading(true)
      const item = await store.get<T>(key)
      if (!item) await store.set(key, defaultValue)
      setValue(item ?? defaultValue)
      setLoading(false)
    })()
  }, [])

  const setValueWrap = (value: T) => {
    try {
      (async () => {
        setLoading(true)
        const currentSettings = await store.get<T>(key)
        const promise = store.set(key, merge(defaultValue, currentSettings, value))
        await store.save()
        setValue(value);
        setLoading(false)
        return promise
      })()
    } catch (e) { console.error(e) }
  };

  return [value, loading, setValueWrap];
}