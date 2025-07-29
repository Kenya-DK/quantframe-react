import { useQuery } from "@tanstack/react-query";
import { TauriClient } from "..";
import { TauriTypes } from "$types";
export class DashboardModule {
  constructor(private readonly client: TauriClient) {}

  summary() {
    return useQuery({
      queryKey: ["dashboard_summary"],
      queryFn: () => this.client.sendInvoke<TauriTypes.DashboardSummary>("dashboard_summary"),
      refetchOnWindowFocus: true,
      retry: false,
    });
  }
}
