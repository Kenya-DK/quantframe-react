import { TauriClient } from "..";
import { TauriTypes } from "$types";

export class SurveyModule {
  constructor(private readonly client: TauriClient) {}

  /** Returns the active survey for the current user, or null if there is none. */
  async getActive(): Promise<TauriTypes.Survey | null> {
    return await this.client.sendInvoke<TauriTypes.Survey | null>("survey_get_active");
  }

  /** Submits the answers for a survey (keyed by question id). */
  async submit(survey_id: string, answers: Record<string, string | string[]>): Promise<TauriTypes.SurveyResponse> {
    return await this.client.sendInvoke<TauriTypes.SurveyResponse>("survey_submit", { survey_id, answers });
  }
}
