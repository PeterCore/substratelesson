mod signallight;
use signallight::{TrafficSignalLight, SignalConfig, LightType};
use std::f64::consts::PI;

fn main() {
    let light = TrafficSignalLight::new(LightType::Yellow);
    let duration= light.duration();
    println!("{}", duration);


    //sum
    let vec: [u32; 7] = [3,23,1,31,31,1,4,];
    println!("{:?}", add_vec(&vec));

     let circle = Circle{radius:5.0};
     let square = Square{side:6.0};
     let triangle = Triangle{side:5.0 ,high:6.0};

    println!("circle is {}",shape_area(circle));
    println!("square is {}",shape_area(square));
    println!("triangle is {}",shape_area(triangle));



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

struct Circle {
    radius: f64,
}

struct Square {
    side: f64
}

struct Triangle {
    side: f64,
    high: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Triangle {
    fn  area(&self) -> f64 {
        self.side * self.high * 0.5
    }
}

fn shape_area<T: Shape>(shape: T) -> f64 {
    shape.area()
}
