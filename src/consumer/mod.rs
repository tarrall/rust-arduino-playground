
use log::{debug};
use std::str;
// use std::time::Instant;
use serde::Deserialize;

mod prometheus_metrics;

// TimeMS,TempC,RelativeHumidity,Pressure,DustConc,DustLPO
#[derive(Debug, Deserialize)]
struct DataRecord {
    time_ms: i64,
    temp_c: f64,
    humidity: u16,
    pressure: f64,
    dust_conc: f64,
    dust_lpo: f64,
    pm_1_0: i64,
    pm_2_5: i64,
    pm_10_0: i64,
}

pub fn consume_data(
    tty: String,
) -> std::io::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_path(tty)?;

    for result in reader.deserialize() {
        let record: DataRecord = result?;
        debug!("{:?}", record);

        prometheus_metrics::ARDUINO_UPTIME.arduino001.set(record.time_ms);
        prometheus_metrics::TEMPERATURE.living_room.set(record.temp_c);
        prometheus_metrics::HUMIDITY.living_room.set((record.humidity as f64) / 100.0);
        prometheus_metrics::PRESSURE.living_room.set(record.pressure);
        prometheus_metrics::DUST_CONC.living_room.set(record.dust_conc);
        prometheus_metrics::LPO_RATIO.living_room.set(record.dust_lpo);
        prometheus_metrics::PM_1_0.living_room.set(record.pm_1_0);
        prometheus_metrics::PM_2_5.living_room.set(record.pm_2_5);
        prometheus_metrics::PM_10_0.living_room.set(record.pm_10_0);
    };
    Ok(())
}
