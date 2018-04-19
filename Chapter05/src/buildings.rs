use elevator_drivers::{ElevatorDriver, ElevatorDriver1, ElevatorDriver2, ElevatorDriver3};
use motor_controllers::{MotorInput, MotorController, newMotorController1, newMotorController2, newMotorController3};

pub trait Building
{
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>;
   fn get_motor_controller(&self) -> Box<MotorController>;
   fn get_floor_heights(&self) -> Vec<f64>;
}

struct Building1;
impl Building for Building1 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver1)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController1()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![8.0, 4.0, 4.0, 4.0, 4.0]
   }
}

struct Building2;
impl Building for Building2 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver2)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController2()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0]
   }
}

struct Building3;
impl Building for Building3 {
   fn get_elevator_driver(&self) -> Box<ElevatorDriver>
   {
      Box::new(ElevatorDriver3)
   }
   fn get_motor_controller(&self) -> Box<MotorController>
   {
      newMotorController3()
   }
   fn get_floor_heights(&self) -> Vec<f64>
   {
      vec![6.0, 4.0, 4.0, 4.0]
   }
}
