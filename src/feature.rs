use hax::{Feature, FeatureTrait};
use serde::{Deserialize, Serialize};

#[derive(Feature, Debug, Default, Serialize, Deserialize)]
pub struct ExampleFeature {
    pub foo: u32,
}

#[typetag::serde]
impl FeatureTrait for ExampleFeature {
    fn setup(&mut self) {
        println!("ExampleFeature::setup");
    }

    fn tick(&mut self) {
        println!("ExampleFeature::tick");
    }

    fn cleanup(&mut self) {
        println!("ExampleFeature::cleanup");
    }
}
