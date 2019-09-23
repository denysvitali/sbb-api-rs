#[derive(Serialize, Deserialize, Debug)]
pub struct LegendOccupancy {
    pub actions: Vec<String>,
    pub code : Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegendItem {
    pub actions: Vec<String>,
    pub code : Option<String>,
    pub description: Option<String>,
}