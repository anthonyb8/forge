pub mod cmake;
pub mod meson;

use crate::Result;
use strum_macros::EnumString;

#[derive(Debug, EnumString, strum_macros::VariantNames, Clone, Copy)]
pub enum TestFrameworkEnum {
    GTest,
    Boost,
    Unit,
    None,
}

#[derive(Debug, EnumString, strum_macros::VariantNames)]
pub enum BuildSystemEnum {
    CMake,
    Meson,
    Make,
}

pub trait BuildSystem {
    fn init(&self) -> Result<()>;
    fn build(&self) -> Result<()>;
    fn google_test(&self) -> String;
    fn boost_test(&self) -> String;
    fn unit_test(&self) -> String;
    fn test_init(&self, framework: &TestFrameworkEnum) -> String {
        match framework {
            TestFrameworkEnum::GTest => self.google_test(),
            TestFrameworkEnum::Boost => self.boost_test(),
            TestFrameworkEnum::Unit => self.unit_test(),
            TestFrameworkEnum::None => "".to_string(),
        }
    }
}
