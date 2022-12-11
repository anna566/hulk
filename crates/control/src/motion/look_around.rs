use color_eyre::Result;
use context_attribute::context;
use framework::MainOutput;
use types::{
    configuration::LookAround as LookAroundConfiguration, HeadJoints, MotionCommand, SensorData,
};

pub struct LookAround {}

#[context]
pub struct CreationContext {
    pub config: Parameter<LookAroundConfiguration, "control.look_around">,
}

#[context]
pub struct CycleContext {
    pub config: Parameter<LookAroundConfiguration, "control.look_around">,

    pub motion_command: RequiredInput<Option<MotionCommand>, "motion_command?">,
    pub sensor_data: Input<SensorData, "sensor_data">,
}

#[context]
#[derive(Default)]
pub struct MainOutputs {
    pub look_around: MainOutput<Option<HeadJoints>>,
}

impl LookAround {
    pub fn new(_context: CreationContext) -> Result<Self> {
        Ok(Self {})
    }

    pub fn cycle(&mut self, _context: CycleContext) -> Result<MainOutputs> {
        Ok(MainOutputs::default())
    }
}
