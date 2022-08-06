
// TODO
// maybe different predictions struct's for card (explorer) 
// and page (complete)
// (or maybe a lot of conditional attributes & different fn's)
// create here the functions to fetch graphql & ipfs
// note about ipfs: use a gateway, avoid running a local node!
#![allow(non_snake_case)]

use serde::{Deserialize};
use chrono::{ TimeZone, Utc };

fn convert_to_datetime(timestamp: String) -> String { //change signature to DateTime<Utc>
    let timestamp_parsed = timestamp.parse::<i64>().unwrap();
    let timestamp_datetime = Utc.timestamp(timestamp_parsed, 0);
    timestamp_datetime.to_rfc2822() // this only when presented
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct PredictionCardData {
    pub id: String,
    pub condition: Option<String>,
}


#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Prediction {
    pub id: String,
    pub condition: Option<String>,
    pub question_id: Option<String>,
    pub created: String, //DateTime<Utc>
    pub collateral: Option<String>,
    pub totalCollateral: Option<String>,
    pub status: Option<String>,
    pub timeout: Option<String>,
    pub probabilitiesTotal: Vec<u16>,
    pub manifesto: Option<PredictionManifesto>
}
impl Prediction {
    pub fn empty(id: String) -> Self {
        Prediction {
            id,
            condition: None,
            question_id: None,
            created: "not found".to_string(),
            collateral: None,
            totalCollateral: Some("not found".to_string()),
            status: Some("not found".to_string()),
            timeout: None,
            probabilitiesTotal: Vec::new(),
            manifesto: None,
        }
    }
    pub fn set(data: Prediction) -> Self {
        Prediction {
            id: data.id,
            condition: data.condition.clone(),
            question_id: data.question_id.clone(),
            created: convert_to_datetime(data.created), // prob should be a datetime type
            collateral: data.collateral,
            totalCollateral: data.totalCollateral,
            status: data.status,
            timeout: data.timeout,
            probabilitiesTotal: data.probabilitiesTotal,
            manifesto: data.manifesto,
        }
    }
    pub fn get_manifesto_cid(question_id: String) -> String { // maybe change to no args
        let multi_str = str::replace(&question_id, "0x", "1220");
        if let Ok(ipfs_cid) = hex::decode(multi_str) {
            let question = bs58::encode(ipfs_cid).into_string();
            question
        } else {
            "Error in manifesto retrieval".to_string()
        }
        
    }
    
}

/* impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
} */

#[derive(Clone, PartialEq, Deserialize, Debug, Eq)]
pub struct PredictionManifesto {
    pub startDate:Option<String>, // //DateTime<Utc>
    pub endDate:Option<String>,   ////DateTime<Utc>
    pub oracle:Option<String>,
    pub selectedSafeAddress:Option<String>,
    pub minionToken:Option<String>,
    pub paymentRequested:Option<String>,
    pub title:Option<String>,
    pub description:Option<String>,
    pub selectedMinion:Option<String>,
    pub conditionOutcomes:Option<String>, // number
    pub condition:Option<String>,
    pub typeOracle:Option<String>,
    pub conditionDescription:Option<String>,
    pub outcomes:Option<Vec<String>>,
}
impl PredictionManifesto {
    pub fn parse(data: PredictionManifesto) -> Self {
        PredictionManifesto {
            startDate:Some(convert_to_datetime(data.startDate.unwrap())), // //DateTime<Utc>
            endDate:Some(convert_to_datetime(data.endDate.unwrap())),   ////DateTime<Utc>
            oracle:data.oracle,
            selectedSafeAddress:data.selectedSafeAddress,
            minionToken:data.minionToken,
            paymentRequested:data.paymentRequested,
            title:data.title,
            description:data.description,
            selectedMinion:data.selectedMinion,
            conditionOutcomes:data.conditionOutcomes, // number
            condition:data.condition,
            typeOracle:data.typeOracle,
            conditionDescription:data.conditionDescription,
            outcomes:data.outcomes,
        }
    }
}