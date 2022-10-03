use chrono::{DateTime, Utc};
use log::debug;

#[cfg(feature = "influxdb")]
use influxdb::InfluxDbWriteable;

//enum IBSEventType {
//    0, // None
//    1, // Button pressed
//    2, // Moving
//    4  // Hall Sensor activated
//    5, // Hall and Button activated
//    10, // object detected
//    20, // proximity detected
//    40, // external input triggered
//}

#[derive(Debug)]
#[cfg_attr(feature = "influxdb", derive(InfluxDbWriteable))]
pub struct SensorReading {
    pub time: DateTime<Utc>,
    #[cfg_attr(feature = "influxdb", influxdb(tag))]
    pub sensor_type: String,
    pub temperature: Option<f32>,
    pub ext_temperature: Option<f32>,
    pub humidity: Option<u32>,
    pub distance: Option<u32>,
    pub battery: f32,
    #[cfg_attr(feature = "influxdb", influxdb(tag))]
    pub userdata: u32,
    pub event_status: u8,
}

impl PartialEq for SensorReading {
    fn eq(&self, other: &Self) -> bool {
        self.temperature == other.temperature
            && self.humidity == other.humidity
            && self.distance == other.distance
            && self.battery == other.battery
            && self.userdata == other.userdata
            && self.sensor_type == other.sensor_type
            && self.event_status == other.event_status
    }
}

