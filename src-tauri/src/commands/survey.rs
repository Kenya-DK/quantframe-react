use std::sync::Mutex;

use qf_api::types::*;
use utils::{get_location, Error};

use crate::{app::AppState, utils::ErrorFromExt};

#[tauri::command]
pub async fn survey_get_active(
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<Option<Survey>, Error> {
    let app = app.lock()?.clone();
    match app.qf_client.survey().get_active().await {
        Ok(survey) => Ok(survey),
        Err(e) => {
            let err = Error::from_qf(
                "SurveyGetActive",
                "Failed to fetch the active survey",
                e,
                get_location!(),
            );
            err.log("survey_get_active.log");
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn survey_submit(
    survey_id: String,
    answers: serde_json::Value,
    app: tauri::State<'_, Mutex<AppState>>,
) -> Result<SurveyResponse, Error> {
    let app = app.lock()?.clone();
    match app.qf_client.survey().submit(survey_id, answers).await {
        Ok(response) => Ok(response),
        Err(e) => {
            let err = Error::from_qf(
                "SurveySubmit",
                "Failed to submit the survey response",
                e,
                get_location!(),
            );
            err.log("survey_submit.log");
            Err(err)
        }
    }
}
