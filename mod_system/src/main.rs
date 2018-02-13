// mod outermost {
//     pub fn middle_function() {}
//
//     fn middle_secret_function() {}
//
//     mod inside {
//         pub fn inner_function() {}
//
//         fn secret_function() {}
//     }
// }
//
// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();    //  报错,私有方法
//     outermost::inside::inner_function();    //  报错,私有模块
//     outermost::inside::secret_function();   //  报错,私有模块
// }

// use like import of js

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("hello world");
            }
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}

use a::series::of::nested_modules;
use TrafficLight::*;

fn main() {
    nested_modules();
    println!("{}", check_run(Yellow));
    println!("{}", check_run(Red));
    println!("{}", check_run(Green));
}

fn check_run(light: TrafficLight) -> bool {
    if let Red = light {
        return false;
    }
    return true;
}
