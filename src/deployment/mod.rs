use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub action: String,
    pub reason: String,
    pub score: f64,
}

pub fn sdn_decision(score: f64) -> Decision {
    if score > 20.0 {
        Decision {
            action: "reroute".to_string(),
            reason: "high congestion detected".to_string(),
            score,
        }
    } else {
        Decision {
            action: "maintain".to_string(),
            reason: "traffic within expected bounds".to_string(),
            score,
        }
    }
}

pub fn nfv_decision(score: f64) -> Decision {
    if score > 15.0 {
        Decision {
            action: "scale_up".to_string(),
            reason: "demand spike detected".to_string(),
            score,
        }
    } else {
        Decision {
            action: "steady_state".to_string(),
            reason: "resources adequate".to_string(),
            score,
        }
    }
}
