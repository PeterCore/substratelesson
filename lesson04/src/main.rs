mod signallight;
use signallight::{TrafficSignalLight, SignalConfig, LightType};

fn main() {
    let light = TrafficSignalLight::new(LightType::Yellow);
    let duration= light.duration();
    println!("{}", duration);


    //sum
    let vec: [u32; 7] = [3,23,1,31,31,1,4,];
    println!("{:?}", add_vec(&vec));


}

fn add_vec(vec: &[u32]) ->Option<u32> {
    let mut result: Option<u32> = Some(0);
    for item in vec.to_vec()  {
        if result.is_none() {
            return None
        }
      result = result.unwrap().checked_add(item) ;
    }
    result
}



