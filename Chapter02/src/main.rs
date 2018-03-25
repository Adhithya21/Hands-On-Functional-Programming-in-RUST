extern crate floating_duration;
use std::time::Instant;
use floating_duration::{TimeAsFloat, TimeFormat};
use std::time::SystemTime;
use std::{thread, time};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::process;
extern crate termion;
use termion::{clear, cursor, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use std::cmp;

fn main()
{

   //1. Store location, velocity, and acceleration state
   let mut location: f64 = 0.0; // meters
   let mut velocity: f64 = 0.0; // meters per second
   let mut acceleration: f64 = 0.0; // meters per second squared

   //2. Store motor input voltage
   let mut up_input_voltage: f64 = 0.0;
   let mut down_input_voltage: f64 = 0.0;

   //3. Store input building description and floor requests
   let mut floor_count: u64 = 0;
   let mut floor_height: f64 = 0.0; // meters
   let mut floor_requests: Vec<u64> = Vec::new();

   //4. Parse input and store as building description and floor requests
   match env::args().nth(1) {
      None => {
         let mut buffer = String::new();
         io::stdin().read_to_string(&mut buffer)
                    .expect("read_to_string failed");
        
         for (li,l) in buffer.lines().enumerate() {
            if li==0 {
               floor_count = l.parse::<u64>().unwrap();
            } else if li==1 {
               floor_height = l.parse::<f64>().unwrap();
            } else {
               floor_requests.push(l.parse::<u64>().unwrap());
            }
         }
      },
      Some(fp) => {
         let mut buffer = String::new();
         File::open(fp)
              .expect("File::open failed")
              .read_to_string(&mut buffer)
              .expect("read_to_string failed");

         for (li,l) in buffer.lines().enumerate() {
            if li==0 {
               floor_count = l.parse::<u64>().unwrap();
            } else if li==1 {
               floor_height = l.parse::<f64>().unwrap();
            } else {
               floor_requests.push(l.parse::<u64>().unwrap());
            }
         }
      }
   }

   //5. Loop while there are remaining floor requests
   let mut prev_loop_time = Instant::now();
   let termsize = termion::terminal_size().ok();
   let termwidth = termsize.map(|(w,_)| w-2).expect("termwidth") as u64;
   let termheight = termsize.map(|(_,h)| h-2).expect("termheight") as u64;
   let mut stdout = io::stdout().into_raw_mode().unwrap();

   while floor_requests.len() > 0
   {
      //5.1. Update location, velocity, and acceleration
      let now = Instant::now();
      let dt = now.duration_since(prev_loop_time)
                  .as_fractional_secs();
      prev_loop_time = now;

      location = location + velocity * dt;
      velocity = velocity + acceleration * dt;
      acceleration = {
         let F = (up_input_voltage - down_input_voltage) * 8.0;
         let m = 1200000.0;
         -9.8 + F/m
      };

      //5.2. If next floor request in queue is satisfied, then remove from queue
      let next_floor = floor_requests[0];
      if (location - (next_floor as f64)*floor_height).abs() < 0.01 &&
         velocity.abs() < 0.01
      {
         velocity = 0.0;
         floor_requests.remove(0);
      }

      //5.3. Adjust motor control to process next floor request

      //5.4. Print realtime statistics
      print!("{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Hide);
      let carriage_floor = (location / floor_height).floor() as u64;
      let carriage_floor = cmp::max(carriage_floor, 0);
      let carriage_floor = cmp::min(carriage_floor, floor_count-1);
      let mut terminal_buffer = vec![' ' as u8; (termwidth*termheight) as usize];
      for ty in 0..floor_count-1
      {
         terminal_buffer[ (ty*termwidth + 0) as usize ] = '[' as u8;
         terminal_buffer[ (ty*termwidth + 1) as usize ] =
            if   (ty as u64)==((floor_count-1)-carriage_floor) { 'X' as u8 }
            else { ' ' as u8 };
         terminal_buffer[ (ty*termwidth + 2) as usize ] = ']' as u8;
         terminal_buffer[ (ty*termwidth + termwidth-2) as usize ] = '\r' as u8;
         terminal_buffer[ (ty*termwidth + termwidth-1) as usize ] = '\n' as u8;
      }
      let stats = vec![
         format!("Carriage at floor {}", carriage_floor+1),
         format!("Location          {:.06}", location),
         format!("Velocity          {:.06}", velocity),
         format!("Acceleration      {:.06}", acceleration),
         format!("Voltage [up-down] {:.06}", up_input_voltage-down_input_voltage),
      ];
      for sy in 0..stats.len()
      {
         for (sx,sc) in stats[sy].chars().enumerate()
         {
            terminal_buffer[ sy*(termwidth as usize) + 6 + sx ] = sc as u8;
         }
      }
      write!(stdout, "{}", String::from_utf8(terminal_buffer).ok().unwrap());
      stdout.flush().unwrap();

      thread::sleep(time::Duration::from_millis(10));
   }
   write!(stdout, "{}{}", cursor::Goto(1, 1), cursor::Show).unwrap();
   stdout.flush().unwrap();

   //6. Print summary
   println!("main");

}
