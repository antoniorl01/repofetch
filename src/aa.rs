use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryData {
    pub name: String, // 1st all, json.name
    pub head: String,
    pub pending: String,
    pub version: String,
    pub created: String, // 1st call, json.created_at
    pub languages: String, // /languages -> de ahi, calcular el porcentaje
    pub dependencies: String,
    pub authors: String,
    pub updated_at: String, // 1st call, json.update_at
    pub contributors: String,
    pub repo: String,
    pub commits: String, // Done
    pub lines_of_code: String, // on local: git ls-files | xargs wc -l, en languages nos devuelve el numero de chars
    pub size: String, // 1st call, json.size me lo da en kilobytes
    pub license: String, // 1st call, json.license
    pub language: String // 1st call, json.language
}
