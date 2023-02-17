#[derive(Clone, Debug, PartialEq, serde::Serialize, Deserialize)]
pub struct InputData {
    pub name: String,
    pub profit_margin: f64,
}
