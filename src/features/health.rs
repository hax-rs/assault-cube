use hax::{Feature, FeatureTrait};
use serde::{Deserialize, Serialize};

#[derive(Feature, Debug, Default, Serialize, Deserialize)]
pub struct HealthFeature {
    //
}

#[typetag::serde]
impl FeatureTrait for HealthFeature {
    fn tick(&mut self) {
        todo!()
    }
}
