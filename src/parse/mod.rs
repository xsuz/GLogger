mod cobs;

mod servo_data;
mod alt_data;
mod imu_data;
mod parser;

pub use cobs::decode_cobs;
pub use servo_data::ServoData;
pub use alt_data::AltData;
pub use imu_data::IMUData;
pub use parser::Parser;

pub trait Data{
    fn parse(data: &Vec<u8>) -> Self;
    fn draw(&self,ui:&mut eframe::egui::Ui);
}
