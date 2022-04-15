pub enum LightType {
    Red,
    Yellow,
    Green
}

pub struct TrafficSignalLight {
    light: LightType
}

pub trait SignalConfig {
    fn duration( self)  -> i16;
}

impl TrafficSignalLight {
    pub fn new(light_type: LightType) -> Self {
        Self { light: light_type}
    }
}

impl SignalConfig for TrafficSignalLight {
    fn duration(self) -> i16 {
        match self.light {
            LightType::Red=> 15,
            LightType::Green=> 20,
            LightType::Yellow=> 30,
        }
    }
}