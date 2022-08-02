
// TODO
// maybe different prediction data's for card (explorer)
// and page (complete)
// create here the functions to fetch graphql & ipfs

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Prediction {
    pub condition: Option<String>,
    pub id: String,
//    pub name: String,
//    pub keywords: Vec<String>,
//    pub image_url: String,
}
impl Prediction {
    pub fn get(id: &String) -> Self {
        Prediction {id: id.to_string(), condition: Some("Yeah!".to_string())}
    }
}
