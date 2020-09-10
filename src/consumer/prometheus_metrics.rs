use prometheus::{GaugeVec, IntGaugeVec};
use prometheus_static_metric::make_static_metric;

make_static_metric! {
    struct ArduinoUptimeGauge: IntGauge {
        "arduino" => {
            arduino001,
        },
    }

    struct TemperatureGauge: Gauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct HumidityGauge: Gauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct BarometricPressureGauge: Gauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct DustConcentrationGauge: Gauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct DustLPOGauge: Gauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct PMGauge: IntGauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct PM2_5_Gauge: IntGauge {
        "location" => {
            living_room,
            patio,
        },
    }

    struct PM10_0_Gauge: IntGauge {
        "location" => {
            living_room,
            patio,
        },
    }
}

// TimeMS,TempC,RelativeHumidity,Pressure,DustConc,DustLPO
lazy_static! {
    static ref ARDUINO_UPTIME_VEC: IntGaugeVec = register_int_gauge_vec!(
        "arduino_uptime_seconds",
        "Seconds since arduino powercycle",
        &["arduino"]
    )
    .unwrap();

    pub static ref ARDUINO_UPTIME: ArduinoUptimeGauge =
        ArduinoUptimeGauge::from(&ARDUINO_UPTIME_VEC);

    static ref TEMPERATURE_VEC: GaugeVec = register_gauge_vec!(
        "temperature_celsius",
        "Temperature in celsius",
        &["location"]
    )
    .unwrap();

    pub static ref TEMPERATURE: TemperatureGauge = TemperatureGauge::from(&TEMPERATURE_VEC);

    static ref HUMIDITY_VEC: GaugeVec = register_gauge_vec!(
        "humidity",
        "Relative humidity pct 0-1",
        &["location"]
    )
    .unwrap();

    pub static ref HUMIDITY: HumidityGauge = HumidityGauge::from(&HUMIDITY_VEC);

    static ref PRESSURE_VEC: GaugeVec = register_gauge_vec!(
        "barometric_pressure",
        "Barometric pressure in hPa",
        &["location"]
    ).unwrap();

    pub static ref PRESSURE: BarometricPressureGauge =
        BarometricPressureGauge::from(&PRESSURE_VEC);

    static ref DUST_CONC_VEC: GaugeVec = register_gauge_vec!(
        "dust_concentration",
        "Dust concentration particles per 0.01cf",
        &["location"]
    )
    .unwrap();

    pub static ref DUST_CONC: DustConcentrationGauge =
        DustConcentrationGauge::from(&DUST_CONC_VEC);

    static ref LPO_RATIO_VEC: GaugeVec = register_gauge_vec!(
        "lpo_ratio",
        "Raw low pulse occupancy percent 0-1",
        &["location"]
    ).unwrap();

    pub static ref LPO_RATIO: DustLPOGauge =
        DustLPOGauge::from(&LPO_RATIO_VEC);

    static ref PM_1_0_VEC: IntGaugeVec = register_int_gauge_vec!(
        "pms_x003_pm_1_0",
        "PM1.0 reported by PMS7003",
        &["location"]
    ).unwrap();

    pub static ref PM_1_0: PMGauge =
        PMGauge::from(&PM_1_0_VEC);

    static ref PM_2_5_VEC: IntGaugeVec = register_int_gauge_vec!(
        "pms_x003_pm_2_5",
        "PM2.5 reported by PMS7003",
        &["location"]
    ).unwrap();

    pub static ref PM_2_5: PMGauge =
        PMGauge::from(&PM_2_5_VEC);

    static ref PM_10_0_VEC: IntGaugeVec = register_int_gauge_vec!(
        "pms_x003_pm_10_0",
        "PM10.0 reported by PMS7003",
        &["location"]
    ).unwrap();

    pub static ref PM_10_0: PMGauge =
        PMGauge::from(&PM_10_0_VEC);
}
