use crate::utils::modules::WsError;
// Route structure with parameter support
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Route {
    pub protocol: String,
    pub path: String,
    pub parameter: Option<String>,
}

impl Route {
    pub fn parse(route_str: &str) -> Result<Self, WsError> {
        let mut route_str = route_str.to_string();
        // Handle if the v1 request is in the old format
        if route_str.contains("@WS") {
            route_str = route_str.replace("@WS/", "@wfm|");
        }

        if let Some(pipe_pos) = route_str.find('|') {
            let protocol = route_str[..pipe_pos].to_string();
            let path_and_param = &route_str[pipe_pos + 1..];

            // Check for parameter (after colon)
            if let Some(colon_pos) = path_and_param.find(':') {
                let path = path_and_param[..colon_pos].to_string();
                let parameter = Some(path_and_param[colon_pos + 1..].to_string());
                Ok(Route {
                    protocol,
                    path,
                    parameter,
                })
            } else {
                let path = path_and_param.to_string();
                Ok(Route {
                    protocol,
                    path,
                    parameter: None,
                })
            }
        } else {
            Err(WsError::InvalidPath(route_str.to_string()))
        }
    }

    // Get the base path without parameter for routing
    pub fn base_path(&self) -> &str {
        &self.path
    }

    // Get the full path with parameter for exact matching
    pub fn full_path(&self) -> String {
        match &self.parameter {
            Some(param) => format!("{}:{}", self.path, param),
            None => self.path.clone(),
        }
    }
}