pub fn parse_data(data: &[u8]) -> Option<SensorReading> {
    // match the Sensor Subtype byte from https://www.ingics.com/doc/Beacon/BC0034_iBS_Sensor_Beacon_Payload.pdf
    match data[11] {
        0x10 => {
            let sensor_type = "iBS03".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        //0x12 => debug!("Sensor type: {:?}, Data: {:x?}", "iBS03P", data),
        0x13 => {
            let sensor_type = "iBS03R".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let distance = convert_byte(data[7], data[8]);
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {distance}mm, {battery}V");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: Some(distance),
                battery,
                userdata,
                event_status,
            })
        }
        0x14 => {
            let sensor_type = "iBS03T_RH".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let event_status = data[4];
            let temperature = convert_byte(data[5], data[6]) as f32 / 100.0;
            let humidity = convert_byte(data[7], data[8]);
            let userdata = convert_byte(data[9], data[10]);

            debug!("Sensor {sensor_type} {userdata}: {temperature}°C, {humidity}%, {battery}V");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: Some(temperature),
                ext_temperature: None,
                humidity: Some(humidity),
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        //0x15 => debug!("Sensor type: {:?}, Data: {:x?}", "iBS03T)", data),
        0x16 => {
            let sensor_type = "iBS03G".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        0x17 => {
            let sensor_type = "iBS03TP".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let temperature = convert_byte(data[5], data[6]) as f32 / 100.0;
            let ext_temperature = convert_byte(data[7], data[8]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {temperature}°C, Ext: {ext_temperature}°C");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: Some(temperature),
                ext_temperature: Some(ext_temperature),
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        //0x18 => debug!("Sensor type: {:?}, Data: {:x?}", "iBS04i", data),
        //0x19 => debug!("Sensor type: {:?}, Data: {:x?}", "iBS04", data),
        0x30 => {
            let sensor_type = "iBS05".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        0x31 => {
            let sensor_type = "iBS05H".to_string();
            //debug!("Sensor type: {:?}, Data: {:x?}", sensor_type, data);

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        0x32 => {
            let sensor_type = "iBS05T".to_string();

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let temperature = convert_byte(data[5], data[6]) as f32 / 100.0;
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: Some(temperature),
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        0x33 => {
            let sensor_type = "iBS05G".to_string();

            let battery = convert_byte(data[2], data[3]) as f32 / 100.0;
            let userdata = convert_byte(data[9], data[10]);
            let event_status = data[4];

            debug!("Sensor {sensor_type} {userdata}: {event_status}");

            Some(SensorReading {
                sensor_type,
                time: Utc::now(),
                temperature: None,
                ext_temperature: None,
                humidity: None,
                distance: None,
                battery,
                userdata,
                event_status,
            })
        }
        //0x40 => debug!("Sensor type: {:?}, Data: {:x?}", "iBS06", data),
        _ => {
            debug!("Unimplemented sensor type");
            None
        }
    }
}

fn convert_byte(a: u8, b: u8) -> u32 {
    (u32::from(b) << 8) | u32::from(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! model_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let source_data = hex_to_bytes(input).unwrap();
                let parsed_data = parse_data(&source_data).unwrap();

                assert_eq!(expected, parsed_data);
            }
        )*
        }
    }

    fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
        if s.len() % 2 == 0 {
            (0..s.len())
                .step_by(2)
                .map(|i| {
                    s.get(i..i + 2)
                        .and_then(|sub| u8::from_str_radix(sub, 16).ok())
                })
                .collect()
        } else {
            None
        }
    }

    #[test]
    fn test_convert_byte() {
        assert_eq!(convert_byte(0x19, 0x01), 0x0119);
    }

    model_tests! {
        // iBS03R, Batt: 0x0117 (2.79V), Range: 0x0119 (281 mm)
        parser_ibs03r: ("83BC170100AAAA19010000130B0600", SensorReading {
            sensor_type: "iBS03R".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: Some(281),
            battery: 2.79,
            userdata: 0,
            event_status: 00,
        }),
        // iBS03T, Batt: 0x014A (3.3V), Temp: 0x0AA1 (27.21), RH: 64%
        parser_ibs03t_rh: ("83BC4A0100A10A4000000014000000", SensorReading {
            sensor_type: "iBS03T_RH".to_string(),
            time: Utc::now(),
            temperature: Some(27.21),
            ext_temperature: None,
            humidity: Some(64),
            distance: None,
            battery: 3.3,
            userdata: 0,
            event_status: 00,
        }),
        // iBS03G, Moving, Batt: 0x0129 (2.97V)
        parser_ibs3g: ("83BC290102FFFFFFFF000016000000", SensorReading {
            sensor_type: "iBS03G".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 2.97,
            userdata: 0,
            event_status: 02,
        }),
        // iBS03, Hall sensor activated, Batt: 0x0129 (2.97V)
        parser_ibs03: ("83BC290104FFFFFFFF000010000000", SensorReading {
            sensor_type: "iBS03".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 2.97,
            userdata: 0,
            event_status: 04,
        }),
        // iBS03TP, Batt: 0x0129 (2.97V), Temp: 0x0944 (23.72), Probe Temp: 0x0943 (23.71)
        parser_ibs03tp: ("83BC29010044094309000017030000", SensorReading {
            sensor_type: "iBS03TP".to_string(),
            time: Utc::now(),
            temperature: Some(23.72),
            ext_temperature: Some(23.71),
            humidity: None,
            distance: None,
            battery: 2.97,
            userdata: 0,
            event_status: 00,
        }),

        // iBS05T, Batt: 0x0149 (3.29V), Temp: 0x0AF0 (28.00 deg C)
        parser_ibs05t: ("83BC490100F00AFFFF000032110400", SensorReading {
            sensor_type: "iBS05T".to_string(),
            time: Utc::now(),
            temperature: Some(28.00),
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 3.29,
            userdata: 0,
            event_status: 00,
        }),
        // iBS05G, Moving
        parser_ibs05g: ("83BC4A0102AAAAFFFF000033110400", SensorReading {
            sensor_type: "iBS05G".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 3.3,
            userdata: 0,
            event_status: 02,
        }),
        // iBS05H, Hall sensor activated
        parser_ibs05h: ("83BC4B0104AAAAFFFF000031110400", SensorReading {
            sensor_type: "iBS05H".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 3.31,
            userdata: 0,
            event_status: 04,
        }),
        // iBS05, Button pressed
        parser_ibs05: ("83BC4B0101AAAAFFFF000030110400", SensorReading {
            sensor_type: "iBS05".to_string(),
            time: Utc::now(),
            temperature: None,
            ext_temperature: None,
            humidity: None,
            distance: None,
            battery: 3.31,
            userdata: 0,
            event_status: 01,
        }),
    }
}
