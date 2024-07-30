use std::time::{Duration, Instant};

use crate::models::simulation_entry::SimulationEntry;

pub fn time_loop(target_loop_duration: Duration, loop_start: Instant) {
    let time_since_loop_start = loop_start.elapsed();
    if time_since_loop_start < target_loop_duration {
        std::thread::sleep(target_loop_duration - time_since_loop_start);
    }
}

pub fn parse_simlation_csv(data: &str) -> Result<Vec<SimulationEntry>, String> {
    let mut sim_data: Vec<SimulationEntry> = Vec::<SimulationEntry>::new();
    for line in data.split("\n") {
        if line.len() == 0 { // If empty, skip
            continue;            
        }
        
        let first_char = line.chars().nth(0 as usize);
        if let Some(first_char) = first_char {
            if first_char  == '#' { // If comment, skip
                continue;
            }
        } else {
            return Err("Error parsing first char".to_string());
        }

        let mut entry: SimulationEntry = SimulationEntry::null();
        let fields: Vec<&str> = line.split(",").collect();
        
        let time = fields[0].parse::<f64>();
        if let Err(e) = time{
            return Err(format!("Error parsing time: {}", e));
        }
        entry.time = time.unwrap();
        
        let altitude = fields[1].parse::<f64>();
        if let Err(e) = altitude{
            return Err(format!("Error parsing altitude: {}", e));
        }
        entry.altitude = altitude.unwrap();

        let vertical_velocity = fields[2].parse::<f64>();
        if let Err(e) = vertical_velocity{
            return Err(format!("Error parsing vertical_velocity: {}", e));
        }
        entry.vertical_velocity = vertical_velocity.unwrap();

        let vertical_acceleration = fields[3].parse::<f64>();
        if let Err(e) = vertical_acceleration{
            return Err(format!("Error parsing vertical_acceleration: {}", e));
        }
        entry.vertical_acceleration = vertical_acceleration.unwrap();

        sim_data.push(entry);
    }
    return Ok(sim_data);
}