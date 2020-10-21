use crate::model::enviroment::EnviromentJson;
use crate::model::project::ProjectJson;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Upload {
    project: ProjectJson,
    enviroment: EnviromentJson,
}
