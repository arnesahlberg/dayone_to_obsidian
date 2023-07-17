use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Weather {
    #[serde(rename = "moonPhaseCode")]
    pub moon_phase_code: Option<String>,
    #[serde(rename = "sunsetDate")]
    pub sunset_date: Option<String>,
    #[serde(rename = "weatherServiceName")]
    pub weather_service_name: Option<String>,
    #[serde(rename = "weatherCode")]
    pub weather_code: Option<String>,
    #[serde(rename = "temperatureCelsius")]
    pub temperature_celsius: Option<f32>,
    #[serde(rename = "windBearing")]
    pub wind_bearing: Option<f32>,
    #[serde(rename = "sunriseDate")]
    pub sunrise_date: Option<String>,
    #[serde(rename = "conditionsDescription")]
    pub conditions_description: Option<String>,
    #[serde(rename = "pressureMB")]
    pub pressure_mb: Option<f32>,
    #[serde(rename = "moonPhase")]
    pub moon_phase: Option<f64>,
    #[serde(rename = "visibilityKM")]
    pub visibility_km: Option<f64>,
    #[serde(rename = "relativeHumidity")]
    pub relative_humidity: Option<f32>,
    #[serde(rename = "windSpeedKPH")]
    pub wind_speed_kph: Option<f32>,
    #[serde(rename = "windChillCelsius")]
    pub wind_chill_celsius: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub region: Region,
    #[serde(rename = "localityName")]
    pub locality_name: Option<String>,
    pub country: Option<String>,
    #[serde(rename = "timeZoneName")]
    pub time_zone_name: Option<String>,
    #[serde(rename = "administrativeArea")]
    pub administrative_area: Option<String>,
    #[serde(rename = "placeName")]
    pub place_name: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub center: Center,
    pub radius: f32,
    pub identifier: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Center {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct Photo {
    #[serde(rename = "fileSize")]
    pub file_size: Option<i32>,
    #[serde(rename = "lensModel")]
    pub lens_model: Option<String>,
    #[serde(rename = "orderInEntry")]
    pub order_in_entry: Option<i32>,
    #[serde(rename = "creationDevice")]
    pub creation_device: Option<String>,
    pub duration: Option<f32>,
    pub favorite: Option<bool>,
    #[serde(rename = "appleCloudIdentifier")]
    pub apple_cloud_identifier: Option<String>,
    #[serde(rename = "cameraMake")]
    pub camera_make: Option<String>,
    #[serde(rename = "type")]
    pub photo_type: Option<String>,
    #[serde(rename = "lensMake")]
    pub lens_make: Option<String>,
    pub identifier: String,
    pub date: Option<String>,
    #[serde(rename = "exposureBiasValue")]
    pub exposure_bias_value: Option<f32>,
    pub filename: Option<String>,
    pub height: i32,
    #[serde(rename = "fnumber")]
    pub f_number: Option<String>,
    pub width: i32,
    pub md5: Option<String>,
    #[serde(rename = "appleLocalIdentifier")]
    pub apple_local_identifier: Option<String>,
    #[serde(rename = "isSketch")]
    pub is_sketch: Option<bool>,
    #[serde(rename = "focalLength")]
    pub focal_length: Option<String>,
    #[serde(rename = "cameraModel")]
    pub camera_model: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    pub location: Option<Location>,
    #[serde(rename = "creationDate")]
    pub creation_date: String,
    #[serde(rename = "modifiedDate")]
    pub modified_date: String,
    #[serde(rename = "isAllDay")]
    pub is_all_day: Option<bool>,
    pub duration: Option<i32>,
    #[serde(rename = "editingTime")]
    pub editing_time: Option<f32>,
    pub weather: Option<Weather>,
    #[serde(rename = "creationOSVersion")]
    pub creation_os_version: Option<String>,
    #[serde(rename = "creationDevice")]
    pub creation_device: Option<String>,
    #[serde(rename = "creationDeviceType")]
    pub creation_device_type: Option<String>,
    pub photos : Option<Vec<Photo>>,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    pub uuid: String,
    pub starred: bool,
    pub text: String,
    #[serde(rename = "richText")]
    pub rich_text : Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub version : String
}

#[derive(Debug, Deserialize)]
pub struct Journal {
    pub entries: Vec<JournalEntry>,
    pub metadata: Metadata,
}