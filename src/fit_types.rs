/* This file was auto-generated */

use std::fmt;

// FIT epoch starts 1989-12-31T00:00:00, 631065600 second after unix timestamp
pub static FIT_EPOCH: i64 = 631065600i64;

#[derive(Debug, Clone)]
pub enum FitType {
    Enum(u8),
    Sint8(i8),
    Uint8(u8),
    Sint16(i16),
    Uint16(u16),
    Sint32(i32),
    Uint32(u32),
    String(String),
    Float32(f32),
    Float64(f64),
    Uint8z(u8),
    Uint16z(u16),
    Uint32z(u32),
    Bytes(Vec<u8>),
    Sint64(i64),
    Uint64(u64),
    Uint64z(u64),
}

impl FitType {
    pub fn is_invalid(&self) -> bool {
        /* See Table 4-6 in D00001275 Flexible & Interoperable Data Transfer (FIT) Protocol Rev 2.3.pdf */
        match *self {
            FitType::Enum(ref v) => *v == u8::max_value(),
            FitType::Sint8(ref v) => *v == i8::max_value(),
            FitType::Uint8(ref v) => *v == u8::max_value(),
            FitType::Sint16(ref v) => *v == i16::max_value(),
            FitType::Uint16(ref v) => *v == u16::max_value(),
            FitType::Sint32(ref v) => *v == i32::max_value(),
            FitType::Uint32(ref v) => *v == u32::max_value(),
            FitType::String(_) => false,
            FitType::Float32(ref v) => v.is_nan(),
            FitType::Float64(ref v) => v.is_nan(),
            FitType::Uint8z(ref v) => *v == 0u8,
            FitType::Uint16z(ref v) => *v == 0u16,
            FitType::Uint32z(ref v) => *v == 0u32,
            FitType::Bytes(_) => false,
            FitType::Sint64(ref v) => *v == i64::max_value(),
            FitType::Uint64(ref v) => *v == u64::max_value(),
            FitType::Uint64z(ref v) => *v == 0u64,
        }
    }
}

impl fmt::Display for FitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_invalid() {
            return write!(f, "Invalid Value")
        }
        match *self {
            FitType::Enum(ref v) => write!(f, "{}", v),
            FitType::Sint8(ref v) => write!(f, "{}", v),
            FitType::Uint8(ref v) => write!(f, "{}", v),
            FitType::Sint16(ref v) => write!(f, "{}", v),
            FitType::Uint16(ref v) => write!(f, "{}", v),
            FitType::Sint32(ref v) => write!(f, "{}", v),
            FitType::Uint32(ref v) => write!(f, "{}", v),
            FitType::String(ref v) => write!(f, "{}", v),
            FitType::Float32(ref v) => write!(f, "{}", v),
            FitType::Float64(ref v) => write!(f, "{}", v),
            FitType::Uint8z(ref v) => write!(f, "{}", v),
            FitType::Uint16z(ref v) => write!(f, "{}", v),
            FitType::Uint32z(ref v) => write!(f, "{}", v),
            FitType::Bytes(ref v) => {
                for b in v { write!(f, "{:02}", b)?; }
                Ok(())
            },
            FitType::Sint64(ref v) => write!(f, "{}", v),
            FitType::Uint64(ref v) => write!(f, "{}", v),
            FitType::Uint64z(ref v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FitValue {
    pub index: u8,
    pub value: FitType,
}

pub trait IntoField<T> where Self: Sized {
    fn into_field(value: T) -> Option<Self>;
}

pub trait Message {
    fn id(&self) -> u16;
}

/* name: camera_orientation_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum CameraOrientationTypeField {
  CameraOrientation0 = 0,
  CameraOrientation90 = 1,
  CameraOrientation180 = 2,
  CameraOrientation270 = 3,
}

impl From<CameraOrientationTypeField> for u8 {
  fn from(source: CameraOrientationTypeField) -> u8 {
    match source {
      CameraOrientationTypeField::CameraOrientation0 => 0,
      CameraOrientationTypeField::CameraOrientation90 => 1,
      CameraOrientationTypeField::CameraOrientation180 => 2,
      CameraOrientationTypeField::CameraOrientation270 => 3,
    }
  }
}

impl IntoField<u8> for CameraOrientationTypeField {
  fn into_field(value: u8) -> Option<CameraOrientationTypeField> {
    match value {
      0 => Some(CameraOrientationTypeField::CameraOrientation0),
      1 => Some(CameraOrientationTypeField::CameraOrientation90),
      2 => Some(CameraOrientationTypeField::CameraOrientation180),
      3 => Some(CameraOrientationTypeField::CameraOrientation270),
      _ => None,
    }
  }
}

/* name: segment_selection_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SegmentSelectionTypeField {
  Starred = 0,
  Suggested = 1,
}

impl From<SegmentSelectionTypeField> for u8 {
  fn from(source: SegmentSelectionTypeField) -> u8 {
    match source {
      SegmentSelectionTypeField::Starred => 0,
      SegmentSelectionTypeField::Suggested => 1,
    }
  }
}

impl IntoField<u8> for SegmentSelectionTypeField {
  fn into_field(value: u8) -> Option<SegmentSelectionTypeField> {
    match value {
      0 => Some(SegmentSelectionTypeField::Starred),
      1 => Some(SegmentSelectionTypeField::Suggested),
      _ => None,
    }
  }
}

/* name: battery_status type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BatteryStatusField {
  New = 1,
  Good = 2,
  Ok = 3,
  Low = 4,
  Critical = 5,
  Charging = 6,
  Unknown = 7,
}

impl From<BatteryStatusField> for u8 {
  fn from(source: BatteryStatusField) -> u8 {
    match source {
      BatteryStatusField::New => 1,
      BatteryStatusField::Good => 2,
      BatteryStatusField::Ok => 3,
      BatteryStatusField::Low => 4,
      BatteryStatusField::Critical => 5,
      BatteryStatusField::Charging => 6,
      BatteryStatusField::Unknown => 7,
    }
  }
}

impl IntoField<u8> for BatteryStatusField {
  fn into_field(value: u8) -> Option<BatteryStatusField> {
    match value {
      1 => Some(BatteryStatusField::New),
      2 => Some(BatteryStatusField::Good),
      3 => Some(BatteryStatusField::Ok),
      4 => Some(BatteryStatusField::Low),
      5 => Some(BatteryStatusField::Critical),
      6 => Some(BatteryStatusField::Charging),
      7 => Some(BatteryStatusField::Unknown),
      _ => None,
    }
  }
}

/* name: side type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SideField {
  Right = 0,
  Left = 1,
}

impl From<SideField> for u8 {
  fn from(source: SideField) -> u8 {
    match source {
      SideField::Right => 0,
      SideField::Left => 1,
    }
  }
}

impl IntoField<u8> for SideField {
  fn into_field(value: u8) -> Option<SideField> {
    match value {
      0 => Some(SideField::Right),
      1 => Some(SideField::Left),
      _ => None,
    }
  }
}

/* name: source_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SourceTypeField {
  Ant = 0,
  Antplus = 1,
  Bluetooth = 2,
  BluetoothLowEnergy = 3,
  Wifi = 4,
  Local = 5,
}

impl From<SourceTypeField> for u8 {
  fn from(source: SourceTypeField) -> u8 {
    match source {
      SourceTypeField::Ant => 0,
      SourceTypeField::Antplus => 1,
      SourceTypeField::Bluetooth => 2,
      SourceTypeField::BluetoothLowEnergy => 3,
      SourceTypeField::Wifi => 4,
      SourceTypeField::Local => 5,
    }
  }
}

impl IntoField<u8> for SourceTypeField {
  fn into_field(value: u8) -> Option<SourceTypeField> {
    match value {
      0 => Some(SourceTypeField::Ant),
      1 => Some(SourceTypeField::Antplus),
      2 => Some(SourceTypeField::Bluetooth),
      3 => Some(SourceTypeField::BluetoothLowEnergy),
      4 => Some(SourceTypeField::Wifi),
      5 => Some(SourceTypeField::Local),
      _ => None,
    }
  }
}

/* name: weight type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WeightField {
  Calculating = 65534,
}

impl From<WeightField> for u16 {
  fn from(source: WeightField) -> u16 {
    match source {
      WeightField::Calculating => 65534,
    }
  }
}

impl IntoField<u16> for WeightField {
  fn into_field(value: u16) -> Option<WeightField> {
    match value {
      65534 => Some(WeightField::Calculating),
      _ => None,
    }
  }
}

/* name: activity_subtype type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ActivitySubtypeField {
  Generic = 0,
  Treadmill = 1,
  Street = 2,
  Trail = 3,
  Track = 4,
  Spin = 5,
  IndoorCycling = 6,
  Road = 7,
  Mountain = 8,
  Downhill = 9,
  Recumbent = 10,
  Cyclocross = 11,
  HandCycling = 12,
  TrackCycling = 13,
  IndoorRowing = 14,
  Elliptical = 15,
  StairClimbing = 16,
  LapSwimming = 17,
  OpenWater = 18,
  All = 254,
}

impl From<ActivitySubtypeField> for u8 {
  fn from(source: ActivitySubtypeField) -> u8 {
    match source {
      ActivitySubtypeField::Generic => 0,
      ActivitySubtypeField::Treadmill => 1,
      ActivitySubtypeField::Street => 2,
      ActivitySubtypeField::Trail => 3,
      ActivitySubtypeField::Track => 4,
      ActivitySubtypeField::Spin => 5,
      ActivitySubtypeField::IndoorCycling => 6,
      ActivitySubtypeField::Road => 7,
      ActivitySubtypeField::Mountain => 8,
      ActivitySubtypeField::Downhill => 9,
      ActivitySubtypeField::Recumbent => 10,
      ActivitySubtypeField::Cyclocross => 11,
      ActivitySubtypeField::HandCycling => 12,
      ActivitySubtypeField::TrackCycling => 13,
      ActivitySubtypeField::IndoorRowing => 14,
      ActivitySubtypeField::Elliptical => 15,
      ActivitySubtypeField::StairClimbing => 16,
      ActivitySubtypeField::LapSwimming => 17,
      ActivitySubtypeField::OpenWater => 18,
      ActivitySubtypeField::All => 254,
    }
  }
}

impl IntoField<u8> for ActivitySubtypeField {
  fn into_field(value: u8) -> Option<ActivitySubtypeField> {
    match value {
      0 => Some(ActivitySubtypeField::Generic),
      1 => Some(ActivitySubtypeField::Treadmill),
      2 => Some(ActivitySubtypeField::Street),
      3 => Some(ActivitySubtypeField::Trail),
      4 => Some(ActivitySubtypeField::Track),
      5 => Some(ActivitySubtypeField::Spin),
      6 => Some(ActivitySubtypeField::IndoorCycling),
      7 => Some(ActivitySubtypeField::Road),
      8 => Some(ActivitySubtypeField::Mountain),
      9 => Some(ActivitySubtypeField::Downhill),
      10 => Some(ActivitySubtypeField::Recumbent),
      11 => Some(ActivitySubtypeField::Cyclocross),
      12 => Some(ActivitySubtypeField::HandCycling),
      13 => Some(ActivitySubtypeField::TrackCycling),
      14 => Some(ActivitySubtypeField::IndoorRowing),
      15 => Some(ActivitySubtypeField::Elliptical),
      16 => Some(ActivitySubtypeField::StairClimbing),
      17 => Some(ActivitySubtypeField::LapSwimming),
      18 => Some(ActivitySubtypeField::OpenWater),
      254 => Some(ActivitySubtypeField::All),
      _ => None,
    }
  }
}

/* name: schedule type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ScheduleField {
  Workout = 0,
  Course = 1,
}

impl From<ScheduleField> for u8 {
  fn from(source: ScheduleField) -> u8 {
    match source {
      ScheduleField::Workout => 0,
      ScheduleField::Course => 1,
    }
  }
}

impl IntoField<u8> for ScheduleField {
  fn into_field(value: u8) -> Option<ScheduleField> {
    match value {
      0 => Some(ScheduleField::Workout),
      1 => Some(ScheduleField::Course),
      _ => None,
    }
  }
}

/* name: exd_layout type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ExdLayoutField {
  FullScreen = 0,
  HalfVertical = 1,
  HalfHorizontal = 2,
  HalfVerticalRightSplit = 3,
  HalfHorizontalBottomSplit = 4,
  FullQuarterSplit = 5,
  HalfVerticalLeftSplit = 6,
  HalfHorizontalTopSplit = 7,
}

impl From<ExdLayoutField> for u8 {
  fn from(source: ExdLayoutField) -> u8 {
    match source {
      ExdLayoutField::FullScreen => 0,
      ExdLayoutField::HalfVertical => 1,
      ExdLayoutField::HalfHorizontal => 2,
      ExdLayoutField::HalfVerticalRightSplit => 3,
      ExdLayoutField::HalfHorizontalBottomSplit => 4,
      ExdLayoutField::FullQuarterSplit => 5,
      ExdLayoutField::HalfVerticalLeftSplit => 6,
      ExdLayoutField::HalfHorizontalTopSplit => 7,
    }
  }
}

impl IntoField<u8> for ExdLayoutField {
  fn into_field(value: u8) -> Option<ExdLayoutField> {
    match value {
      0 => Some(ExdLayoutField::FullScreen),
      1 => Some(ExdLayoutField::HalfVertical),
      2 => Some(ExdLayoutField::HalfHorizontal),
      3 => Some(ExdLayoutField::HalfVerticalRightSplit),
      4 => Some(ExdLayoutField::HalfHorizontalBottomSplit),
      5 => Some(ExdLayoutField::FullQuarterSplit),
      6 => Some(ExdLayoutField::HalfVerticalLeftSplit),
      7 => Some(ExdLayoutField::HalfHorizontalTopSplit),
      _ => None,
    }
  }
}

/* name: language_bits_0 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageBits0Field {
  English = 1,
  French = 2,
  Italian = 4,
  German = 8,
  Spanish = 16,
  Croatian = 32,
  Czech = 64,
  Danish = 128,
}

impl From<LanguageBits0Field> for u8 {
  fn from(source: LanguageBits0Field) -> u8 {
    match source {
      LanguageBits0Field::English => 1,
      LanguageBits0Field::French => 2,
      LanguageBits0Field::Italian => 4,
      LanguageBits0Field::German => 8,
      LanguageBits0Field::Spanish => 16,
      LanguageBits0Field::Croatian => 32,
      LanguageBits0Field::Czech => 64,
      LanguageBits0Field::Danish => 128,
    }
  }
}

impl IntoField<u8> for LanguageBits0Field {
  fn into_field(value: u8) -> Option<LanguageBits0Field> {
    match value {
      1 => Some(LanguageBits0Field::English),
      2 => Some(LanguageBits0Field::French),
      4 => Some(LanguageBits0Field::Italian),
      8 => Some(LanguageBits0Field::German),
      16 => Some(LanguageBits0Field::Spanish),
      32 => Some(LanguageBits0Field::Croatian),
      64 => Some(LanguageBits0Field::Czech),
      128 => Some(LanguageBits0Field::Danish),
      _ => None,
    }
  }
}

/* name: auto_activity_detect type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AutoActivityDetectField {
  None = 0,
  Running = 1,
  Cycling = 2,
  Swimming = 4,
  Walking = 8,
  Elliptical = 32,
  Sedentary = 1024,
}

impl From<AutoActivityDetectField> for u32 {
  fn from(source: AutoActivityDetectField) -> u32 {
    match source {
      AutoActivityDetectField::None => 0,
      AutoActivityDetectField::Running => 1,
      AutoActivityDetectField::Cycling => 2,
      AutoActivityDetectField::Swimming => 4,
      AutoActivityDetectField::Walking => 8,
      AutoActivityDetectField::Elliptical => 32,
      AutoActivityDetectField::Sedentary => 1024,
    }
  }
}

impl IntoField<u32> for AutoActivityDetectField {
  fn into_field(value: u32) -> Option<AutoActivityDetectField> {
    match value {
      0 => Some(AutoActivityDetectField::None),
      1 => Some(AutoActivityDetectField::Running),
      2 => Some(AutoActivityDetectField::Cycling),
      4 => Some(AutoActivityDetectField::Swimming),
      8 => Some(AutoActivityDetectField::Walking),
      32 => Some(AutoActivityDetectField::Elliptical),
      1024 => Some(AutoActivityDetectField::Sedentary),
      _ => None,
    }
  }
}

/* name: body_location type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BodyLocationField {
  LeftLeg = 0,
  LeftCalf = 1,
  LeftShin = 2,
  LeftHamstring = 3,
  LeftQuad = 4,
  LeftGlute = 5,
  RightLeg = 6,
  RightCalf = 7,
  RightShin = 8,
  RightHamstring = 9,
  RightQuad = 10,
  RightGlute = 11,
  TorsoBack = 12,
  LeftLowerBack = 13,
  LeftUpperBack = 14,
  RightLowerBack = 15,
  RightUpperBack = 16,
  TorsoFront = 17,
  LeftAbdomen = 18,
  LeftChest = 19,
  RightAbdomen = 20,
  RightChest = 21,
  LeftArm = 22,
  LeftShoulder = 23,
  LeftBicep = 24,
  LeftTricep = 25,
  LeftBrachioradialis = 26,
  LeftForearmExtensors = 27,
  RightArm = 28,
  RightShoulder = 29,
  RightBicep = 30,
  RightTricep = 31,
  RightBrachioradialis = 32,
  RightForearmExtensors = 33,
  Neck = 34,
  Throat = 35,
  WaistMidBack = 36,
  WaistFront = 37,
  WaistLeft = 38,
  WaistRight = 39,
}

impl From<BodyLocationField> for u8 {
  fn from(source: BodyLocationField) -> u8 {
    match source {
      BodyLocationField::LeftLeg => 0,
      BodyLocationField::LeftCalf => 1,
      BodyLocationField::LeftShin => 2,
      BodyLocationField::LeftHamstring => 3,
      BodyLocationField::LeftQuad => 4,
      BodyLocationField::LeftGlute => 5,
      BodyLocationField::RightLeg => 6,
      BodyLocationField::RightCalf => 7,
      BodyLocationField::RightShin => 8,
      BodyLocationField::RightHamstring => 9,
      BodyLocationField::RightQuad => 10,
      BodyLocationField::RightGlute => 11,
      BodyLocationField::TorsoBack => 12,
      BodyLocationField::LeftLowerBack => 13,
      BodyLocationField::LeftUpperBack => 14,
      BodyLocationField::RightLowerBack => 15,
      BodyLocationField::RightUpperBack => 16,
      BodyLocationField::TorsoFront => 17,
      BodyLocationField::LeftAbdomen => 18,
      BodyLocationField::LeftChest => 19,
      BodyLocationField::RightAbdomen => 20,
      BodyLocationField::RightChest => 21,
      BodyLocationField::LeftArm => 22,
      BodyLocationField::LeftShoulder => 23,
      BodyLocationField::LeftBicep => 24,
      BodyLocationField::LeftTricep => 25,
      BodyLocationField::LeftBrachioradialis => 26,
      BodyLocationField::LeftForearmExtensors => 27,
      BodyLocationField::RightArm => 28,
      BodyLocationField::RightShoulder => 29,
      BodyLocationField::RightBicep => 30,
      BodyLocationField::RightTricep => 31,
      BodyLocationField::RightBrachioradialis => 32,
      BodyLocationField::RightForearmExtensors => 33,
      BodyLocationField::Neck => 34,
      BodyLocationField::Throat => 35,
      BodyLocationField::WaistMidBack => 36,
      BodyLocationField::WaistFront => 37,
      BodyLocationField::WaistLeft => 38,
      BodyLocationField::WaistRight => 39,
    }
  }
}

impl IntoField<u8> for BodyLocationField {
  fn into_field(value: u8) -> Option<BodyLocationField> {
    match value {
      0 => Some(BodyLocationField::LeftLeg),
      1 => Some(BodyLocationField::LeftCalf),
      2 => Some(BodyLocationField::LeftShin),
      3 => Some(BodyLocationField::LeftHamstring),
      4 => Some(BodyLocationField::LeftQuad),
      5 => Some(BodyLocationField::LeftGlute),
      6 => Some(BodyLocationField::RightLeg),
      7 => Some(BodyLocationField::RightCalf),
      8 => Some(BodyLocationField::RightShin),
      9 => Some(BodyLocationField::RightHamstring),
      10 => Some(BodyLocationField::RightQuad),
      11 => Some(BodyLocationField::RightGlute),
      12 => Some(BodyLocationField::TorsoBack),
      13 => Some(BodyLocationField::LeftLowerBack),
      14 => Some(BodyLocationField::LeftUpperBack),
      15 => Some(BodyLocationField::RightLowerBack),
      16 => Some(BodyLocationField::RightUpperBack),
      17 => Some(BodyLocationField::TorsoFront),
      18 => Some(BodyLocationField::LeftAbdomen),
      19 => Some(BodyLocationField::LeftChest),
      20 => Some(BodyLocationField::RightAbdomen),
      21 => Some(BodyLocationField::RightChest),
      22 => Some(BodyLocationField::LeftArm),
      23 => Some(BodyLocationField::LeftShoulder),
      24 => Some(BodyLocationField::LeftBicep),
      25 => Some(BodyLocationField::LeftTricep),
      26 => Some(BodyLocationField::LeftBrachioradialis),
      27 => Some(BodyLocationField::LeftForearmExtensors),
      28 => Some(BodyLocationField::RightArm),
      29 => Some(BodyLocationField::RightShoulder),
      30 => Some(BodyLocationField::RightBicep),
      31 => Some(BodyLocationField::RightTricep),
      32 => Some(BodyLocationField::RightBrachioradialis),
      33 => Some(BodyLocationField::RightForearmExtensors),
      34 => Some(BodyLocationField::Neck),
      35 => Some(BodyLocationField::Throat),
      36 => Some(BodyLocationField::WaistMidBack),
      37 => Some(BodyLocationField::WaistFront),
      38 => Some(BodyLocationField::WaistLeft),
      39 => Some(BodyLocationField::WaistRight),
      _ => None,
    }
  }
}

/* name: language_bits_1 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageBits1Field {
  Dutch = 1,
  Finnish = 2,
  Greek = 4,
  Hungarian = 8,
  Norwegian = 16,
  Polish = 32,
  Portuguese = 64,
  Slovakian = 128,
}

impl From<LanguageBits1Field> for u8 {
  fn from(source: LanguageBits1Field) -> u8 {
    match source {
      LanguageBits1Field::Dutch => 1,
      LanguageBits1Field::Finnish => 2,
      LanguageBits1Field::Greek => 4,
      LanguageBits1Field::Hungarian => 8,
      LanguageBits1Field::Norwegian => 16,
      LanguageBits1Field::Polish => 32,
      LanguageBits1Field::Portuguese => 64,
      LanguageBits1Field::Slovakian => 128,
    }
  }
}

impl IntoField<u8> for LanguageBits1Field {
  fn into_field(value: u8) -> Option<LanguageBits1Field> {
    match value {
      1 => Some(LanguageBits1Field::Dutch),
      2 => Some(LanguageBits1Field::Finnish),
      4 => Some(LanguageBits1Field::Greek),
      8 => Some(LanguageBits1Field::Hungarian),
      16 => Some(LanguageBits1Field::Norwegian),
      32 => Some(LanguageBits1Field::Polish),
      64 => Some(LanguageBits1Field::Portuguese),
      128 => Some(LanguageBits1Field::Slovakian),
      _ => None,
    }
  }
}

/* name: course_capabilities type: uint32z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum CourseCapabilitiesField {
  Processed = 1,
  Valid = 2,
  Time = 4,
  Distance = 8,
  Position = 16,
  HeartRate = 32,
  Power = 64,
  Cadence = 128,
  Training = 256,
  Navigation = 512,
  Bikeway = 1024,
}

impl From<CourseCapabilitiesField> for u32 {
  fn from(source: CourseCapabilitiesField) -> u32 {
    match source {
      CourseCapabilitiesField::Processed => 1,
      CourseCapabilitiesField::Valid => 2,
      CourseCapabilitiesField::Time => 4,
      CourseCapabilitiesField::Distance => 8,
      CourseCapabilitiesField::Position => 16,
      CourseCapabilitiesField::HeartRate => 32,
      CourseCapabilitiesField::Power => 64,
      CourseCapabilitiesField::Cadence => 128,
      CourseCapabilitiesField::Training => 256,
      CourseCapabilitiesField::Navigation => 512,
      CourseCapabilitiesField::Bikeway => 1024,
    }
  }
}

impl IntoField<u32> for CourseCapabilitiesField {
  fn into_field(value: u32) -> Option<CourseCapabilitiesField> {
    match value {
      1 => Some(CourseCapabilitiesField::Processed),
      2 => Some(CourseCapabilitiesField::Valid),
      4 => Some(CourseCapabilitiesField::Time),
      8 => Some(CourseCapabilitiesField::Distance),
      16 => Some(CourseCapabilitiesField::Position),
      32 => Some(CourseCapabilitiesField::HeartRate),
      64 => Some(CourseCapabilitiesField::Power),
      128 => Some(CourseCapabilitiesField::Cadence),
      256 => Some(CourseCapabilitiesField::Training),
      512 => Some(CourseCapabilitiesField::Navigation),
      1024 => Some(CourseCapabilitiesField::Bikeway),
      _ => None,
    }
  }
}

/* name: checksum type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ChecksumField {
  Clear = 0,
  Ok = 1,
}

impl From<ChecksumField> for u8 {
  fn from(source: ChecksumField) -> u8 {
    match source {
      ChecksumField::Clear => 0,
      ChecksumField::Ok => 1,
    }
  }
}

impl IntoField<u8> for ChecksumField {
  fn into_field(value: u8) -> Option<ChecksumField> {
    match value {
      0 => Some(ChecksumField::Clear),
      1 => Some(ChecksumField::Ok),
      _ => None,
    }
  }
}

/* name: display_orientation type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DisplayOrientationField {
  Auto = 0,
  Portrait = 1,
  Landscape = 2,
  PortraitFlipped = 3,
  LandscapeFlipped = 4,
}

impl From<DisplayOrientationField> for u8 {
  fn from(source: DisplayOrientationField) -> u8 {
    match source {
      DisplayOrientationField::Auto => 0,
      DisplayOrientationField::Portrait => 1,
      DisplayOrientationField::Landscape => 2,
      DisplayOrientationField::PortraitFlipped => 3,
      DisplayOrientationField::LandscapeFlipped => 4,
    }
  }
}

impl IntoField<u8> for DisplayOrientationField {
  fn into_field(value: u8) -> Option<DisplayOrientationField> {
    match value {
      0 => Some(DisplayOrientationField::Auto),
      1 => Some(DisplayOrientationField::Portrait),
      2 => Some(DisplayOrientationField::Landscape),
      3 => Some(DisplayOrientationField::PortraitFlipped),
      4 => Some(DisplayOrientationField::LandscapeFlipped),
      _ => None,
    }
  }
}

/* name: sensor_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SensorTypeField {
  Accelerometer = 0,
  Gyroscope = 1,
  Compass = 2,
  Barometer = 3,
}

impl From<SensorTypeField> for u8 {
  fn from(source: SensorTypeField) -> u8 {
    match source {
      SensorTypeField::Accelerometer => 0,
      SensorTypeField::Gyroscope => 1,
      SensorTypeField::Compass => 2,
      SensorTypeField::Barometer => 3,
    }
  }
}

impl IntoField<u8> for SensorTypeField {
  fn into_field(value: u8) -> Option<SensorTypeField> {
    match value {
      0 => Some(SensorTypeField::Accelerometer),
      1 => Some(SensorTypeField::Gyroscope),
      2 => Some(SensorTypeField::Compass),
      3 => Some(SensorTypeField::Barometer),
      _ => None,
    }
  }
}

/* name: time_zone type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimeZoneField {
  Almaty = 0,
  Bangkok = 1,
  Bombay = 2,
  Brasilia = 3,
  Cairo = 4,
  CapeVerdeIs = 5,
  Darwin = 6,
  Eniwetok = 7,
  Fiji = 8,
  HongKong = 9,
  Islamabad = 10,
  Kabul = 11,
  Magadan = 12,
  MidAtlantic = 13,
  Moscow = 14,
  Muscat = 15,
  Newfoundland = 16,
  Samoa = 17,
  Sydney = 18,
  Tehran = 19,
  Tokyo = 20,
  UsAlaska = 21,
  UsAtlantic = 22,
  UsCentral = 23,
  UsEastern = 24,
  UsHawaii = 25,
  UsMountain = 26,
  UsPacific = 27,
  Other = 28,
  Auckland = 29,
  Kathmandu = 30,
  EuropeWesternWet = 31,
  EuropeCentralCet = 32,
  EuropeEasternEet = 33,
  Jakarta = 34,
  Perth = 35,
  Adelaide = 36,
  Brisbane = 37,
  Tasmania = 38,
  Iceland = 39,
  Amsterdam = 40,
  Athens = 41,
  Barcelona = 42,
  Berlin = 43,
  Brussels = 44,
  Budapest = 45,
  Copenhagen = 46,
  Dublin = 47,
  Helsinki = 48,
  Lisbon = 49,
  London = 50,
  Madrid = 51,
  Munich = 52,
  Oslo = 53,
  Paris = 54,
  Prague = 55,
  Reykjavik = 56,
  Rome = 57,
  Stockholm = 58,
  Vienna = 59,
  Warsaw = 60,
  Zurich = 61,
  Quebec = 62,
  Ontario = 63,
  Manitoba = 64,
  Saskatchewan = 65,
  Alberta = 66,
  BritishColumbia = 67,
  Boise = 68,
  Boston = 69,
  Chicago = 70,
  Dallas = 71,
  Denver = 72,
  KansasCity = 73,
  LasVegas = 74,
  LosAngeles = 75,
  Miami = 76,
  Minneapolis = 77,
  NewYork = 78,
  NewOrleans = 79,
  Phoenix = 80,
  SantaFe = 81,
  Seattle = 82,
  WashingtonDc = 83,
  UsArizona = 84,
  Chita = 85,
  Ekaterinburg = 86,
  Irkutsk = 87,
  Kaliningrad = 88,
  Krasnoyarsk = 89,
  Novosibirsk = 90,
  PetropavlovskKamchatskiy = 91,
  Samara = 92,
  Vladivostok = 93,
  MexicoCentral = 94,
  MexicoMountain = 95,
  MexicoPacific = 96,
  CapeTown = 97,
  Winkhoek = 98,
  Lagos = 99,
  Riyahd = 100,
  Venezuela = 101,
  AustraliaLh = 102,
  Santiago = 103,
  Manual = 253,
  Automatic = 254,
}

impl From<TimeZoneField> for u8 {
  fn from(source: TimeZoneField) -> u8 {
    match source {
      TimeZoneField::Almaty => 0,
      TimeZoneField::Bangkok => 1,
      TimeZoneField::Bombay => 2,
      TimeZoneField::Brasilia => 3,
      TimeZoneField::Cairo => 4,
      TimeZoneField::CapeVerdeIs => 5,
      TimeZoneField::Darwin => 6,
      TimeZoneField::Eniwetok => 7,
      TimeZoneField::Fiji => 8,
      TimeZoneField::HongKong => 9,
      TimeZoneField::Islamabad => 10,
      TimeZoneField::Kabul => 11,
      TimeZoneField::Magadan => 12,
      TimeZoneField::MidAtlantic => 13,
      TimeZoneField::Moscow => 14,
      TimeZoneField::Muscat => 15,
      TimeZoneField::Newfoundland => 16,
      TimeZoneField::Samoa => 17,
      TimeZoneField::Sydney => 18,
      TimeZoneField::Tehran => 19,
      TimeZoneField::Tokyo => 20,
      TimeZoneField::UsAlaska => 21,
      TimeZoneField::UsAtlantic => 22,
      TimeZoneField::UsCentral => 23,
      TimeZoneField::UsEastern => 24,
      TimeZoneField::UsHawaii => 25,
      TimeZoneField::UsMountain => 26,
      TimeZoneField::UsPacific => 27,
      TimeZoneField::Other => 28,
      TimeZoneField::Auckland => 29,
      TimeZoneField::Kathmandu => 30,
      TimeZoneField::EuropeWesternWet => 31,
      TimeZoneField::EuropeCentralCet => 32,
      TimeZoneField::EuropeEasternEet => 33,
      TimeZoneField::Jakarta => 34,
      TimeZoneField::Perth => 35,
      TimeZoneField::Adelaide => 36,
      TimeZoneField::Brisbane => 37,
      TimeZoneField::Tasmania => 38,
      TimeZoneField::Iceland => 39,
      TimeZoneField::Amsterdam => 40,
      TimeZoneField::Athens => 41,
      TimeZoneField::Barcelona => 42,
      TimeZoneField::Berlin => 43,
      TimeZoneField::Brussels => 44,
      TimeZoneField::Budapest => 45,
      TimeZoneField::Copenhagen => 46,
      TimeZoneField::Dublin => 47,
      TimeZoneField::Helsinki => 48,
      TimeZoneField::Lisbon => 49,
      TimeZoneField::London => 50,
      TimeZoneField::Madrid => 51,
      TimeZoneField::Munich => 52,
      TimeZoneField::Oslo => 53,
      TimeZoneField::Paris => 54,
      TimeZoneField::Prague => 55,
      TimeZoneField::Reykjavik => 56,
      TimeZoneField::Rome => 57,
      TimeZoneField::Stockholm => 58,
      TimeZoneField::Vienna => 59,
      TimeZoneField::Warsaw => 60,
      TimeZoneField::Zurich => 61,
      TimeZoneField::Quebec => 62,
      TimeZoneField::Ontario => 63,
      TimeZoneField::Manitoba => 64,
      TimeZoneField::Saskatchewan => 65,
      TimeZoneField::Alberta => 66,
      TimeZoneField::BritishColumbia => 67,
      TimeZoneField::Boise => 68,
      TimeZoneField::Boston => 69,
      TimeZoneField::Chicago => 70,
      TimeZoneField::Dallas => 71,
      TimeZoneField::Denver => 72,
      TimeZoneField::KansasCity => 73,
      TimeZoneField::LasVegas => 74,
      TimeZoneField::LosAngeles => 75,
      TimeZoneField::Miami => 76,
      TimeZoneField::Minneapolis => 77,
      TimeZoneField::NewYork => 78,
      TimeZoneField::NewOrleans => 79,
      TimeZoneField::Phoenix => 80,
      TimeZoneField::SantaFe => 81,
      TimeZoneField::Seattle => 82,
      TimeZoneField::WashingtonDc => 83,
      TimeZoneField::UsArizona => 84,
      TimeZoneField::Chita => 85,
      TimeZoneField::Ekaterinburg => 86,
      TimeZoneField::Irkutsk => 87,
      TimeZoneField::Kaliningrad => 88,
      TimeZoneField::Krasnoyarsk => 89,
      TimeZoneField::Novosibirsk => 90,
      TimeZoneField::PetropavlovskKamchatskiy => 91,
      TimeZoneField::Samara => 92,
      TimeZoneField::Vladivostok => 93,
      TimeZoneField::MexicoCentral => 94,
      TimeZoneField::MexicoMountain => 95,
      TimeZoneField::MexicoPacific => 96,
      TimeZoneField::CapeTown => 97,
      TimeZoneField::Winkhoek => 98,
      TimeZoneField::Lagos => 99,
      TimeZoneField::Riyahd => 100,
      TimeZoneField::Venezuela => 101,
      TimeZoneField::AustraliaLh => 102,
      TimeZoneField::Santiago => 103,
      TimeZoneField::Manual => 253,
      TimeZoneField::Automatic => 254,
    }
  }
}

impl IntoField<u8> for TimeZoneField {
  fn into_field(value: u8) -> Option<TimeZoneField> {
    match value {
      0 => Some(TimeZoneField::Almaty),
      1 => Some(TimeZoneField::Bangkok),
      2 => Some(TimeZoneField::Bombay),
      3 => Some(TimeZoneField::Brasilia),
      4 => Some(TimeZoneField::Cairo),
      5 => Some(TimeZoneField::CapeVerdeIs),
      6 => Some(TimeZoneField::Darwin),
      7 => Some(TimeZoneField::Eniwetok),
      8 => Some(TimeZoneField::Fiji),
      9 => Some(TimeZoneField::HongKong),
      10 => Some(TimeZoneField::Islamabad),
      11 => Some(TimeZoneField::Kabul),
      12 => Some(TimeZoneField::Magadan),
      13 => Some(TimeZoneField::MidAtlantic),
      14 => Some(TimeZoneField::Moscow),
      15 => Some(TimeZoneField::Muscat),
      16 => Some(TimeZoneField::Newfoundland),
      17 => Some(TimeZoneField::Samoa),
      18 => Some(TimeZoneField::Sydney),
      19 => Some(TimeZoneField::Tehran),
      20 => Some(TimeZoneField::Tokyo),
      21 => Some(TimeZoneField::UsAlaska),
      22 => Some(TimeZoneField::UsAtlantic),
      23 => Some(TimeZoneField::UsCentral),
      24 => Some(TimeZoneField::UsEastern),
      25 => Some(TimeZoneField::UsHawaii),
      26 => Some(TimeZoneField::UsMountain),
      27 => Some(TimeZoneField::UsPacific),
      28 => Some(TimeZoneField::Other),
      29 => Some(TimeZoneField::Auckland),
      30 => Some(TimeZoneField::Kathmandu),
      31 => Some(TimeZoneField::EuropeWesternWet),
      32 => Some(TimeZoneField::EuropeCentralCet),
      33 => Some(TimeZoneField::EuropeEasternEet),
      34 => Some(TimeZoneField::Jakarta),
      35 => Some(TimeZoneField::Perth),
      36 => Some(TimeZoneField::Adelaide),
      37 => Some(TimeZoneField::Brisbane),
      38 => Some(TimeZoneField::Tasmania),
      39 => Some(TimeZoneField::Iceland),
      40 => Some(TimeZoneField::Amsterdam),
      41 => Some(TimeZoneField::Athens),
      42 => Some(TimeZoneField::Barcelona),
      43 => Some(TimeZoneField::Berlin),
      44 => Some(TimeZoneField::Brussels),
      45 => Some(TimeZoneField::Budapest),
      46 => Some(TimeZoneField::Copenhagen),
      47 => Some(TimeZoneField::Dublin),
      48 => Some(TimeZoneField::Helsinki),
      49 => Some(TimeZoneField::Lisbon),
      50 => Some(TimeZoneField::London),
      51 => Some(TimeZoneField::Madrid),
      52 => Some(TimeZoneField::Munich),
      53 => Some(TimeZoneField::Oslo),
      54 => Some(TimeZoneField::Paris),
      55 => Some(TimeZoneField::Prague),
      56 => Some(TimeZoneField::Reykjavik),
      57 => Some(TimeZoneField::Rome),
      58 => Some(TimeZoneField::Stockholm),
      59 => Some(TimeZoneField::Vienna),
      60 => Some(TimeZoneField::Warsaw),
      61 => Some(TimeZoneField::Zurich),
      62 => Some(TimeZoneField::Quebec),
      63 => Some(TimeZoneField::Ontario),
      64 => Some(TimeZoneField::Manitoba),
      65 => Some(TimeZoneField::Saskatchewan),
      66 => Some(TimeZoneField::Alberta),
      67 => Some(TimeZoneField::BritishColumbia),
      68 => Some(TimeZoneField::Boise),
      69 => Some(TimeZoneField::Boston),
      70 => Some(TimeZoneField::Chicago),
      71 => Some(TimeZoneField::Dallas),
      72 => Some(TimeZoneField::Denver),
      73 => Some(TimeZoneField::KansasCity),
      74 => Some(TimeZoneField::LasVegas),
      75 => Some(TimeZoneField::LosAngeles),
      76 => Some(TimeZoneField::Miami),
      77 => Some(TimeZoneField::Minneapolis),
      78 => Some(TimeZoneField::NewYork),
      79 => Some(TimeZoneField::NewOrleans),
      80 => Some(TimeZoneField::Phoenix),
      81 => Some(TimeZoneField::SantaFe),
      82 => Some(TimeZoneField::Seattle),
      83 => Some(TimeZoneField::WashingtonDc),
      84 => Some(TimeZoneField::UsArizona),
      85 => Some(TimeZoneField::Chita),
      86 => Some(TimeZoneField::Ekaterinburg),
      87 => Some(TimeZoneField::Irkutsk),
      88 => Some(TimeZoneField::Kaliningrad),
      89 => Some(TimeZoneField::Krasnoyarsk),
      90 => Some(TimeZoneField::Novosibirsk),
      91 => Some(TimeZoneField::PetropavlovskKamchatskiy),
      92 => Some(TimeZoneField::Samara),
      93 => Some(TimeZoneField::Vladivostok),
      94 => Some(TimeZoneField::MexicoCentral),
      95 => Some(TimeZoneField::MexicoMountain),
      96 => Some(TimeZoneField::MexicoPacific),
      97 => Some(TimeZoneField::CapeTown),
      98 => Some(TimeZoneField::Winkhoek),
      99 => Some(TimeZoneField::Lagos),
      100 => Some(TimeZoneField::Riyahd),
      101 => Some(TimeZoneField::Venezuela),
      102 => Some(TimeZoneField::AustraliaLh),
      103 => Some(TimeZoneField::Santiago),
      253 => Some(TimeZoneField::Manual),
      254 => Some(TimeZoneField::Automatic),
      _ => None,
    }
  }
}

/* name: activity type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ActivityField {
  Manual = 0,
  AutoMultiSport = 1,
}

impl From<ActivityField> for u8 {
  fn from(source: ActivityField) -> u8 {
    match source {
      ActivityField::Manual => 0,
      ActivityField::AutoMultiSport => 1,
    }
  }
}

impl IntoField<u8> for ActivityField {
  fn into_field(value: u8) -> Option<ActivityField> {
    match value {
      0 => Some(ActivityField::Manual),
      1 => Some(ActivityField::AutoMultiSport),
      _ => None,
    }
  }
}

/* name: activity_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ActivityTypeField {
  Generic = 0,
  Running = 1,
  Cycling = 2,
  Transition = 3,
  FitnessEquipment = 4,
  Swimming = 5,
  Walking = 6,
  Sedentary = 8,
  All = 254,
}

impl From<ActivityTypeField> for u8 {
  fn from(source: ActivityTypeField) -> u8 {
    match source {
      ActivityTypeField::Generic => 0,
      ActivityTypeField::Running => 1,
      ActivityTypeField::Cycling => 2,
      ActivityTypeField::Transition => 3,
      ActivityTypeField::FitnessEquipment => 4,
      ActivityTypeField::Swimming => 5,
      ActivityTypeField::Walking => 6,
      ActivityTypeField::Sedentary => 8,
      ActivityTypeField::All => 254,
    }
  }
}

impl IntoField<u8> for ActivityTypeField {
  fn into_field(value: u8) -> Option<ActivityTypeField> {
    match value {
      0 => Some(ActivityTypeField::Generic),
      1 => Some(ActivityTypeField::Running),
      2 => Some(ActivityTypeField::Cycling),
      3 => Some(ActivityTypeField::Transition),
      4 => Some(ActivityTypeField::FitnessEquipment),
      5 => Some(ActivityTypeField::Swimming),
      6 => Some(ActivityTypeField::Walking),
      8 => Some(ActivityTypeField::Sedentary),
      254 => Some(ActivityTypeField::All),
      _ => None,
    }
  }
}

/* name: autoscroll type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AutoscrollField {
  None = 0,
  Slow = 1,
  Medium = 2,
  Fast = 3,
}

impl From<AutoscrollField> for u8 {
  fn from(source: AutoscrollField) -> u8 {
    match source {
      AutoscrollField::None => 0,
      AutoscrollField::Slow => 1,
      AutoscrollField::Medium => 2,
      AutoscrollField::Fast => 3,
    }
  }
}

impl IntoField<u8> for AutoscrollField {
  fn into_field(value: u8) -> Option<AutoscrollField> {
    match value {
      0 => Some(AutoscrollField::None),
      1 => Some(AutoscrollField::Slow),
      2 => Some(AutoscrollField::Medium),
      3 => Some(AutoscrollField::Fast),
      _ => None,
    }
  }
}

/* name: hr_zone_calc type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum HrZoneCalcField {
  Custom = 0,
  PercentMaxHr = 1,
  PercentHrr = 2,
}

impl From<HrZoneCalcField> for u8 {
  fn from(source: HrZoneCalcField) -> u8 {
    match source {
      HrZoneCalcField::Custom => 0,
      HrZoneCalcField::PercentMaxHr => 1,
      HrZoneCalcField::PercentHrr => 2,
    }
  }
}

impl IntoField<u8> for HrZoneCalcField {
  fn into_field(value: u8) -> Option<HrZoneCalcField> {
    match value {
      0 => Some(HrZoneCalcField::Custom),
      1 => Some(HrZoneCalcField::PercentMaxHr),
      2 => Some(HrZoneCalcField::PercentHrr),
      _ => None,
    }
  }
}

/* name: time_into_day type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimeIntoDayField {
}

impl From<TimeIntoDayField> for u32 {
  fn from(source: TimeIntoDayField) -> u32 {
    match source {
    }
  }
}

impl IntoField<u32> for TimeIntoDayField {
  fn into_field(value: u32) -> Option<TimeIntoDayField> {
    match value {
      _ => None,
    }
  }
}

/* name: lap_trigger type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LapTriggerField {
  Manual = 0,
  Time = 1,
  Distance = 2,
  PositionStart = 3,
  PositionLap = 4,
  PositionWaypoint = 5,
  PositionMarked = 6,
  SessionEnd = 7,
  FitnessEquipment = 8,
}

impl From<LapTriggerField> for u8 {
  fn from(source: LapTriggerField) -> u8 {
    match source {
      LapTriggerField::Manual => 0,
      LapTriggerField::Time => 1,
      LapTriggerField::Distance => 2,
      LapTriggerField::PositionStart => 3,
      LapTriggerField::PositionLap => 4,
      LapTriggerField::PositionWaypoint => 5,
      LapTriggerField::PositionMarked => 6,
      LapTriggerField::SessionEnd => 7,
      LapTriggerField::FitnessEquipment => 8,
    }
  }
}

impl IntoField<u8> for LapTriggerField {
  fn into_field(value: u8) -> Option<LapTriggerField> {
    match value {
      0 => Some(LapTriggerField::Manual),
      1 => Some(LapTriggerField::Time),
      2 => Some(LapTriggerField::Distance),
      3 => Some(LapTriggerField::PositionStart),
      4 => Some(LapTriggerField::PositionLap),
      5 => Some(LapTriggerField::PositionWaypoint),
      6 => Some(LapTriggerField::PositionMarked),
      7 => Some(LapTriggerField::SessionEnd),
      8 => Some(LapTriggerField::FitnessEquipment),
      _ => None,
    }
  }
}

/* name: display_measure type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DisplayMeasureField {
  Metric = 0,
  Statute = 1,
  Nautical = 2,
}

impl From<DisplayMeasureField> for u8 {
  fn from(source: DisplayMeasureField) -> u8 {
    match source {
      DisplayMeasureField::Metric => 0,
      DisplayMeasureField::Statute => 1,
      DisplayMeasureField::Nautical => 2,
    }
  }
}

impl IntoField<u8> for DisplayMeasureField {
  fn into_field(value: u8) -> Option<DisplayMeasureField> {
    match value {
      0 => Some(DisplayMeasureField::Metric),
      1 => Some(DisplayMeasureField::Statute),
      2 => Some(DisplayMeasureField::Nautical),
      _ => None,
    }
  }
}

/* name: supported_exd_screen_layouts type: uint32z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SupportedExdScreenLayoutsField {
  FullScreen = 1,
  HalfVertical = 2,
  HalfHorizontal = 4,
  HalfVerticalRightSplit = 8,
  HalfHorizontalBottomSplit = 16,
  FullQuarterSplit = 32,
  HalfVerticalLeftSplit = 64,
  HalfHorizontalTopSplit = 128,
}

impl From<SupportedExdScreenLayoutsField> for u32 {
  fn from(source: SupportedExdScreenLayoutsField) -> u32 {
    match source {
      SupportedExdScreenLayoutsField::FullScreen => 1,
      SupportedExdScreenLayoutsField::HalfVertical => 2,
      SupportedExdScreenLayoutsField::HalfHorizontal => 4,
      SupportedExdScreenLayoutsField::HalfVerticalRightSplit => 8,
      SupportedExdScreenLayoutsField::HalfHorizontalBottomSplit => 16,
      SupportedExdScreenLayoutsField::FullQuarterSplit => 32,
      SupportedExdScreenLayoutsField::HalfVerticalLeftSplit => 64,
      SupportedExdScreenLayoutsField::HalfHorizontalTopSplit => 128,
    }
  }
}

impl IntoField<u32> for SupportedExdScreenLayoutsField {
  fn into_field(value: u32) -> Option<SupportedExdScreenLayoutsField> {
    match value {
      1 => Some(SupportedExdScreenLayoutsField::FullScreen),
      2 => Some(SupportedExdScreenLayoutsField::HalfVertical),
      4 => Some(SupportedExdScreenLayoutsField::HalfHorizontal),
      8 => Some(SupportedExdScreenLayoutsField::HalfVerticalRightSplit),
      16 => Some(SupportedExdScreenLayoutsField::HalfHorizontalBottomSplit),
      32 => Some(SupportedExdScreenLayoutsField::FullQuarterSplit),
      64 => Some(SupportedExdScreenLayoutsField::HalfVerticalLeftSplit),
      128 => Some(SupportedExdScreenLayoutsField::HalfHorizontalTopSplit),
      _ => None,
    }
  }
}

/* name: turn_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum TurnTypeField {
  ArrivingIdx = 0,
  ArrivingLeftIdx = 1,
  ArrivingRightIdx = 2,
  ArrivingViaIdx = 3,
  ArrivingViaLeftIdx = 4,
  ArrivingViaRightIdx = 5,
  BearKeepLeftIdx = 6,
  BearKeepRightIdx = 7,
  ContinueIdx = 8,
  ExitLeftIdx = 9,
  ExitRightIdx = 10,
  FerryIdx = 11,
  Roundabout45Idx = 12,
  Roundabout90Idx = 13,
  Roundabout135Idx = 14,
  Roundabout180Idx = 15,
  Roundabout225Idx = 16,
  Roundabout270Idx = 17,
  Roundabout315Idx = 18,
  Roundabout360Idx = 19,
  RoundaboutNeg45Idx = 20,
  RoundaboutNeg90Idx = 21,
  RoundaboutNeg135Idx = 22,
  RoundaboutNeg180Idx = 23,
  RoundaboutNeg225Idx = 24,
  RoundaboutNeg270Idx = 25,
  RoundaboutNeg315Idx = 26,
  RoundaboutNeg360Idx = 27,
  RoundaboutGenericIdx = 28,
  RoundaboutNegGenericIdx = 29,
  SharpTurnLeftIdx = 30,
  SharpTurnRightIdx = 31,
  TurnLeftIdx = 32,
  TurnRightIdx = 33,
  UturnLeftIdx = 34,
  UturnRightIdx = 35,
  IconInvIdx = 36,
  IconIdxCnt = 37,
}

impl From<TurnTypeField> for u8 {
  fn from(source: TurnTypeField) -> u8 {
    match source {
      TurnTypeField::ArrivingIdx => 0,
      TurnTypeField::ArrivingLeftIdx => 1,
      TurnTypeField::ArrivingRightIdx => 2,
      TurnTypeField::ArrivingViaIdx => 3,
      TurnTypeField::ArrivingViaLeftIdx => 4,
      TurnTypeField::ArrivingViaRightIdx => 5,
      TurnTypeField::BearKeepLeftIdx => 6,
      TurnTypeField::BearKeepRightIdx => 7,
      TurnTypeField::ContinueIdx => 8,
      TurnTypeField::ExitLeftIdx => 9,
      TurnTypeField::ExitRightIdx => 10,
      TurnTypeField::FerryIdx => 11,
      TurnTypeField::Roundabout45Idx => 12,
      TurnTypeField::Roundabout90Idx => 13,
      TurnTypeField::Roundabout135Idx => 14,
      TurnTypeField::Roundabout180Idx => 15,
      TurnTypeField::Roundabout225Idx => 16,
      TurnTypeField::Roundabout270Idx => 17,
      TurnTypeField::Roundabout315Idx => 18,
      TurnTypeField::Roundabout360Idx => 19,
      TurnTypeField::RoundaboutNeg45Idx => 20,
      TurnTypeField::RoundaboutNeg90Idx => 21,
      TurnTypeField::RoundaboutNeg135Idx => 22,
      TurnTypeField::RoundaboutNeg180Idx => 23,
      TurnTypeField::RoundaboutNeg225Idx => 24,
      TurnTypeField::RoundaboutNeg270Idx => 25,
      TurnTypeField::RoundaboutNeg315Idx => 26,
      TurnTypeField::RoundaboutNeg360Idx => 27,
      TurnTypeField::RoundaboutGenericIdx => 28,
      TurnTypeField::RoundaboutNegGenericIdx => 29,
      TurnTypeField::SharpTurnLeftIdx => 30,
      TurnTypeField::SharpTurnRightIdx => 31,
      TurnTypeField::TurnLeftIdx => 32,
      TurnTypeField::TurnRightIdx => 33,
      TurnTypeField::UturnLeftIdx => 34,
      TurnTypeField::UturnRightIdx => 35,
      TurnTypeField::IconInvIdx => 36,
      TurnTypeField::IconIdxCnt => 37,
    }
  }
}

impl IntoField<u8> for TurnTypeField {
  fn into_field(value: u8) -> Option<TurnTypeField> {
    match value {
      0 => Some(TurnTypeField::ArrivingIdx),
      1 => Some(TurnTypeField::ArrivingLeftIdx),
      2 => Some(TurnTypeField::ArrivingRightIdx),
      3 => Some(TurnTypeField::ArrivingViaIdx),
      4 => Some(TurnTypeField::ArrivingViaLeftIdx),
      5 => Some(TurnTypeField::ArrivingViaRightIdx),
      6 => Some(TurnTypeField::BearKeepLeftIdx),
      7 => Some(TurnTypeField::BearKeepRightIdx),
      8 => Some(TurnTypeField::ContinueIdx),
      9 => Some(TurnTypeField::ExitLeftIdx),
      10 => Some(TurnTypeField::ExitRightIdx),
      11 => Some(TurnTypeField::FerryIdx),
      12 => Some(TurnTypeField::Roundabout45Idx),
      13 => Some(TurnTypeField::Roundabout90Idx),
      14 => Some(TurnTypeField::Roundabout135Idx),
      15 => Some(TurnTypeField::Roundabout180Idx),
      16 => Some(TurnTypeField::Roundabout225Idx),
      17 => Some(TurnTypeField::Roundabout270Idx),
      18 => Some(TurnTypeField::Roundabout315Idx),
      19 => Some(TurnTypeField::Roundabout360Idx),
      20 => Some(TurnTypeField::RoundaboutNeg45Idx),
      21 => Some(TurnTypeField::RoundaboutNeg90Idx),
      22 => Some(TurnTypeField::RoundaboutNeg135Idx),
      23 => Some(TurnTypeField::RoundaboutNeg180Idx),
      24 => Some(TurnTypeField::RoundaboutNeg225Idx),
      25 => Some(TurnTypeField::RoundaboutNeg270Idx),
      26 => Some(TurnTypeField::RoundaboutNeg315Idx),
      27 => Some(TurnTypeField::RoundaboutNeg360Idx),
      28 => Some(TurnTypeField::RoundaboutGenericIdx),
      29 => Some(TurnTypeField::RoundaboutNegGenericIdx),
      30 => Some(TurnTypeField::SharpTurnLeftIdx),
      31 => Some(TurnTypeField::SharpTurnRightIdx),
      32 => Some(TurnTypeField::TurnLeftIdx),
      33 => Some(TurnTypeField::TurnRightIdx),
      34 => Some(TurnTypeField::UturnLeftIdx),
      35 => Some(TurnTypeField::UturnRightIdx),
      36 => Some(TurnTypeField::IconInvIdx),
      37 => Some(TurnTypeField::IconIdxCnt),
      _ => None,
    }
  }
}

/* name: digital_watchface_layout type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DigitalWatchfaceLayoutField {
  Traditional = 0,
  Modern = 1,
  Bold = 2,
}

impl From<DigitalWatchfaceLayoutField> for u8 {
  fn from(source: DigitalWatchfaceLayoutField) -> u8 {
    match source {
      DigitalWatchfaceLayoutField::Traditional => 0,
      DigitalWatchfaceLayoutField::Modern => 1,
      DigitalWatchfaceLayoutField::Bold => 2,
    }
  }
}

impl IntoField<u8> for DigitalWatchfaceLayoutField {
  fn into_field(value: u8) -> Option<DigitalWatchfaceLayoutField> {
    match value {
      0 => Some(DigitalWatchfaceLayoutField::Traditional),
      1 => Some(DigitalWatchfaceLayoutField::Modern),
      2 => Some(DigitalWatchfaceLayoutField::Bold),
      _ => None,
    }
  }
}

/* name: pwr_zone_calc type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum PwrZoneCalcField {
  Custom = 0,
  PercentFtp = 1,
}

impl From<PwrZoneCalcField> for u8 {
  fn from(source: PwrZoneCalcField) -> u8 {
    match source {
      PwrZoneCalcField::Custom => 0,
      PwrZoneCalcField::PercentFtp => 1,
    }
  }
}

impl IntoField<u8> for PwrZoneCalcField {
  fn into_field(value: u8) -> Option<PwrZoneCalcField> {
    match value {
      0 => Some(PwrZoneCalcField::Custom),
      1 => Some(PwrZoneCalcField::PercentFtp),
      _ => None,
    }
  }
}

/* name: attitude_validity type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AttitudeValidityField {
  TrackAngleHeadingValid = 1,
  PitchValid = 2,
  RollValid = 4,
  LateralBodyAccelValid = 8,
  NormalBodyAccelValid = 16,
  TurnRateValid = 32,
  HwFail = 64,
  MagInvalid = 128,
  NoGps = 256,
  GpsInvalid = 512,
  SolutionCoasting = 1024,
  TrueTrackAngle = 2048,
  MagneticHeading = 4096,
}

impl From<AttitudeValidityField> for u16 {
  fn from(source: AttitudeValidityField) -> u16 {
    match source {
      AttitudeValidityField::TrackAngleHeadingValid => 1,
      AttitudeValidityField::PitchValid => 2,
      AttitudeValidityField::RollValid => 4,
      AttitudeValidityField::LateralBodyAccelValid => 8,
      AttitudeValidityField::NormalBodyAccelValid => 16,
      AttitudeValidityField::TurnRateValid => 32,
      AttitudeValidityField::HwFail => 64,
      AttitudeValidityField::MagInvalid => 128,
      AttitudeValidityField::NoGps => 256,
      AttitudeValidityField::GpsInvalid => 512,
      AttitudeValidityField::SolutionCoasting => 1024,
      AttitudeValidityField::TrueTrackAngle => 2048,
      AttitudeValidityField::MagneticHeading => 4096,
    }
  }
}

impl IntoField<u16> for AttitudeValidityField {
  fn into_field(value: u16) -> Option<AttitudeValidityField> {
    match value {
      1 => Some(AttitudeValidityField::TrackAngleHeadingValid),
      2 => Some(AttitudeValidityField::PitchValid),
      4 => Some(AttitudeValidityField::RollValid),
      8 => Some(AttitudeValidityField::LateralBodyAccelValid),
      16 => Some(AttitudeValidityField::NormalBodyAccelValid),
      32 => Some(AttitudeValidityField::TurnRateValid),
      64 => Some(AttitudeValidityField::HwFail),
      128 => Some(AttitudeValidityField::MagInvalid),
      256 => Some(AttitudeValidityField::NoGps),
      512 => Some(AttitudeValidityField::GpsInvalid),
      1024 => Some(AttitudeValidityField::SolutionCoasting),
      2048 => Some(AttitudeValidityField::TrueTrackAngle),
      4096 => Some(AttitudeValidityField::MagneticHeading),
      _ => None,
    }
  }
}

/* name: sport_bits_5 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits5Field {
  WaterSkiing = 1,
  Kayaking = 2,
  Rafting = 4,
  Windsurfing = 8,
  Kitesurfing = 16,
  Tactical = 32,
  Jumpmaster = 64,
  Boxing = 128,
}

impl From<SportBits5Field> for u8 {
  fn from(source: SportBits5Field) -> u8 {
    match source {
      SportBits5Field::WaterSkiing => 1,
      SportBits5Field::Kayaking => 2,
      SportBits5Field::Rafting => 4,
      SportBits5Field::Windsurfing => 8,
      SportBits5Field::Kitesurfing => 16,
      SportBits5Field::Tactical => 32,
      SportBits5Field::Jumpmaster => 64,
      SportBits5Field::Boxing => 128,
    }
  }
}

impl IntoField<u8> for SportBits5Field {
  fn into_field(value: u8) -> Option<SportBits5Field> {
    match value {
      1 => Some(SportBits5Field::WaterSkiing),
      2 => Some(SportBits5Field::Kayaking),
      4 => Some(SportBits5Field::Rafting),
      8 => Some(SportBits5Field::Windsurfing),
      16 => Some(SportBits5Field::Kitesurfing),
      32 => Some(SportBits5Field::Tactical),
      64 => Some(SportBits5Field::Jumpmaster),
      128 => Some(SportBits5Field::Boxing),
      _ => None,
    }
  }
}

/* name: wkt_step_target type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WktStepTargetField {
  Speed = 0,
  HeartRate = 1,
  Open = 2,
  Cadence = 3,
  Power = 4,
  Grade = 5,
  Resistance = 6,
  Power3s = 7,
  Power10s = 8,
  Power30s = 9,
  PowerLap = 10,
  SwimStroke = 11,
  SpeedLap = 12,
  HeartRateLap = 13,
}

impl From<WktStepTargetField> for u8 {
  fn from(source: WktStepTargetField) -> u8 {
    match source {
      WktStepTargetField::Speed => 0,
      WktStepTargetField::HeartRate => 1,
      WktStepTargetField::Open => 2,
      WktStepTargetField::Cadence => 3,
      WktStepTargetField::Power => 4,
      WktStepTargetField::Grade => 5,
      WktStepTargetField::Resistance => 6,
      WktStepTargetField::Power3s => 7,
      WktStepTargetField::Power10s => 8,
      WktStepTargetField::Power30s => 9,
      WktStepTargetField::PowerLap => 10,
      WktStepTargetField::SwimStroke => 11,
      WktStepTargetField::SpeedLap => 12,
      WktStepTargetField::HeartRateLap => 13,
    }
  }
}

impl IntoField<u8> for WktStepTargetField {
  fn into_field(value: u8) -> Option<WktStepTargetField> {
    match value {
      0 => Some(WktStepTargetField::Speed),
      1 => Some(WktStepTargetField::HeartRate),
      2 => Some(WktStepTargetField::Open),
      3 => Some(WktStepTargetField::Cadence),
      4 => Some(WktStepTargetField::Power),
      5 => Some(WktStepTargetField::Grade),
      6 => Some(WktStepTargetField::Resistance),
      7 => Some(WktStepTargetField::Power3s),
      8 => Some(WktStepTargetField::Power10s),
      9 => Some(WktStepTargetField::Power30s),
      10 => Some(WktStepTargetField::PowerLap),
      11 => Some(WktStepTargetField::SwimStroke),
      12 => Some(WktStepTargetField::SpeedLap),
      13 => Some(WktStepTargetField::HeartRateLap),
      _ => None,
    }
  }
}

/* name: mesg_num type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum MesgNumField {
  FileId = 0,
  Capabilities = 1,
  DeviceSettings = 2,
  UserProfile = 3,
  HrmProfile = 4,
  SdmProfile = 5,
  BikeProfile = 6,
  ZonesTarget = 7,
  HrZone = 8,
  PowerZone = 9,
  MetZone = 10,
  Sport = 12,
  Goal = 15,
  Session = 18,
  Lap = 19,
  Record = 20,
  Event = 21,
  DeviceInfo = 23,
  Workout = 26,
  WorkoutStep = 27,
  Schedule = 28,
  WeightScale = 30,
  Course = 31,
  CoursePoint = 32,
  Totals = 33,
  Activity = 34,
  Software = 35,
  FileCapabilities = 37,
  MesgCapabilities = 38,
  FieldCapabilities = 39,
  FileCreator = 49,
  BloodPressure = 51,
  SpeedZone = 53,
  Monitoring = 55,
  TrainingFile = 72,
  Hrv = 78,
  AntRx = 80,
  AntTx = 81,
  AntChannelId = 82,
  Length = 101,
  MonitoringInfo = 103,
  Pad = 105,
  SlaveDevice = 106,
  Connectivity = 127,
  WeatherConditions = 128,
  WeatherAlert = 129,
  CadenceZone = 131,
  Hr = 132,
  SegmentLap = 142,
  MemoGlob = 145,
  SegmentId = 148,
  SegmentLeaderboardEntry = 149,
  SegmentPoint = 150,
  SegmentFile = 151,
  WorkoutSession = 158,
  WatchfaceSettings = 159,
  GpsMetadata = 160,
  CameraEvent = 161,
  TimestampCorrelation = 162,
  GyroscopeData = 164,
  AccelerometerData = 165,
  ThreeDSensorCalibration = 167,
  VideoFrame = 169,
  ObdiiData = 174,
  NmeaSentence = 177,
  AviationAttitude = 178,
  Video = 184,
  VideoTitle = 185,
  VideoDescription = 186,
  VideoClip = 187,
  OhrSettings = 188,
  ExdScreenConfiguration = 200,
  ExdDataFieldConfiguration = 201,
  ExdDataConceptConfiguration = 202,
  FieldDescription = 206,
  DeveloperDataId = 207,
  MagnetometerData = 208,
  BarometerData = 209,
  OneDSensorCalibration = 210,
  StressLevel = 227,
  MfgRangeMin = 65280,
  MfgRangeMax = 65534,
}

impl From<MesgNumField> for u16 {
  fn from(source: MesgNumField) -> u16 {
    match source {
      MesgNumField::FileId => 0,
      MesgNumField::Capabilities => 1,
      MesgNumField::DeviceSettings => 2,
      MesgNumField::UserProfile => 3,
      MesgNumField::HrmProfile => 4,
      MesgNumField::SdmProfile => 5,
      MesgNumField::BikeProfile => 6,
      MesgNumField::ZonesTarget => 7,
      MesgNumField::HrZone => 8,
      MesgNumField::PowerZone => 9,
      MesgNumField::MetZone => 10,
      MesgNumField::Sport => 12,
      MesgNumField::Goal => 15,
      MesgNumField::Session => 18,
      MesgNumField::Lap => 19,
      MesgNumField::Record => 20,
      MesgNumField::Event => 21,
      MesgNumField::DeviceInfo => 23,
      MesgNumField::Workout => 26,
      MesgNumField::WorkoutStep => 27,
      MesgNumField::Schedule => 28,
      MesgNumField::WeightScale => 30,
      MesgNumField::Course => 31,
      MesgNumField::CoursePoint => 32,
      MesgNumField::Totals => 33,
      MesgNumField::Activity => 34,
      MesgNumField::Software => 35,
      MesgNumField::FileCapabilities => 37,
      MesgNumField::MesgCapabilities => 38,
      MesgNumField::FieldCapabilities => 39,
      MesgNumField::FileCreator => 49,
      MesgNumField::BloodPressure => 51,
      MesgNumField::SpeedZone => 53,
      MesgNumField::Monitoring => 55,
      MesgNumField::TrainingFile => 72,
      MesgNumField::Hrv => 78,
      MesgNumField::AntRx => 80,
      MesgNumField::AntTx => 81,
      MesgNumField::AntChannelId => 82,
      MesgNumField::Length => 101,
      MesgNumField::MonitoringInfo => 103,
      MesgNumField::Pad => 105,
      MesgNumField::SlaveDevice => 106,
      MesgNumField::Connectivity => 127,
      MesgNumField::WeatherConditions => 128,
      MesgNumField::WeatherAlert => 129,
      MesgNumField::CadenceZone => 131,
      MesgNumField::Hr => 132,
      MesgNumField::SegmentLap => 142,
      MesgNumField::MemoGlob => 145,
      MesgNumField::SegmentId => 148,
      MesgNumField::SegmentLeaderboardEntry => 149,
      MesgNumField::SegmentPoint => 150,
      MesgNumField::SegmentFile => 151,
      MesgNumField::WorkoutSession => 158,
      MesgNumField::WatchfaceSettings => 159,
      MesgNumField::GpsMetadata => 160,
      MesgNumField::CameraEvent => 161,
      MesgNumField::TimestampCorrelation => 162,
      MesgNumField::GyroscopeData => 164,
      MesgNumField::AccelerometerData => 165,
      MesgNumField::ThreeDSensorCalibration => 167,
      MesgNumField::VideoFrame => 169,
      MesgNumField::ObdiiData => 174,
      MesgNumField::NmeaSentence => 177,
      MesgNumField::AviationAttitude => 178,
      MesgNumField::Video => 184,
      MesgNumField::VideoTitle => 185,
      MesgNumField::VideoDescription => 186,
      MesgNumField::VideoClip => 187,
      MesgNumField::OhrSettings => 188,
      MesgNumField::ExdScreenConfiguration => 200,
      MesgNumField::ExdDataFieldConfiguration => 201,
      MesgNumField::ExdDataConceptConfiguration => 202,
      MesgNumField::FieldDescription => 206,
      MesgNumField::DeveloperDataId => 207,
      MesgNumField::MagnetometerData => 208,
      MesgNumField::BarometerData => 209,
      MesgNumField::OneDSensorCalibration => 210,
      MesgNumField::StressLevel => 227,
      MesgNumField::MfgRangeMin => 65280,
      MesgNumField::MfgRangeMax => 65534,
    }
  }
}

impl IntoField<u16> for MesgNumField {
  fn into_field(value: u16) -> Option<MesgNumField> {
    match value {
      0 => Some(MesgNumField::FileId),
      1 => Some(MesgNumField::Capabilities),
      2 => Some(MesgNumField::DeviceSettings),
      3 => Some(MesgNumField::UserProfile),
      4 => Some(MesgNumField::HrmProfile),
      5 => Some(MesgNumField::SdmProfile),
      6 => Some(MesgNumField::BikeProfile),
      7 => Some(MesgNumField::ZonesTarget),
      8 => Some(MesgNumField::HrZone),
      9 => Some(MesgNumField::PowerZone),
      10 => Some(MesgNumField::MetZone),
      12 => Some(MesgNumField::Sport),
      15 => Some(MesgNumField::Goal),
      18 => Some(MesgNumField::Session),
      19 => Some(MesgNumField::Lap),
      20 => Some(MesgNumField::Record),
      21 => Some(MesgNumField::Event),
      23 => Some(MesgNumField::DeviceInfo),
      26 => Some(MesgNumField::Workout),
      27 => Some(MesgNumField::WorkoutStep),
      28 => Some(MesgNumField::Schedule),
      30 => Some(MesgNumField::WeightScale),
      31 => Some(MesgNumField::Course),
      32 => Some(MesgNumField::CoursePoint),
      33 => Some(MesgNumField::Totals),
      34 => Some(MesgNumField::Activity),
      35 => Some(MesgNumField::Software),
      37 => Some(MesgNumField::FileCapabilities),
      38 => Some(MesgNumField::MesgCapabilities),
      39 => Some(MesgNumField::FieldCapabilities),
      49 => Some(MesgNumField::FileCreator),
      51 => Some(MesgNumField::BloodPressure),
      53 => Some(MesgNumField::SpeedZone),
      55 => Some(MesgNumField::Monitoring),
      72 => Some(MesgNumField::TrainingFile),
      78 => Some(MesgNumField::Hrv),
      80 => Some(MesgNumField::AntRx),
      81 => Some(MesgNumField::AntTx),
      82 => Some(MesgNumField::AntChannelId),
      101 => Some(MesgNumField::Length),
      103 => Some(MesgNumField::MonitoringInfo),
      105 => Some(MesgNumField::Pad),
      106 => Some(MesgNumField::SlaveDevice),
      127 => Some(MesgNumField::Connectivity),
      128 => Some(MesgNumField::WeatherConditions),
      129 => Some(MesgNumField::WeatherAlert),
      131 => Some(MesgNumField::CadenceZone),
      132 => Some(MesgNumField::Hr),
      142 => Some(MesgNumField::SegmentLap),
      145 => Some(MesgNumField::MemoGlob),
      148 => Some(MesgNumField::SegmentId),
      149 => Some(MesgNumField::SegmentLeaderboardEntry),
      150 => Some(MesgNumField::SegmentPoint),
      151 => Some(MesgNumField::SegmentFile),
      158 => Some(MesgNumField::WorkoutSession),
      159 => Some(MesgNumField::WatchfaceSettings),
      160 => Some(MesgNumField::GpsMetadata),
      161 => Some(MesgNumField::CameraEvent),
      162 => Some(MesgNumField::TimestampCorrelation),
      164 => Some(MesgNumField::GyroscopeData),
      165 => Some(MesgNumField::AccelerometerData),
      167 => Some(MesgNumField::ThreeDSensorCalibration),
      169 => Some(MesgNumField::VideoFrame),
      174 => Some(MesgNumField::ObdiiData),
      177 => Some(MesgNumField::NmeaSentence),
      178 => Some(MesgNumField::AviationAttitude),
      184 => Some(MesgNumField::Video),
      185 => Some(MesgNumField::VideoTitle),
      186 => Some(MesgNumField::VideoDescription),
      187 => Some(MesgNumField::VideoClip),
      188 => Some(MesgNumField::OhrSettings),
      200 => Some(MesgNumField::ExdScreenConfiguration),
      201 => Some(MesgNumField::ExdDataFieldConfiguration),
      202 => Some(MesgNumField::ExdDataConceptConfiguration),
      206 => Some(MesgNumField::FieldDescription),
      207 => Some(MesgNumField::DeveloperDataId),
      208 => Some(MesgNumField::MagnetometerData),
      209 => Some(MesgNumField::BarometerData),
      210 => Some(MesgNumField::OneDSensorCalibration),
      227 => Some(MesgNumField::StressLevel),
      65280 => Some(MesgNumField::MfgRangeMin),
      65534 => Some(MesgNumField::MfgRangeMax),
      _ => None,
    }
  }
}

/* name: goal_recurrence type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum GoalRecurrenceField {
  Off = 0,
  Daily = 1,
  Weekly = 2,
  Monthly = 3,
  Yearly = 4,
  Custom = 5,
}

impl From<GoalRecurrenceField> for u8 {
  fn from(source: GoalRecurrenceField) -> u8 {
    match source {
      GoalRecurrenceField::Off => 0,
      GoalRecurrenceField::Daily => 1,
      GoalRecurrenceField::Weekly => 2,
      GoalRecurrenceField::Monthly => 3,
      GoalRecurrenceField::Yearly => 4,
      GoalRecurrenceField::Custom => 5,
    }
  }
}

impl IntoField<u8> for GoalRecurrenceField {
  fn into_field(value: u8) -> Option<GoalRecurrenceField> {
    match value {
      0 => Some(GoalRecurrenceField::Off),
      1 => Some(GoalRecurrenceField::Daily),
      2 => Some(GoalRecurrenceField::Weekly),
      3 => Some(GoalRecurrenceField::Monthly),
      4 => Some(GoalRecurrenceField::Yearly),
      5 => Some(GoalRecurrenceField::Custom),
      _ => None,
    }
  }
}

/* name: language_bits_4 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageBits4Field {
  BrazilianPortuguese = 1,
  Indonesian = 2,
  Malaysian = 4,
  Vietnamese = 8,
  Burmese = 16,
  Mongolian = 32,
}

impl From<LanguageBits4Field> for u8 {
  fn from(source: LanguageBits4Field) -> u8 {
    match source {
      LanguageBits4Field::BrazilianPortuguese => 1,
      LanguageBits4Field::Indonesian => 2,
      LanguageBits4Field::Malaysian => 4,
      LanguageBits4Field::Vietnamese => 8,
      LanguageBits4Field::Burmese => 16,
      LanguageBits4Field::Mongolian => 32,
    }
  }
}

impl IntoField<u8> for LanguageBits4Field {
  fn into_field(value: u8) -> Option<LanguageBits4Field> {
    match value {
      1 => Some(LanguageBits4Field::BrazilianPortuguese),
      2 => Some(LanguageBits4Field::Indonesian),
      4 => Some(LanguageBits4Field::Malaysian),
      8 => Some(LanguageBits4Field::Vietnamese),
      16 => Some(LanguageBits4Field::Burmese),
      32 => Some(LanguageBits4Field::Mongolian),
      _ => None,
    }
  }
}

/* name: left_right_balance_100 type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LeftRightBalance100Field {
  Mask = 16383,
  Right = 32768,
}

impl From<LeftRightBalance100Field> for u16 {
  fn from(source: LeftRightBalance100Field) -> u16 {
    match source {
      LeftRightBalance100Field::Mask => 16383,
      LeftRightBalance100Field::Right => 32768,
    }
  }
}

impl IntoField<u16> for LeftRightBalance100Field {
  fn into_field(value: u16) -> Option<LeftRightBalance100Field> {
    match value {
      16383 => Some(LeftRightBalance100Field::Mask),
      32768 => Some(LeftRightBalance100Field::Right),
      _ => None,
    }
  }
}

/* name: connectivity_capabilities type: uint32z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ConnectivityCapabilitiesField {
  Bluetooth = 1,
  BluetoothLe = 2,
  Ant = 4,
  ActivityUpload = 8,
  CourseDownload = 16,
  WorkoutDownload = 32,
  LiveTrack = 64,
  WeatherConditions = 128,
  WeatherAlerts = 256,
  GpsEphemerisDownload = 512,
  ExplicitArchive = 1024,
  SetupIncomplete = 2048,
  ContinueSyncAfterSoftwareUpdate = 4096,
  ConnectIqAppDownload = 8192,
  GolfCourseDownload = 16384,
  DeviceInitiatesSync = 32768,
  ConnectIqWatchAppDownload = 65536,
  ConnectIqWidgetDownload = 131072,
  ConnectIqWatchFaceDownload = 262144,
  ConnectIqDataFieldDownload = 524288,
  ConnectIqAppManagment = 1048576,
  SwingSensor = 2097152,
  SwingSensorRemote = 4194304,
  IncidentDetection = 8388608,
  AudioPrompts = 16777216,
  WifiVerification = 33554432,
  TrueUp = 67108864,
  FindMyWatch = 134217728,
  RemoteManualSync = 268435456,
  LiveTrackAutoStart = 536870912,
  LiveTrackMessaging = 1073741824,
  InstantInput = 2147483648,
}

impl From<ConnectivityCapabilitiesField> for u32 {
  fn from(source: ConnectivityCapabilitiesField) -> u32 {
    match source {
      ConnectivityCapabilitiesField::Bluetooth => 1,
      ConnectivityCapabilitiesField::BluetoothLe => 2,
      ConnectivityCapabilitiesField::Ant => 4,
      ConnectivityCapabilitiesField::ActivityUpload => 8,
      ConnectivityCapabilitiesField::CourseDownload => 16,
      ConnectivityCapabilitiesField::WorkoutDownload => 32,
      ConnectivityCapabilitiesField::LiveTrack => 64,
      ConnectivityCapabilitiesField::WeatherConditions => 128,
      ConnectivityCapabilitiesField::WeatherAlerts => 256,
      ConnectivityCapabilitiesField::GpsEphemerisDownload => 512,
      ConnectivityCapabilitiesField::ExplicitArchive => 1024,
      ConnectivityCapabilitiesField::SetupIncomplete => 2048,
      ConnectivityCapabilitiesField::ContinueSyncAfterSoftwareUpdate => 4096,
      ConnectivityCapabilitiesField::ConnectIqAppDownload => 8192,
      ConnectivityCapabilitiesField::GolfCourseDownload => 16384,
      ConnectivityCapabilitiesField::DeviceInitiatesSync => 32768,
      ConnectivityCapabilitiesField::ConnectIqWatchAppDownload => 65536,
      ConnectivityCapabilitiesField::ConnectIqWidgetDownload => 131072,
      ConnectivityCapabilitiesField::ConnectIqWatchFaceDownload => 262144,
      ConnectivityCapabilitiesField::ConnectIqDataFieldDownload => 524288,
      ConnectivityCapabilitiesField::ConnectIqAppManagment => 1048576,
      ConnectivityCapabilitiesField::SwingSensor => 2097152,
      ConnectivityCapabilitiesField::SwingSensorRemote => 4194304,
      ConnectivityCapabilitiesField::IncidentDetection => 8388608,
      ConnectivityCapabilitiesField::AudioPrompts => 16777216,
      ConnectivityCapabilitiesField::WifiVerification => 33554432,
      ConnectivityCapabilitiesField::TrueUp => 67108864,
      ConnectivityCapabilitiesField::FindMyWatch => 134217728,
      ConnectivityCapabilitiesField::RemoteManualSync => 268435456,
      ConnectivityCapabilitiesField::LiveTrackAutoStart => 536870912,
      ConnectivityCapabilitiesField::LiveTrackMessaging => 1073741824,
      ConnectivityCapabilitiesField::InstantInput => 2147483648,
    }
  }
}

impl IntoField<u32> for ConnectivityCapabilitiesField {
  fn into_field(value: u32) -> Option<ConnectivityCapabilitiesField> {
    match value {
      1 => Some(ConnectivityCapabilitiesField::Bluetooth),
      2 => Some(ConnectivityCapabilitiesField::BluetoothLe),
      4 => Some(ConnectivityCapabilitiesField::Ant),
      8 => Some(ConnectivityCapabilitiesField::ActivityUpload),
      16 => Some(ConnectivityCapabilitiesField::CourseDownload),
      32 => Some(ConnectivityCapabilitiesField::WorkoutDownload),
      64 => Some(ConnectivityCapabilitiesField::LiveTrack),
      128 => Some(ConnectivityCapabilitiesField::WeatherConditions),
      256 => Some(ConnectivityCapabilitiesField::WeatherAlerts),
      512 => Some(ConnectivityCapabilitiesField::GpsEphemerisDownload),
      1024 => Some(ConnectivityCapabilitiesField::ExplicitArchive),
      2048 => Some(ConnectivityCapabilitiesField::SetupIncomplete),
      4096 => Some(ConnectivityCapabilitiesField::ContinueSyncAfterSoftwareUpdate),
      8192 => Some(ConnectivityCapabilitiesField::ConnectIqAppDownload),
      16384 => Some(ConnectivityCapabilitiesField::GolfCourseDownload),
      32768 => Some(ConnectivityCapabilitiesField::DeviceInitiatesSync),
      65536 => Some(ConnectivityCapabilitiesField::ConnectIqWatchAppDownload),
      131072 => Some(ConnectivityCapabilitiesField::ConnectIqWidgetDownload),
      262144 => Some(ConnectivityCapabilitiesField::ConnectIqWatchFaceDownload),
      524288 => Some(ConnectivityCapabilitiesField::ConnectIqDataFieldDownload),
      1048576 => Some(ConnectivityCapabilitiesField::ConnectIqAppManagment),
      2097152 => Some(ConnectivityCapabilitiesField::SwingSensor),
      4194304 => Some(ConnectivityCapabilitiesField::SwingSensorRemote),
      8388608 => Some(ConnectivityCapabilitiesField::IncidentDetection),
      16777216 => Some(ConnectivityCapabilitiesField::AudioPrompts),
      33554432 => Some(ConnectivityCapabilitiesField::WifiVerification),
      67108864 => Some(ConnectivityCapabilitiesField::TrueUp),
      134217728 => Some(ConnectivityCapabilitiesField::FindMyWatch),
      268435456 => Some(ConnectivityCapabilitiesField::RemoteManualSync),
      536870912 => Some(ConnectivityCapabilitiesField::LiveTrackAutoStart),
      1073741824 => Some(ConnectivityCapabilitiesField::LiveTrackMessaging),
      2147483648 => Some(ConnectivityCapabilitiesField::InstantInput),
      _ => None,
    }
  }
}

/* name: display_position type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DisplayPositionField {
  Degree = 0,
  DegreeMinute = 1,
  DegreeMinuteSecond = 2,
  AustrianGrid = 3,
  BritishGrid = 4,
  DutchGrid = 5,
  HungarianGrid = 6,
  FinnishGrid = 7,
  GermanGrid = 8,
  IcelandicGrid = 9,
  IndonesianEquatorial = 10,
  IndonesianIrian = 11,
  IndonesianSouthern = 12,
  IndiaZone0 = 13,
  IndiaZoneIa = 14,
  IndiaZoneIb = 15,
  IndiaZoneIia = 16,
  IndiaZoneIib = 17,
  IndiaZoneIiia = 18,
  IndiaZoneIiib = 19,
  IndiaZoneIva = 20,
  IndiaZoneIvb = 21,
  IrishTransverse = 22,
  IrishGrid = 23,
  Loran = 24,
  MaidenheadGrid = 25,
  MgrsGrid = 26,
  NewZealandGrid = 27,
  NewZealandTransverse = 28,
  QatarGrid = 29,
  ModifiedSwedishGrid = 30,
  SwedishGrid = 31,
  SouthAfricanGrid = 32,
  SwissGrid = 33,
  TaiwanGrid = 34,
  UnitedStatesGrid = 35,
  UtmUpsGrid = 36,
  WestMalayan = 37,
  BorneoRso = 38,
  EstonianGrid = 39,
  LatvianGrid = 40,
  SwedishRef99Grid = 41,
}

impl From<DisplayPositionField> for u8 {
  fn from(source: DisplayPositionField) -> u8 {
    match source {
      DisplayPositionField::Degree => 0,
      DisplayPositionField::DegreeMinute => 1,
      DisplayPositionField::DegreeMinuteSecond => 2,
      DisplayPositionField::AustrianGrid => 3,
      DisplayPositionField::BritishGrid => 4,
      DisplayPositionField::DutchGrid => 5,
      DisplayPositionField::HungarianGrid => 6,
      DisplayPositionField::FinnishGrid => 7,
      DisplayPositionField::GermanGrid => 8,
      DisplayPositionField::IcelandicGrid => 9,
      DisplayPositionField::IndonesianEquatorial => 10,
      DisplayPositionField::IndonesianIrian => 11,
      DisplayPositionField::IndonesianSouthern => 12,
      DisplayPositionField::IndiaZone0 => 13,
      DisplayPositionField::IndiaZoneIa => 14,
      DisplayPositionField::IndiaZoneIb => 15,
      DisplayPositionField::IndiaZoneIia => 16,
      DisplayPositionField::IndiaZoneIib => 17,
      DisplayPositionField::IndiaZoneIiia => 18,
      DisplayPositionField::IndiaZoneIiib => 19,
      DisplayPositionField::IndiaZoneIva => 20,
      DisplayPositionField::IndiaZoneIvb => 21,
      DisplayPositionField::IrishTransverse => 22,
      DisplayPositionField::IrishGrid => 23,
      DisplayPositionField::Loran => 24,
      DisplayPositionField::MaidenheadGrid => 25,
      DisplayPositionField::MgrsGrid => 26,
      DisplayPositionField::NewZealandGrid => 27,
      DisplayPositionField::NewZealandTransverse => 28,
      DisplayPositionField::QatarGrid => 29,
      DisplayPositionField::ModifiedSwedishGrid => 30,
      DisplayPositionField::SwedishGrid => 31,
      DisplayPositionField::SouthAfricanGrid => 32,
      DisplayPositionField::SwissGrid => 33,
      DisplayPositionField::TaiwanGrid => 34,
      DisplayPositionField::UnitedStatesGrid => 35,
      DisplayPositionField::UtmUpsGrid => 36,
      DisplayPositionField::WestMalayan => 37,
      DisplayPositionField::BorneoRso => 38,
      DisplayPositionField::EstonianGrid => 39,
      DisplayPositionField::LatvianGrid => 40,
      DisplayPositionField::SwedishRef99Grid => 41,
    }
  }
}

impl IntoField<u8> for DisplayPositionField {
  fn into_field(value: u8) -> Option<DisplayPositionField> {
    match value {
      0 => Some(DisplayPositionField::Degree),
      1 => Some(DisplayPositionField::DegreeMinute),
      2 => Some(DisplayPositionField::DegreeMinuteSecond),
      3 => Some(DisplayPositionField::AustrianGrid),
      4 => Some(DisplayPositionField::BritishGrid),
      5 => Some(DisplayPositionField::DutchGrid),
      6 => Some(DisplayPositionField::HungarianGrid),
      7 => Some(DisplayPositionField::FinnishGrid),
      8 => Some(DisplayPositionField::GermanGrid),
      9 => Some(DisplayPositionField::IcelandicGrid),
      10 => Some(DisplayPositionField::IndonesianEquatorial),
      11 => Some(DisplayPositionField::IndonesianIrian),
      12 => Some(DisplayPositionField::IndonesianSouthern),
      13 => Some(DisplayPositionField::IndiaZone0),
      14 => Some(DisplayPositionField::IndiaZoneIa),
      15 => Some(DisplayPositionField::IndiaZoneIb),
      16 => Some(DisplayPositionField::IndiaZoneIia),
      17 => Some(DisplayPositionField::IndiaZoneIib),
      18 => Some(DisplayPositionField::IndiaZoneIiia),
      19 => Some(DisplayPositionField::IndiaZoneIiib),
      20 => Some(DisplayPositionField::IndiaZoneIva),
      21 => Some(DisplayPositionField::IndiaZoneIvb),
      22 => Some(DisplayPositionField::IrishTransverse),
      23 => Some(DisplayPositionField::IrishGrid),
      24 => Some(DisplayPositionField::Loran),
      25 => Some(DisplayPositionField::MaidenheadGrid),
      26 => Some(DisplayPositionField::MgrsGrid),
      27 => Some(DisplayPositionField::NewZealandGrid),
      28 => Some(DisplayPositionField::NewZealandTransverse),
      29 => Some(DisplayPositionField::QatarGrid),
      30 => Some(DisplayPositionField::ModifiedSwedishGrid),
      31 => Some(DisplayPositionField::SwedishGrid),
      32 => Some(DisplayPositionField::SouthAfricanGrid),
      33 => Some(DisplayPositionField::SwissGrid),
      34 => Some(DisplayPositionField::TaiwanGrid),
      35 => Some(DisplayPositionField::UnitedStatesGrid),
      36 => Some(DisplayPositionField::UtmUpsGrid),
      37 => Some(DisplayPositionField::WestMalayan),
      38 => Some(DisplayPositionField::BorneoRso),
      39 => Some(DisplayPositionField::EstonianGrid),
      40 => Some(DisplayPositionField::LatvianGrid),
      41 => Some(DisplayPositionField::SwedishRef99Grid),
      _ => None,
    }
  }
}

/* name: switch type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SwitchField {
  Off = 0,
  On = 1,
  Auto = 2,
}

impl From<SwitchField> for u8 {
  fn from(source: SwitchField) -> u8 {
    match source {
      SwitchField::Off => 0,
      SwitchField::On => 1,
      SwitchField::Auto => 2,
    }
  }
}

impl IntoField<u8> for SwitchField {
  fn into_field(value: u8) -> Option<SwitchField> {
    match value {
      0 => Some(SwitchField::Off),
      1 => Some(SwitchField::On),
      2 => Some(SwitchField::Auto),
      _ => None,
    }
  }
}

/* name: camera_event_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum CameraEventTypeField {
  VideoStart = 0,
  VideoSplit = 1,
  VideoEnd = 2,
  PhotoTaken = 3,
  VideoSecondStreamStart = 4,
  VideoSecondStreamSplit = 5,
  VideoSecondStreamEnd = 6,
  VideoSplitStart = 7,
  VideoSecondStreamSplitStart = 8,
  VideoPause = 11,
  VideoSecondStreamPause = 12,
  VideoResume = 13,
  VideoSecondStreamResume = 14,
}

impl From<CameraEventTypeField> for u8 {
  fn from(source: CameraEventTypeField) -> u8 {
    match source {
      CameraEventTypeField::VideoStart => 0,
      CameraEventTypeField::VideoSplit => 1,
      CameraEventTypeField::VideoEnd => 2,
      CameraEventTypeField::PhotoTaken => 3,
      CameraEventTypeField::VideoSecondStreamStart => 4,
      CameraEventTypeField::VideoSecondStreamSplit => 5,
      CameraEventTypeField::VideoSecondStreamEnd => 6,
      CameraEventTypeField::VideoSplitStart => 7,
      CameraEventTypeField::VideoSecondStreamSplitStart => 8,
      CameraEventTypeField::VideoPause => 11,
      CameraEventTypeField::VideoSecondStreamPause => 12,
      CameraEventTypeField::VideoResume => 13,
      CameraEventTypeField::VideoSecondStreamResume => 14,
    }
  }
}

impl IntoField<u8> for CameraEventTypeField {
  fn into_field(value: u8) -> Option<CameraEventTypeField> {
    match value {
      0 => Some(CameraEventTypeField::VideoStart),
      1 => Some(CameraEventTypeField::VideoSplit),
      2 => Some(CameraEventTypeField::VideoEnd),
      3 => Some(CameraEventTypeField::PhotoTaken),
      4 => Some(CameraEventTypeField::VideoSecondStreamStart),
      5 => Some(CameraEventTypeField::VideoSecondStreamSplit),
      6 => Some(CameraEventTypeField::VideoSecondStreamEnd),
      7 => Some(CameraEventTypeField::VideoSplitStart),
      8 => Some(CameraEventTypeField::VideoSecondStreamSplitStart),
      11 => Some(CameraEventTypeField::VideoPause),
      12 => Some(CameraEventTypeField::VideoSecondStreamPause),
      13 => Some(CameraEventTypeField::VideoResume),
      14 => Some(CameraEventTypeField::VideoSecondStreamResume),
      _ => None,
    }
  }
}

/* name: bike_light_network_config_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BikeLightNetworkConfigTypeField {
  Auto = 0,
  Individual = 4,
  HighVisibility = 5,
  Trail = 6,
}

impl From<BikeLightNetworkConfigTypeField> for u8 {
  fn from(source: BikeLightNetworkConfigTypeField) -> u8 {
    match source {
      BikeLightNetworkConfigTypeField::Auto => 0,
      BikeLightNetworkConfigTypeField::Individual => 4,
      BikeLightNetworkConfigTypeField::HighVisibility => 5,
      BikeLightNetworkConfigTypeField::Trail => 6,
    }
  }
}

impl IntoField<u8> for BikeLightNetworkConfigTypeField {
  fn into_field(value: u8) -> Option<BikeLightNetworkConfigTypeField> {
    match value {
      0 => Some(BikeLightNetworkConfigTypeField::Auto),
      4 => Some(BikeLightNetworkConfigTypeField::Individual),
      5 => Some(BikeLightNetworkConfigTypeField::HighVisibility),
      6 => Some(BikeLightNetworkConfigTypeField::Trail),
      _ => None,
    }
  }
}

/* name: goal type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum GoalField {
  Time = 0,
  Distance = 1,
  Calories = 2,
  Frequency = 3,
  Steps = 4,
  Ascent = 5,
  ActiveMinutes = 6,
}

impl From<GoalField> for u8 {
  fn from(source: GoalField) -> u8 {
    match source {
      GoalField::Time => 0,
      GoalField::Distance => 1,
      GoalField::Calories => 2,
      GoalField::Frequency => 3,
      GoalField::Steps => 4,
      GoalField::Ascent => 5,
      GoalField::ActiveMinutes => 6,
    }
  }
}

impl IntoField<u8> for GoalField {
  fn into_field(value: u8) -> Option<GoalField> {
    match value {
      0 => Some(GoalField::Time),
      1 => Some(GoalField::Distance),
      2 => Some(GoalField::Calories),
      3 => Some(GoalField::Frequency),
      4 => Some(GoalField::Steps),
      5 => Some(GoalField::Ascent),
      6 => Some(GoalField::ActiveMinutes),
      _ => None,
    }
  }
}

/* name: gender type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum GenderField {
  Female = 0,
  Male = 1,
}

impl From<GenderField> for u8 {
  fn from(source: GenderField) -> u8 {
    match source {
      GenderField::Female => 0,
      GenderField::Male => 1,
    }
  }
}

impl IntoField<u8> for GenderField {
  fn into_field(value: u8) -> Option<GenderField> {
    match value {
      0 => Some(GenderField::Female),
      1 => Some(GenderField::Male),
      _ => None,
    }
  }
}

/* name: auto_sync_frequency type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AutoSyncFrequencyField {
  Never = 0,
  Occasionally = 1,
  Frequent = 2,
  OnceADay = 3,
  Remote = 4,
}

impl From<AutoSyncFrequencyField> for u8 {
  fn from(source: AutoSyncFrequencyField) -> u8 {
    match source {
      AutoSyncFrequencyField::Never => 0,
      AutoSyncFrequencyField::Occasionally => 1,
      AutoSyncFrequencyField::Frequent => 2,
      AutoSyncFrequencyField::OnceADay => 3,
      AutoSyncFrequencyField::Remote => 4,
    }
  }
}

impl IntoField<u8> for AutoSyncFrequencyField {
  fn into_field(value: u8) -> Option<AutoSyncFrequencyField> {
    match value {
      0 => Some(AutoSyncFrequencyField::Never),
      1 => Some(AutoSyncFrequencyField::Occasionally),
      2 => Some(AutoSyncFrequencyField::Frequent),
      3 => Some(AutoSyncFrequencyField::OnceADay),
      4 => Some(AutoSyncFrequencyField::Remote),
      _ => None,
    }
  }
}

/* name: event type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum EventField {
  Timer = 0,
  Workout = 3,
  WorkoutStep = 4,
  PowerDown = 5,
  PowerUp = 6,
  OffCourse = 7,
  Session = 8,
  Lap = 9,
  CoursePoint = 10,
  Battery = 11,
  VirtualPartnerPace = 12,
  HrHighAlert = 13,
  HrLowAlert = 14,
  SpeedHighAlert = 15,
  SpeedLowAlert = 16,
  CadHighAlert = 17,
  CadLowAlert = 18,
  PowerHighAlert = 19,
  PowerLowAlert = 20,
  RecoveryHr = 21,
  BatteryLow = 22,
  TimeDurationAlert = 23,
  DistanceDurationAlert = 24,
  CalorieDurationAlert = 25,
  Activity = 26,
  FitnessEquipment = 27,
  Length = 28,
  UserMarker = 32,
  SportPoint = 33,
  Calibration = 36,
  FrontGearChange = 42,
  RearGearChange = 43,
  RiderPositionChange = 44,
  ElevHighAlert = 45,
  ElevLowAlert = 46,
  CommTimeout = 47,
}

impl From<EventField> for u8 {
  fn from(source: EventField) -> u8 {
    match source {
      EventField::Timer => 0,
      EventField::Workout => 3,
      EventField::WorkoutStep => 4,
      EventField::PowerDown => 5,
      EventField::PowerUp => 6,
      EventField::OffCourse => 7,
      EventField::Session => 8,
      EventField::Lap => 9,
      EventField::CoursePoint => 10,
      EventField::Battery => 11,
      EventField::VirtualPartnerPace => 12,
      EventField::HrHighAlert => 13,
      EventField::HrLowAlert => 14,
      EventField::SpeedHighAlert => 15,
      EventField::SpeedLowAlert => 16,
      EventField::CadHighAlert => 17,
      EventField::CadLowAlert => 18,
      EventField::PowerHighAlert => 19,
      EventField::PowerLowAlert => 20,
      EventField::RecoveryHr => 21,
      EventField::BatteryLow => 22,
      EventField::TimeDurationAlert => 23,
      EventField::DistanceDurationAlert => 24,
      EventField::CalorieDurationAlert => 25,
      EventField::Activity => 26,
      EventField::FitnessEquipment => 27,
      EventField::Length => 28,
      EventField::UserMarker => 32,
      EventField::SportPoint => 33,
      EventField::Calibration => 36,
      EventField::FrontGearChange => 42,
      EventField::RearGearChange => 43,
      EventField::RiderPositionChange => 44,
      EventField::ElevHighAlert => 45,
      EventField::ElevLowAlert => 46,
      EventField::CommTimeout => 47,
    }
  }
}

impl IntoField<u8> for EventField {
  fn into_field(value: u8) -> Option<EventField> {
    match value {
      0 => Some(EventField::Timer),
      3 => Some(EventField::Workout),
      4 => Some(EventField::WorkoutStep),
      5 => Some(EventField::PowerDown),
      6 => Some(EventField::PowerUp),
      7 => Some(EventField::OffCourse),
      8 => Some(EventField::Session),
      9 => Some(EventField::Lap),
      10 => Some(EventField::CoursePoint),
      11 => Some(EventField::Battery),
      12 => Some(EventField::VirtualPartnerPace),
      13 => Some(EventField::HrHighAlert),
      14 => Some(EventField::HrLowAlert),
      15 => Some(EventField::SpeedHighAlert),
      16 => Some(EventField::SpeedLowAlert),
      17 => Some(EventField::CadHighAlert),
      18 => Some(EventField::CadLowAlert),
      19 => Some(EventField::PowerHighAlert),
      20 => Some(EventField::PowerLowAlert),
      21 => Some(EventField::RecoveryHr),
      22 => Some(EventField::BatteryLow),
      23 => Some(EventField::TimeDurationAlert),
      24 => Some(EventField::DistanceDurationAlert),
      25 => Some(EventField::CalorieDurationAlert),
      26 => Some(EventField::Activity),
      27 => Some(EventField::FitnessEquipment),
      28 => Some(EventField::Length),
      32 => Some(EventField::UserMarker),
      33 => Some(EventField::SportPoint),
      36 => Some(EventField::Calibration),
      42 => Some(EventField::FrontGearChange),
      43 => Some(EventField::RearGearChange),
      44 => Some(EventField::RiderPositionChange),
      45 => Some(EventField::ElevHighAlert),
      46 => Some(EventField::ElevLowAlert),
      47 => Some(EventField::CommTimeout),
      _ => None,
    }
  }
}

/* name: antplus_device_type type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AntplusDeviceTypeField {
  Antfs = 1,
  BikePower = 11,
  EnvironmentSensorLegacy = 12,
  MultiSportSpeedDistance = 15,
  Control = 16,
  FitnessEquipment = 17,
  BloodPressure = 18,
  GeocacheNode = 19,
  LightElectricVehicle = 20,
  EnvSensor = 25,
  Racquet = 26,
  ControlHub = 27,
  MuscleOxygen = 31,
  BikeLightMain = 35,
  BikeLightShared = 36,
  Exd = 38,
  BikeRadar = 40,
  WeightScale = 119,
  HeartRate = 120,
  BikeSpeedCadence = 121,
  BikeCadence = 122,
  BikeSpeed = 123,
  StrideSpeedDistance = 124,
}

impl From<AntplusDeviceTypeField> for u8 {
  fn from(source: AntplusDeviceTypeField) -> u8 {
    match source {
      AntplusDeviceTypeField::Antfs => 1,
      AntplusDeviceTypeField::BikePower => 11,
      AntplusDeviceTypeField::EnvironmentSensorLegacy => 12,
      AntplusDeviceTypeField::MultiSportSpeedDistance => 15,
      AntplusDeviceTypeField::Control => 16,
      AntplusDeviceTypeField::FitnessEquipment => 17,
      AntplusDeviceTypeField::BloodPressure => 18,
      AntplusDeviceTypeField::GeocacheNode => 19,
      AntplusDeviceTypeField::LightElectricVehicle => 20,
      AntplusDeviceTypeField::EnvSensor => 25,
      AntplusDeviceTypeField::Racquet => 26,
      AntplusDeviceTypeField::ControlHub => 27,
      AntplusDeviceTypeField::MuscleOxygen => 31,
      AntplusDeviceTypeField::BikeLightMain => 35,
      AntplusDeviceTypeField::BikeLightShared => 36,
      AntplusDeviceTypeField::Exd => 38,
      AntplusDeviceTypeField::BikeRadar => 40,
      AntplusDeviceTypeField::WeightScale => 119,
      AntplusDeviceTypeField::HeartRate => 120,
      AntplusDeviceTypeField::BikeSpeedCadence => 121,
      AntplusDeviceTypeField::BikeCadence => 122,
      AntplusDeviceTypeField::BikeSpeed => 123,
      AntplusDeviceTypeField::StrideSpeedDistance => 124,
    }
  }
}

impl IntoField<u8> for AntplusDeviceTypeField {
  fn into_field(value: u8) -> Option<AntplusDeviceTypeField> {
    match value {
      1 => Some(AntplusDeviceTypeField::Antfs),
      11 => Some(AntplusDeviceTypeField::BikePower),
      12 => Some(AntplusDeviceTypeField::EnvironmentSensorLegacy),
      15 => Some(AntplusDeviceTypeField::MultiSportSpeedDistance),
      16 => Some(AntplusDeviceTypeField::Control),
      17 => Some(AntplusDeviceTypeField::FitnessEquipment),
      18 => Some(AntplusDeviceTypeField::BloodPressure),
      19 => Some(AntplusDeviceTypeField::GeocacheNode),
      20 => Some(AntplusDeviceTypeField::LightElectricVehicle),
      25 => Some(AntplusDeviceTypeField::EnvSensor),
      26 => Some(AntplusDeviceTypeField::Racquet),
      27 => Some(AntplusDeviceTypeField::ControlHub),
      31 => Some(AntplusDeviceTypeField::MuscleOxygen),
      35 => Some(AntplusDeviceTypeField::BikeLightMain),
      36 => Some(AntplusDeviceTypeField::BikeLightShared),
      38 => Some(AntplusDeviceTypeField::Exd),
      40 => Some(AntplusDeviceTypeField::BikeRadar),
      119 => Some(AntplusDeviceTypeField::WeightScale),
      120 => Some(AntplusDeviceTypeField::HeartRate),
      121 => Some(AntplusDeviceTypeField::BikeSpeedCadence),
      122 => Some(AntplusDeviceTypeField::BikeCadence),
      123 => Some(AntplusDeviceTypeField::BikeSpeed),
      124 => Some(AntplusDeviceTypeField::StrideSpeedDistance),
      _ => None,
    }
  }
}

/* name: activity_level type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ActivityLevelField {
  Low = 0,
  Medium = 1,
  High = 2,
}

impl From<ActivityLevelField> for u8 {
  fn from(source: ActivityLevelField) -> u8 {
    match source {
      ActivityLevelField::Low => 0,
      ActivityLevelField::Medium => 1,
      ActivityLevelField::High => 2,
    }
  }
}

impl IntoField<u8> for ActivityLevelField {
  fn into_field(value: u8) -> Option<ActivityLevelField> {
    match value {
      0 => Some(ActivityLevelField::Low),
      1 => Some(ActivityLevelField::Medium),
      2 => Some(ActivityLevelField::High),
      _ => None,
    }
  }
}

/* name: course_point type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum CoursePointField {
  Generic = 0,
  Summit = 1,
  Valley = 2,
  Water = 3,
  Food = 4,
  Danger = 5,
  Left = 6,
  Right = 7,
  Straight = 8,
  FirstAid = 9,
  FourthCategory = 10,
  ThirdCategory = 11,
  SecondCategory = 12,
  FirstCategory = 13,
  HorsCategory = 14,
  Sprint = 15,
  LeftFork = 16,
  RightFork = 17,
  MiddleFork = 18,
  SlightLeft = 19,
  SharpLeft = 20,
  SlightRight = 21,
  SharpRight = 22,
  UTurn = 23,
  SegmentStart = 24,
  SegmentEnd = 25,
}

impl From<CoursePointField> for u8 {
  fn from(source: CoursePointField) -> u8 {
    match source {
      CoursePointField::Generic => 0,
      CoursePointField::Summit => 1,
      CoursePointField::Valley => 2,
      CoursePointField::Water => 3,
      CoursePointField::Food => 4,
      CoursePointField::Danger => 5,
      CoursePointField::Left => 6,
      CoursePointField::Right => 7,
      CoursePointField::Straight => 8,
      CoursePointField::FirstAid => 9,
      CoursePointField::FourthCategory => 10,
      CoursePointField::ThirdCategory => 11,
      CoursePointField::SecondCategory => 12,
      CoursePointField::FirstCategory => 13,
      CoursePointField::HorsCategory => 14,
      CoursePointField::Sprint => 15,
      CoursePointField::LeftFork => 16,
      CoursePointField::RightFork => 17,
      CoursePointField::MiddleFork => 18,
      CoursePointField::SlightLeft => 19,
      CoursePointField::SharpLeft => 20,
      CoursePointField::SlightRight => 21,
      CoursePointField::SharpRight => 22,
      CoursePointField::UTurn => 23,
      CoursePointField::SegmentStart => 24,
      CoursePointField::SegmentEnd => 25,
    }
  }
}

impl IntoField<u8> for CoursePointField {
  fn into_field(value: u8) -> Option<CoursePointField> {
    match value {
      0 => Some(CoursePointField::Generic),
      1 => Some(CoursePointField::Summit),
      2 => Some(CoursePointField::Valley),
      3 => Some(CoursePointField::Water),
      4 => Some(CoursePointField::Food),
      5 => Some(CoursePointField::Danger),
      6 => Some(CoursePointField::Left),
      7 => Some(CoursePointField::Right),
      8 => Some(CoursePointField::Straight),
      9 => Some(CoursePointField::FirstAid),
      10 => Some(CoursePointField::FourthCategory),
      11 => Some(CoursePointField::ThirdCategory),
      12 => Some(CoursePointField::SecondCategory),
      13 => Some(CoursePointField::FirstCategory),
      14 => Some(CoursePointField::HorsCategory),
      15 => Some(CoursePointField::Sprint),
      16 => Some(CoursePointField::LeftFork),
      17 => Some(CoursePointField::RightFork),
      18 => Some(CoursePointField::MiddleFork),
      19 => Some(CoursePointField::SlightLeft),
      20 => Some(CoursePointField::SharpLeft),
      21 => Some(CoursePointField::SlightRight),
      22 => Some(CoursePointField::SharpRight),
      23 => Some(CoursePointField::UTurn),
      24 => Some(CoursePointField::SegmentStart),
      25 => Some(CoursePointField::SegmentEnd),
      _ => None,
    }
  }
}

/* name: file_flags type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum FileFlagsField {
  Read = 2,
  Write = 4,
  Erase = 8,
}

impl From<FileFlagsField> for u8 {
  fn from(source: FileFlagsField) -> u8 {
    match source {
      FileFlagsField::Read => 2,
      FileFlagsField::Write => 4,
      FileFlagsField::Erase => 8,
    }
  }
}

impl IntoField<u8> for FileFlagsField {
  fn into_field(value: u8) -> Option<FileFlagsField> {
    match value {
      2 => Some(FileFlagsField::Read),
      4 => Some(FileFlagsField::Write),
      8 => Some(FileFlagsField::Erase),
      _ => None,
    }
  }
}

/* name: mesg_count type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum MesgCountField {
  NumPerFile = 0,
  MaxPerFile = 1,
  MaxPerFileType = 2,
}

impl From<MesgCountField> for u8 {
  fn from(source: MesgCountField) -> u8 {
    match source {
      MesgCountField::NumPerFile => 0,
      MesgCountField::MaxPerFile => 1,
      MesgCountField::MaxPerFileType => 2,
    }
  }
}

impl IntoField<u8> for MesgCountField {
  fn into_field(value: u8) -> Option<MesgCountField> {
    match value {
      0 => Some(MesgCountField::NumPerFile),
      1 => Some(MesgCountField::MaxPerFile),
      2 => Some(MesgCountField::MaxPerFileType),
      _ => None,
    }
  }
}

/* name: sport_event type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportEventField {
  Uncategorized = 0,
  Geocaching = 1,
  Fitness = 2,
  Recreation = 3,
  Race = 4,
  SpecialEvent = 5,
  Training = 6,
  Transportation = 7,
  Touring = 8,
}

impl From<SportEventField> for u8 {
  fn from(source: SportEventField) -> u8 {
    match source {
      SportEventField::Uncategorized => 0,
      SportEventField::Geocaching => 1,
      SportEventField::Fitness => 2,
      SportEventField::Recreation => 3,
      SportEventField::Race => 4,
      SportEventField::SpecialEvent => 5,
      SportEventField::Training => 6,
      SportEventField::Transportation => 7,
      SportEventField::Touring => 8,
    }
  }
}

impl IntoField<u8> for SportEventField {
  fn into_field(value: u8) -> Option<SportEventField> {
    match value {
      0 => Some(SportEventField::Uncategorized),
      1 => Some(SportEventField::Geocaching),
      2 => Some(SportEventField::Fitness),
      3 => Some(SportEventField::Recreation),
      4 => Some(SportEventField::Race),
      5 => Some(SportEventField::SpecialEvent),
      6 => Some(SportEventField::Training),
      7 => Some(SportEventField::Transportation),
      8 => Some(SportEventField::Touring),
      _ => None,
    }
  }
}

/* name: language type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageField {
  English = 0,
  French = 1,
  Italian = 2,
  German = 3,
  Spanish = 4,
  Croatian = 5,
  Czech = 6,
  Danish = 7,
  Dutch = 8,
  Finnish = 9,
  Greek = 10,
  Hungarian = 11,
  Norwegian = 12,
  Polish = 13,
  Portuguese = 14,
  Slovakian = 15,
  Slovenian = 16,
  Swedish = 17,
  Russian = 18,
  Turkish = 19,
  Latvian = 20,
  Ukrainian = 21,
  Arabic = 22,
  Farsi = 23,
  Bulgarian = 24,
  Romanian = 25,
  Chinese = 26,
  Japanese = 27,
  Korean = 28,
  Taiwanese = 29,
  Thai = 30,
  Hebrew = 31,
  BrazilianPortuguese = 32,
  Indonesian = 33,
  Malaysian = 34,
  Vietnamese = 35,
  Burmese = 36,
  Mongolian = 37,
  Custom = 254,
}

impl From<LanguageField> for u8 {
  fn from(source: LanguageField) -> u8 {
    match source {
      LanguageField::English => 0,
      LanguageField::French => 1,
      LanguageField::Italian => 2,
      LanguageField::German => 3,
      LanguageField::Spanish => 4,
      LanguageField::Croatian => 5,
      LanguageField::Czech => 6,
      LanguageField::Danish => 7,
      LanguageField::Dutch => 8,
      LanguageField::Finnish => 9,
      LanguageField::Greek => 10,
      LanguageField::Hungarian => 11,
      LanguageField::Norwegian => 12,
      LanguageField::Polish => 13,
      LanguageField::Portuguese => 14,
      LanguageField::Slovakian => 15,
      LanguageField::Slovenian => 16,
      LanguageField::Swedish => 17,
      LanguageField::Russian => 18,
      LanguageField::Turkish => 19,
      LanguageField::Latvian => 20,
      LanguageField::Ukrainian => 21,
      LanguageField::Arabic => 22,
      LanguageField::Farsi => 23,
      LanguageField::Bulgarian => 24,
      LanguageField::Romanian => 25,
      LanguageField::Chinese => 26,
      LanguageField::Japanese => 27,
      LanguageField::Korean => 28,
      LanguageField::Taiwanese => 29,
      LanguageField::Thai => 30,
      LanguageField::Hebrew => 31,
      LanguageField::BrazilianPortuguese => 32,
      LanguageField::Indonesian => 33,
      LanguageField::Malaysian => 34,
      LanguageField::Vietnamese => 35,
      LanguageField::Burmese => 36,
      LanguageField::Mongolian => 37,
      LanguageField::Custom => 254,
    }
  }
}

impl IntoField<u8> for LanguageField {
  fn into_field(value: u8) -> Option<LanguageField> {
    match value {
      0 => Some(LanguageField::English),
      1 => Some(LanguageField::French),
      2 => Some(LanguageField::Italian),
      3 => Some(LanguageField::German),
      4 => Some(LanguageField::Spanish),
      5 => Some(LanguageField::Croatian),
      6 => Some(LanguageField::Czech),
      7 => Some(LanguageField::Danish),
      8 => Some(LanguageField::Dutch),
      9 => Some(LanguageField::Finnish),
      10 => Some(LanguageField::Greek),
      11 => Some(LanguageField::Hungarian),
      12 => Some(LanguageField::Norwegian),
      13 => Some(LanguageField::Polish),
      14 => Some(LanguageField::Portuguese),
      15 => Some(LanguageField::Slovakian),
      16 => Some(LanguageField::Slovenian),
      17 => Some(LanguageField::Swedish),
      18 => Some(LanguageField::Russian),
      19 => Some(LanguageField::Turkish),
      20 => Some(LanguageField::Latvian),
      21 => Some(LanguageField::Ukrainian),
      22 => Some(LanguageField::Arabic),
      23 => Some(LanguageField::Farsi),
      24 => Some(LanguageField::Bulgarian),
      25 => Some(LanguageField::Romanian),
      26 => Some(LanguageField::Chinese),
      27 => Some(LanguageField::Japanese),
      28 => Some(LanguageField::Korean),
      29 => Some(LanguageField::Taiwanese),
      30 => Some(LanguageField::Thai),
      31 => Some(LanguageField::Hebrew),
      32 => Some(LanguageField::BrazilianPortuguese),
      33 => Some(LanguageField::Indonesian),
      34 => Some(LanguageField::Malaysian),
      35 => Some(LanguageField::Vietnamese),
      36 => Some(LanguageField::Burmese),
      37 => Some(LanguageField::Mongolian),
      254 => Some(LanguageField::Custom),
      _ => None,
    }
  }
}

/* name: sub_sport type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SubSportField {
  Generic = 0,
  Treadmill = 1,
  Street = 2,
  Trail = 3,
  Track = 4,
  Spin = 5,
  IndoorCycling = 6,
  Road = 7,
  Mountain = 8,
  Downhill = 9,
  Recumbent = 10,
  Cyclocross = 11,
  HandCycling = 12,
  TrackCycling = 13,
  IndoorRowing = 14,
  Elliptical = 15,
  StairClimbing = 16,
  LapSwimming = 17,
  OpenWater = 18,
  FlexibilityTraining = 19,
  StrengthTraining = 20,
  WarmUp = 21,
  Match = 22,
  Exercise = 23,
  Challenge = 24,
  IndoorSkiing = 25,
  CardioTraining = 26,
  IndoorWalking = 27,
  EBikeFitness = 28,
  Bmx = 29,
  CasualWalking = 30,
  SpeedWalking = 31,
  BikeToRunTransition = 32,
  RunToBikeTransition = 33,
  SwimToBikeTransition = 34,
  Atv = 35,
  Motocross = 36,
  Backcountry = 37,
  Resort = 38,
  RcDrone = 39,
  Wingsuit = 40,
  Whitewater = 41,
  SkateSkiing = 42,
  Yoga = 43,
  Pilates = 44,
  IndoorRunning = 45,
  GravelCycling = 46,
  EBikeMountain = 47,
  Commuting = 48,
  MixedSurface = 49,
  Navigate = 50,
  TrackMe = 51,
  Map = 52,
  VirtualActivity = 58,
  Obstacle = 59,
  All = 254,
}

impl From<SubSportField> for u8 {
  fn from(source: SubSportField) -> u8 {
    match source {
      SubSportField::Generic => 0,
      SubSportField::Treadmill => 1,
      SubSportField::Street => 2,
      SubSportField::Trail => 3,
      SubSportField::Track => 4,
      SubSportField::Spin => 5,
      SubSportField::IndoorCycling => 6,
      SubSportField::Road => 7,
      SubSportField::Mountain => 8,
      SubSportField::Downhill => 9,
      SubSportField::Recumbent => 10,
      SubSportField::Cyclocross => 11,
      SubSportField::HandCycling => 12,
      SubSportField::TrackCycling => 13,
      SubSportField::IndoorRowing => 14,
      SubSportField::Elliptical => 15,
      SubSportField::StairClimbing => 16,
      SubSportField::LapSwimming => 17,
      SubSportField::OpenWater => 18,
      SubSportField::FlexibilityTraining => 19,
      SubSportField::StrengthTraining => 20,
      SubSportField::WarmUp => 21,
      SubSportField::Match => 22,
      SubSportField::Exercise => 23,
      SubSportField::Challenge => 24,
      SubSportField::IndoorSkiing => 25,
      SubSportField::CardioTraining => 26,
      SubSportField::IndoorWalking => 27,
      SubSportField::EBikeFitness => 28,
      SubSportField::Bmx => 29,
      SubSportField::CasualWalking => 30,
      SubSportField::SpeedWalking => 31,
      SubSportField::BikeToRunTransition => 32,
      SubSportField::RunToBikeTransition => 33,
      SubSportField::SwimToBikeTransition => 34,
      SubSportField::Atv => 35,
      SubSportField::Motocross => 36,
      SubSportField::Backcountry => 37,
      SubSportField::Resort => 38,
      SubSportField::RcDrone => 39,
      SubSportField::Wingsuit => 40,
      SubSportField::Whitewater => 41,
      SubSportField::SkateSkiing => 42,
      SubSportField::Yoga => 43,
      SubSportField::Pilates => 44,
      SubSportField::IndoorRunning => 45,
      SubSportField::GravelCycling => 46,
      SubSportField::EBikeMountain => 47,
      SubSportField::Commuting => 48,
      SubSportField::MixedSurface => 49,
      SubSportField::Navigate => 50,
      SubSportField::TrackMe => 51,
      SubSportField::Map => 52,
      SubSportField::VirtualActivity => 58,
      SubSportField::Obstacle => 59,
      SubSportField::All => 254,
    }
  }
}

impl IntoField<u8> for SubSportField {
  fn into_field(value: u8) -> Option<SubSportField> {
    match value {
      0 => Some(SubSportField::Generic),
      1 => Some(SubSportField::Treadmill),
      2 => Some(SubSportField::Street),
      3 => Some(SubSportField::Trail),
      4 => Some(SubSportField::Track),
      5 => Some(SubSportField::Spin),
      6 => Some(SubSportField::IndoorCycling),
      7 => Some(SubSportField::Road),
      8 => Some(SubSportField::Mountain),
      9 => Some(SubSportField::Downhill),
      10 => Some(SubSportField::Recumbent),
      11 => Some(SubSportField::Cyclocross),
      12 => Some(SubSportField::HandCycling),
      13 => Some(SubSportField::TrackCycling),
      14 => Some(SubSportField::IndoorRowing),
      15 => Some(SubSportField::Elliptical),
      16 => Some(SubSportField::StairClimbing),
      17 => Some(SubSportField::LapSwimming),
      18 => Some(SubSportField::OpenWater),
      19 => Some(SubSportField::FlexibilityTraining),
      20 => Some(SubSportField::StrengthTraining),
      21 => Some(SubSportField::WarmUp),
      22 => Some(SubSportField::Match),
      23 => Some(SubSportField::Exercise),
      24 => Some(SubSportField::Challenge),
      25 => Some(SubSportField::IndoorSkiing),
      26 => Some(SubSportField::CardioTraining),
      27 => Some(SubSportField::IndoorWalking),
      28 => Some(SubSportField::EBikeFitness),
      29 => Some(SubSportField::Bmx),
      30 => Some(SubSportField::CasualWalking),
      31 => Some(SubSportField::SpeedWalking),
      32 => Some(SubSportField::BikeToRunTransition),
      33 => Some(SubSportField::RunToBikeTransition),
      34 => Some(SubSportField::SwimToBikeTransition),
      35 => Some(SubSportField::Atv),
      36 => Some(SubSportField::Motocross),
      37 => Some(SubSportField::Backcountry),
      38 => Some(SubSportField::Resort),
      39 => Some(SubSportField::RcDrone),
      40 => Some(SubSportField::Wingsuit),
      41 => Some(SubSportField::Whitewater),
      42 => Some(SubSportField::SkateSkiing),
      43 => Some(SubSportField::Yoga),
      44 => Some(SubSportField::Pilates),
      45 => Some(SubSportField::IndoorRunning),
      46 => Some(SubSportField::GravelCycling),
      47 => Some(SubSportField::EBikeMountain),
      48 => Some(SubSportField::Commuting),
      49 => Some(SubSportField::MixedSurface),
      50 => Some(SubSportField::Navigate),
      51 => Some(SubSportField::TrackMe),
      52 => Some(SubSportField::Map),
      58 => Some(SubSportField::VirtualActivity),
      59 => Some(SubSportField::Obstacle),
      254 => Some(SubSportField::All),
      _ => None,
    }
  }
}

/* name: segment_lap_status type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SegmentLapStatusField {
  End = 0,
  Fail = 1,
}

impl From<SegmentLapStatusField> for u8 {
  fn from(source: SegmentLapStatusField) -> u8 {
    match source {
      SegmentLapStatusField::End => 0,
      SegmentLapStatusField::Fail => 1,
    }
  }
}

impl IntoField<u8> for SegmentLapStatusField {
  fn into_field(value: u8) -> Option<SegmentLapStatusField> {
    match value {
      0 => Some(SegmentLapStatusField::End),
      1 => Some(SegmentLapStatusField::Fail),
      _ => None,
    }
  }
}

/* name: manufacturer type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ManufacturerField {
  Garmin = 1,
  GarminFr405Antfs = 2,
  Zephyr = 3,
  Dayton = 4,
  Idt = 5,
  Srm = 6,
  Quarq = 7,
  Ibike = 8,
  Saris = 9,
  SparkHk = 10,
  Tanita = 11,
  Echowell = 12,
  DynastreamOem = 13,
  Nautilus = 14,
  Dynastream = 15,
  Timex = 16,
  Metrigear = 17,
  Xelic = 18,
  Beurer = 19,
  Cardiosport = 20,
  AAndD = 21,
  Hmm = 22,
  Suunto = 23,
  ThitaElektronik = 24,
  Gpulse = 25,
  CleanMobile = 26,
  PedalBrain = 27,
  Peaksware = 28,
  Saxonar = 29,
  LemondFitness = 30,
  Dexcom = 31,
  WahooFitness = 32,
  OctaneFitness = 33,
  Archinoetics = 34,
  TheHurtBox = 35,
  CitizenSystems = 36,
  Magellan = 37,
  Osynce = 38,
  Holux = 39,
  Concept2 = 40,
  OneGiantLeap = 42,
  AceSensor = 43,
  BrimBrothers = 44,
  Xplova = 45,
  PerceptionDigital = 46,
  Bf1systems = 47,
  Pioneer = 48,
  Spantec = 49,
  Metalogics = 50,
  _4iiiis = 51,
  SeikoEpson = 52,
  SeikoEpsonOem = 53,
  IforPowell = 54,
  MaxwellGuider = 55,
  StarTrac = 56,
  Breakaway = 57,
  AlatechTechnologyLtd = 58,
  MioTechnologyEurope = 59,
  Rotor = 60,
  Geonaute = 61,
  IdBike = 62,
  Specialized = 63,
  Wtek = 64,
  PhysicalEnterprises = 65,
  NorthPoleEngineering = 66,
  Bkool = 67,
  Cateye = 68,
  StagesCycling = 69,
  Sigmasport = 70,
  Tomtom = 71,
  Peripedal = 72,
  Wattbike = 73,
  Moxy = 76,
  Ciclosport = 77,
  Powerbahn = 78,
  AcornProjectsAps = 79,
  Lifebeam = 80,
  Bontrager = 81,
  Wellgo = 82,
  Scosche = 83,
  Magura = 84,
  Woodway = 85,
  Elite = 86,
  NielsenKellerman = 87,
  DkCity = 88,
  Tacx = 89,
  DirectionTechnology = 90,
  Magtonic = 91,
  _1partcarbon = 92,
  InsideRideTechnologies = 93,
  SoundOfMotion = 94,
  Stryd = 95,
  Icg = 96,
  Mipulse = 97,
  BsxAthletics = 98,
  Look = 99,
  CampagnoloSrl = 100,
  BodyBikeSmart = 101,
  Praxisworks = 102,
  LimitsTechnology = 103,
  TopactionTechnology = 104,
  Cosinuss = 105,
  Fitcare = 106,
  Magene = 107,
  GiantManufacturingCo = 108,
  Tigrasport = 109,
  Salutron = 110,
  Technogym = 111,
  BrytonSensors = 112,
  LatitudeLimited = 113,
  SoaringTechnology = 114,
  Igpsport = 115,
  Thinkrider = 116,
  Development = 255,
  Healthandlife = 257,
  Lezyne = 258,
  ScribeLabs = 259,
  Zwift = 260,
  Watteam = 261,
  Recon = 262,
  FaveroElectronics = 263,
  Dynovelo = 264,
  Strava = 265,
  Precor = 266,
  Bryton = 267,
  Sram = 268,
  Navman = 269,
  Cobi = 270,
  Spivi = 271,
  MioMagellan = 272,
  Evesports = 273,
  SensitivusGauge = 274,
  Podoon = 275,
  LifeTimeFitness = 276,
  FalcoEMotors = 277,
  Minoura = 278,
  Cycliq = 279,
  Luxottica = 280,
  TrainerRoad = 281,
  TheSufferfest = 282,
  Fullspeedahead = 283,
  Virtualtraining = 284,
  Actigraphcorp = 5759,
}

impl From<ManufacturerField> for u16 {
  fn from(source: ManufacturerField) -> u16 {
    match source {
      ManufacturerField::Garmin => 1,
      ManufacturerField::GarminFr405Antfs => 2,
      ManufacturerField::Zephyr => 3,
      ManufacturerField::Dayton => 4,
      ManufacturerField::Idt => 5,
      ManufacturerField::Srm => 6,
      ManufacturerField::Quarq => 7,
      ManufacturerField::Ibike => 8,
      ManufacturerField::Saris => 9,
      ManufacturerField::SparkHk => 10,
      ManufacturerField::Tanita => 11,
      ManufacturerField::Echowell => 12,
      ManufacturerField::DynastreamOem => 13,
      ManufacturerField::Nautilus => 14,
      ManufacturerField::Dynastream => 15,
      ManufacturerField::Timex => 16,
      ManufacturerField::Metrigear => 17,
      ManufacturerField::Xelic => 18,
      ManufacturerField::Beurer => 19,
      ManufacturerField::Cardiosport => 20,
      ManufacturerField::AAndD => 21,
      ManufacturerField::Hmm => 22,
      ManufacturerField::Suunto => 23,
      ManufacturerField::ThitaElektronik => 24,
      ManufacturerField::Gpulse => 25,
      ManufacturerField::CleanMobile => 26,
      ManufacturerField::PedalBrain => 27,
      ManufacturerField::Peaksware => 28,
      ManufacturerField::Saxonar => 29,
      ManufacturerField::LemondFitness => 30,
      ManufacturerField::Dexcom => 31,
      ManufacturerField::WahooFitness => 32,
      ManufacturerField::OctaneFitness => 33,
      ManufacturerField::Archinoetics => 34,
      ManufacturerField::TheHurtBox => 35,
      ManufacturerField::CitizenSystems => 36,
      ManufacturerField::Magellan => 37,
      ManufacturerField::Osynce => 38,
      ManufacturerField::Holux => 39,
      ManufacturerField::Concept2 => 40,
      ManufacturerField::OneGiantLeap => 42,
      ManufacturerField::AceSensor => 43,
      ManufacturerField::BrimBrothers => 44,
      ManufacturerField::Xplova => 45,
      ManufacturerField::PerceptionDigital => 46,
      ManufacturerField::Bf1systems => 47,
      ManufacturerField::Pioneer => 48,
      ManufacturerField::Spantec => 49,
      ManufacturerField::Metalogics => 50,
      ManufacturerField::_4iiiis => 51,
      ManufacturerField::SeikoEpson => 52,
      ManufacturerField::SeikoEpsonOem => 53,
      ManufacturerField::IforPowell => 54,
      ManufacturerField::MaxwellGuider => 55,
      ManufacturerField::StarTrac => 56,
      ManufacturerField::Breakaway => 57,
      ManufacturerField::AlatechTechnologyLtd => 58,
      ManufacturerField::MioTechnologyEurope => 59,
      ManufacturerField::Rotor => 60,
      ManufacturerField::Geonaute => 61,
      ManufacturerField::IdBike => 62,
      ManufacturerField::Specialized => 63,
      ManufacturerField::Wtek => 64,
      ManufacturerField::PhysicalEnterprises => 65,
      ManufacturerField::NorthPoleEngineering => 66,
      ManufacturerField::Bkool => 67,
      ManufacturerField::Cateye => 68,
      ManufacturerField::StagesCycling => 69,
      ManufacturerField::Sigmasport => 70,
      ManufacturerField::Tomtom => 71,
      ManufacturerField::Peripedal => 72,
      ManufacturerField::Wattbike => 73,
      ManufacturerField::Moxy => 76,
      ManufacturerField::Ciclosport => 77,
      ManufacturerField::Powerbahn => 78,
      ManufacturerField::AcornProjectsAps => 79,
      ManufacturerField::Lifebeam => 80,
      ManufacturerField::Bontrager => 81,
      ManufacturerField::Wellgo => 82,
      ManufacturerField::Scosche => 83,
      ManufacturerField::Magura => 84,
      ManufacturerField::Woodway => 85,
      ManufacturerField::Elite => 86,
      ManufacturerField::NielsenKellerman => 87,
      ManufacturerField::DkCity => 88,
      ManufacturerField::Tacx => 89,
      ManufacturerField::DirectionTechnology => 90,
      ManufacturerField::Magtonic => 91,
      ManufacturerField::_1partcarbon => 92,
      ManufacturerField::InsideRideTechnologies => 93,
      ManufacturerField::SoundOfMotion => 94,
      ManufacturerField::Stryd => 95,
      ManufacturerField::Icg => 96,
      ManufacturerField::Mipulse => 97,
      ManufacturerField::BsxAthletics => 98,
      ManufacturerField::Look => 99,
      ManufacturerField::CampagnoloSrl => 100,
      ManufacturerField::BodyBikeSmart => 101,
      ManufacturerField::Praxisworks => 102,
      ManufacturerField::LimitsTechnology => 103,
      ManufacturerField::TopactionTechnology => 104,
      ManufacturerField::Cosinuss => 105,
      ManufacturerField::Fitcare => 106,
      ManufacturerField::Magene => 107,
      ManufacturerField::GiantManufacturingCo => 108,
      ManufacturerField::Tigrasport => 109,
      ManufacturerField::Salutron => 110,
      ManufacturerField::Technogym => 111,
      ManufacturerField::BrytonSensors => 112,
      ManufacturerField::LatitudeLimited => 113,
      ManufacturerField::SoaringTechnology => 114,
      ManufacturerField::Igpsport => 115,
      ManufacturerField::Thinkrider => 116,
      ManufacturerField::Development => 255,
      ManufacturerField::Healthandlife => 257,
      ManufacturerField::Lezyne => 258,
      ManufacturerField::ScribeLabs => 259,
      ManufacturerField::Zwift => 260,
      ManufacturerField::Watteam => 261,
      ManufacturerField::Recon => 262,
      ManufacturerField::FaveroElectronics => 263,
      ManufacturerField::Dynovelo => 264,
      ManufacturerField::Strava => 265,
      ManufacturerField::Precor => 266,
      ManufacturerField::Bryton => 267,
      ManufacturerField::Sram => 268,
      ManufacturerField::Navman => 269,
      ManufacturerField::Cobi => 270,
      ManufacturerField::Spivi => 271,
      ManufacturerField::MioMagellan => 272,
      ManufacturerField::Evesports => 273,
      ManufacturerField::SensitivusGauge => 274,
      ManufacturerField::Podoon => 275,
      ManufacturerField::LifeTimeFitness => 276,
      ManufacturerField::FalcoEMotors => 277,
      ManufacturerField::Minoura => 278,
      ManufacturerField::Cycliq => 279,
      ManufacturerField::Luxottica => 280,
      ManufacturerField::TrainerRoad => 281,
      ManufacturerField::TheSufferfest => 282,
      ManufacturerField::Fullspeedahead => 283,
      ManufacturerField::Virtualtraining => 284,
      ManufacturerField::Actigraphcorp => 5759,
    }
  }
}

impl IntoField<u16> for ManufacturerField {
  fn into_field(value: u16) -> Option<ManufacturerField> {
    match value {
      1 => Some(ManufacturerField::Garmin),
      2 => Some(ManufacturerField::GarminFr405Antfs),
      3 => Some(ManufacturerField::Zephyr),
      4 => Some(ManufacturerField::Dayton),
      5 => Some(ManufacturerField::Idt),
      6 => Some(ManufacturerField::Srm),
      7 => Some(ManufacturerField::Quarq),
      8 => Some(ManufacturerField::Ibike),
      9 => Some(ManufacturerField::Saris),
      10 => Some(ManufacturerField::SparkHk),
      11 => Some(ManufacturerField::Tanita),
      12 => Some(ManufacturerField::Echowell),
      13 => Some(ManufacturerField::DynastreamOem),
      14 => Some(ManufacturerField::Nautilus),
      15 => Some(ManufacturerField::Dynastream),
      16 => Some(ManufacturerField::Timex),
      17 => Some(ManufacturerField::Metrigear),
      18 => Some(ManufacturerField::Xelic),
      19 => Some(ManufacturerField::Beurer),
      20 => Some(ManufacturerField::Cardiosport),
      21 => Some(ManufacturerField::AAndD),
      22 => Some(ManufacturerField::Hmm),
      23 => Some(ManufacturerField::Suunto),
      24 => Some(ManufacturerField::ThitaElektronik),
      25 => Some(ManufacturerField::Gpulse),
      26 => Some(ManufacturerField::CleanMobile),
      27 => Some(ManufacturerField::PedalBrain),
      28 => Some(ManufacturerField::Peaksware),
      29 => Some(ManufacturerField::Saxonar),
      30 => Some(ManufacturerField::LemondFitness),
      31 => Some(ManufacturerField::Dexcom),
      32 => Some(ManufacturerField::WahooFitness),
      33 => Some(ManufacturerField::OctaneFitness),
      34 => Some(ManufacturerField::Archinoetics),
      35 => Some(ManufacturerField::TheHurtBox),
      36 => Some(ManufacturerField::CitizenSystems),
      37 => Some(ManufacturerField::Magellan),
      38 => Some(ManufacturerField::Osynce),
      39 => Some(ManufacturerField::Holux),
      40 => Some(ManufacturerField::Concept2),
      42 => Some(ManufacturerField::OneGiantLeap),
      43 => Some(ManufacturerField::AceSensor),
      44 => Some(ManufacturerField::BrimBrothers),
      45 => Some(ManufacturerField::Xplova),
      46 => Some(ManufacturerField::PerceptionDigital),
      47 => Some(ManufacturerField::Bf1systems),
      48 => Some(ManufacturerField::Pioneer),
      49 => Some(ManufacturerField::Spantec),
      50 => Some(ManufacturerField::Metalogics),
      51 => Some(ManufacturerField::_4iiiis),
      52 => Some(ManufacturerField::SeikoEpson),
      53 => Some(ManufacturerField::SeikoEpsonOem),
      54 => Some(ManufacturerField::IforPowell),
      55 => Some(ManufacturerField::MaxwellGuider),
      56 => Some(ManufacturerField::StarTrac),
      57 => Some(ManufacturerField::Breakaway),
      58 => Some(ManufacturerField::AlatechTechnologyLtd),
      59 => Some(ManufacturerField::MioTechnologyEurope),
      60 => Some(ManufacturerField::Rotor),
      61 => Some(ManufacturerField::Geonaute),
      62 => Some(ManufacturerField::IdBike),
      63 => Some(ManufacturerField::Specialized),
      64 => Some(ManufacturerField::Wtek),
      65 => Some(ManufacturerField::PhysicalEnterprises),
      66 => Some(ManufacturerField::NorthPoleEngineering),
      67 => Some(ManufacturerField::Bkool),
      68 => Some(ManufacturerField::Cateye),
      69 => Some(ManufacturerField::StagesCycling),
      70 => Some(ManufacturerField::Sigmasport),
      71 => Some(ManufacturerField::Tomtom),
      72 => Some(ManufacturerField::Peripedal),
      73 => Some(ManufacturerField::Wattbike),
      76 => Some(ManufacturerField::Moxy),
      77 => Some(ManufacturerField::Ciclosport),
      78 => Some(ManufacturerField::Powerbahn),
      79 => Some(ManufacturerField::AcornProjectsAps),
      80 => Some(ManufacturerField::Lifebeam),
      81 => Some(ManufacturerField::Bontrager),
      82 => Some(ManufacturerField::Wellgo),
      83 => Some(ManufacturerField::Scosche),
      84 => Some(ManufacturerField::Magura),
      85 => Some(ManufacturerField::Woodway),
      86 => Some(ManufacturerField::Elite),
      87 => Some(ManufacturerField::NielsenKellerman),
      88 => Some(ManufacturerField::DkCity),
      89 => Some(ManufacturerField::Tacx),
      90 => Some(ManufacturerField::DirectionTechnology),
      91 => Some(ManufacturerField::Magtonic),
      92 => Some(ManufacturerField::_1partcarbon),
      93 => Some(ManufacturerField::InsideRideTechnologies),
      94 => Some(ManufacturerField::SoundOfMotion),
      95 => Some(ManufacturerField::Stryd),
      96 => Some(ManufacturerField::Icg),
      97 => Some(ManufacturerField::Mipulse),
      98 => Some(ManufacturerField::BsxAthletics),
      99 => Some(ManufacturerField::Look),
      100 => Some(ManufacturerField::CampagnoloSrl),
      101 => Some(ManufacturerField::BodyBikeSmart),
      102 => Some(ManufacturerField::Praxisworks),
      103 => Some(ManufacturerField::LimitsTechnology),
      104 => Some(ManufacturerField::TopactionTechnology),
      105 => Some(ManufacturerField::Cosinuss),
      106 => Some(ManufacturerField::Fitcare),
      107 => Some(ManufacturerField::Magene),
      108 => Some(ManufacturerField::GiantManufacturingCo),
      109 => Some(ManufacturerField::Tigrasport),
      110 => Some(ManufacturerField::Salutron),
      111 => Some(ManufacturerField::Technogym),
      112 => Some(ManufacturerField::BrytonSensors),
      113 => Some(ManufacturerField::LatitudeLimited),
      114 => Some(ManufacturerField::SoaringTechnology),
      115 => Some(ManufacturerField::Igpsport),
      116 => Some(ManufacturerField::Thinkrider),
      255 => Some(ManufacturerField::Development),
      257 => Some(ManufacturerField::Healthandlife),
      258 => Some(ManufacturerField::Lezyne),
      259 => Some(ManufacturerField::ScribeLabs),
      260 => Some(ManufacturerField::Zwift),
      261 => Some(ManufacturerField::Watteam),
      262 => Some(ManufacturerField::Recon),
      263 => Some(ManufacturerField::FaveroElectronics),
      264 => Some(ManufacturerField::Dynovelo),
      265 => Some(ManufacturerField::Strava),
      266 => Some(ManufacturerField::Precor),
      267 => Some(ManufacturerField::Bryton),
      268 => Some(ManufacturerField::Sram),
      269 => Some(ManufacturerField::Navman),
      270 => Some(ManufacturerField::Cobi),
      271 => Some(ManufacturerField::Spivi),
      272 => Some(ManufacturerField::MioMagellan),
      273 => Some(ManufacturerField::Evesports),
      274 => Some(ManufacturerField::SensitivusGauge),
      275 => Some(ManufacturerField::Podoon),
      276 => Some(ManufacturerField::LifeTimeFitness),
      277 => Some(ManufacturerField::FalcoEMotors),
      278 => Some(ManufacturerField::Minoura),
      279 => Some(ManufacturerField::Cycliq),
      280 => Some(ManufacturerField::Luxottica),
      281 => Some(ManufacturerField::TrainerRoad),
      282 => Some(ManufacturerField::TheSufferfest),
      283 => Some(ManufacturerField::Fullspeedahead),
      284 => Some(ManufacturerField::Virtualtraining),
      5759 => Some(ManufacturerField::Actigraphcorp),
      _ => None,
    }
  }
}

/* name: power_phase_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum PowerPhaseTypeField {
  PowerPhaseStartAngle = 0,
  PowerPhaseEndAngle = 1,
  PowerPhaseArcLength = 2,
  PowerPhaseCenter = 3,
}

impl From<PowerPhaseTypeField> for u8 {
  fn from(source: PowerPhaseTypeField) -> u8 {
    match source {
      PowerPhaseTypeField::PowerPhaseStartAngle => 0,
      PowerPhaseTypeField::PowerPhaseEndAngle => 1,
      PowerPhaseTypeField::PowerPhaseArcLength => 2,
      PowerPhaseTypeField::PowerPhaseCenter => 3,
    }
  }
}

impl IntoField<u8> for PowerPhaseTypeField {
  fn into_field(value: u8) -> Option<PowerPhaseTypeField> {
    match value {
      0 => Some(PowerPhaseTypeField::PowerPhaseStartAngle),
      1 => Some(PowerPhaseTypeField::PowerPhaseEndAngle),
      2 => Some(PowerPhaseTypeField::PowerPhaseArcLength),
      3 => Some(PowerPhaseTypeField::PowerPhaseCenter),
      _ => None,
    }
  }
}

/* name: weather_severe_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WeatherSevereTypeField {
  Unspecified = 0,
  Tornado = 1,
  Tsunami = 2,
  Hurricane = 3,
  ExtremeWind = 4,
  Typhoon = 5,
  InlandHurricane = 6,
  HurricaneForceWind = 7,
  Waterspout = 8,
  SevereThunderstorm = 9,
  WreckhouseWinds = 10,
  LesSuetesWind = 11,
  Avalanche = 12,
  FlashFlood = 13,
  TropicalStorm = 14,
  InlandTropicalStorm = 15,
  Blizzard = 16,
  IceStorm = 17,
  FreezingRain = 18,
  DebrisFlow = 19,
  FlashFreeze = 20,
  DustStorm = 21,
  HighWind = 22,
  WinterStorm = 23,
  HeavyFreezingSpray = 24,
  ExtremeCold = 25,
  WindChill = 26,
  ColdWave = 27,
  HeavySnowAlert = 28,
  LakeEffectBlowingSnow = 29,
  SnowSquall = 30,
  LakeEffectSnow = 31,
  WinterWeather = 32,
  Sleet = 33,
  Snowfall = 34,
  SnowAndBlowingSnow = 35,
  BlowingSnow = 36,
  SnowAlert = 37,
  ArcticOutflow = 38,
  FreezingDrizzle = 39,
  Storm = 40,
  StormSurge = 41,
  Rainfall = 42,
  ArealFlood = 43,
  CoastalFlood = 44,
  LakeshoreFlood = 45,
  ExcessiveHeat = 46,
  Heat = 47,
  Weather = 48,
  HighHeatAndHumidity = 49,
  HumidexAndHealth = 50,
  Humidex = 51,
  Gale = 52,
  FreezingSpray = 53,
  SpecialMarine = 54,
  Squall = 55,
  StrongWind = 56,
  LakeWind = 57,
  MarineWeather = 58,
  Wind = 59,
  SmallCraftHazardousSeas = 60,
  HazardousSeas = 61,
  SmallCraft = 62,
  SmallCraftWinds = 63,
  SmallCraftRoughBar = 64,
  HighWaterLevel = 65,
  Ashfall = 66,
  FreezingFog = 67,
  DenseFog = 68,
  DenseSmoke = 69,
  BlowingDust = 70,
  HardFreeze = 71,
  Freeze = 72,
  Frost = 73,
  FireWeather = 74,
  Flood = 75,
  RipTide = 76,
  HighSurf = 77,
  Smog = 78,
  AirQuality = 79,
  BriskWind = 80,
  AirStagnation = 81,
  LowWater = 82,
  Hydrological = 83,
  SpecialWeather = 84,
}

impl From<WeatherSevereTypeField> for u8 {
  fn from(source: WeatherSevereTypeField) -> u8 {
    match source {
      WeatherSevereTypeField::Unspecified => 0,
      WeatherSevereTypeField::Tornado => 1,
      WeatherSevereTypeField::Tsunami => 2,
      WeatherSevereTypeField::Hurricane => 3,
      WeatherSevereTypeField::ExtremeWind => 4,
      WeatherSevereTypeField::Typhoon => 5,
      WeatherSevereTypeField::InlandHurricane => 6,
      WeatherSevereTypeField::HurricaneForceWind => 7,
      WeatherSevereTypeField::Waterspout => 8,
      WeatherSevereTypeField::SevereThunderstorm => 9,
      WeatherSevereTypeField::WreckhouseWinds => 10,
      WeatherSevereTypeField::LesSuetesWind => 11,
      WeatherSevereTypeField::Avalanche => 12,
      WeatherSevereTypeField::FlashFlood => 13,
      WeatherSevereTypeField::TropicalStorm => 14,
      WeatherSevereTypeField::InlandTropicalStorm => 15,
      WeatherSevereTypeField::Blizzard => 16,
      WeatherSevereTypeField::IceStorm => 17,
      WeatherSevereTypeField::FreezingRain => 18,
      WeatherSevereTypeField::DebrisFlow => 19,
      WeatherSevereTypeField::FlashFreeze => 20,
      WeatherSevereTypeField::DustStorm => 21,
      WeatherSevereTypeField::HighWind => 22,
      WeatherSevereTypeField::WinterStorm => 23,
      WeatherSevereTypeField::HeavyFreezingSpray => 24,
      WeatherSevereTypeField::ExtremeCold => 25,
      WeatherSevereTypeField::WindChill => 26,
      WeatherSevereTypeField::ColdWave => 27,
      WeatherSevereTypeField::HeavySnowAlert => 28,
      WeatherSevereTypeField::LakeEffectBlowingSnow => 29,
      WeatherSevereTypeField::SnowSquall => 30,
      WeatherSevereTypeField::LakeEffectSnow => 31,
      WeatherSevereTypeField::WinterWeather => 32,
      WeatherSevereTypeField::Sleet => 33,
      WeatherSevereTypeField::Snowfall => 34,
      WeatherSevereTypeField::SnowAndBlowingSnow => 35,
      WeatherSevereTypeField::BlowingSnow => 36,
      WeatherSevereTypeField::SnowAlert => 37,
      WeatherSevereTypeField::ArcticOutflow => 38,
      WeatherSevereTypeField::FreezingDrizzle => 39,
      WeatherSevereTypeField::Storm => 40,
      WeatherSevereTypeField::StormSurge => 41,
      WeatherSevereTypeField::Rainfall => 42,
      WeatherSevereTypeField::ArealFlood => 43,
      WeatherSevereTypeField::CoastalFlood => 44,
      WeatherSevereTypeField::LakeshoreFlood => 45,
      WeatherSevereTypeField::ExcessiveHeat => 46,
      WeatherSevereTypeField::Heat => 47,
      WeatherSevereTypeField::Weather => 48,
      WeatherSevereTypeField::HighHeatAndHumidity => 49,
      WeatherSevereTypeField::HumidexAndHealth => 50,
      WeatherSevereTypeField::Humidex => 51,
      WeatherSevereTypeField::Gale => 52,
      WeatherSevereTypeField::FreezingSpray => 53,
      WeatherSevereTypeField::SpecialMarine => 54,
      WeatherSevereTypeField::Squall => 55,
      WeatherSevereTypeField::StrongWind => 56,
      WeatherSevereTypeField::LakeWind => 57,
      WeatherSevereTypeField::MarineWeather => 58,
      WeatherSevereTypeField::Wind => 59,
      WeatherSevereTypeField::SmallCraftHazardousSeas => 60,
      WeatherSevereTypeField::HazardousSeas => 61,
      WeatherSevereTypeField::SmallCraft => 62,
      WeatherSevereTypeField::SmallCraftWinds => 63,
      WeatherSevereTypeField::SmallCraftRoughBar => 64,
      WeatherSevereTypeField::HighWaterLevel => 65,
      WeatherSevereTypeField::Ashfall => 66,
      WeatherSevereTypeField::FreezingFog => 67,
      WeatherSevereTypeField::DenseFog => 68,
      WeatherSevereTypeField::DenseSmoke => 69,
      WeatherSevereTypeField::BlowingDust => 70,
      WeatherSevereTypeField::HardFreeze => 71,
      WeatherSevereTypeField::Freeze => 72,
      WeatherSevereTypeField::Frost => 73,
      WeatherSevereTypeField::FireWeather => 74,
      WeatherSevereTypeField::Flood => 75,
      WeatherSevereTypeField::RipTide => 76,
      WeatherSevereTypeField::HighSurf => 77,
      WeatherSevereTypeField::Smog => 78,
      WeatherSevereTypeField::AirQuality => 79,
      WeatherSevereTypeField::BriskWind => 80,
      WeatherSevereTypeField::AirStagnation => 81,
      WeatherSevereTypeField::LowWater => 82,
      WeatherSevereTypeField::Hydrological => 83,
      WeatherSevereTypeField::SpecialWeather => 84,
    }
  }
}

impl IntoField<u8> for WeatherSevereTypeField {
  fn into_field(value: u8) -> Option<WeatherSevereTypeField> {
    match value {
      0 => Some(WeatherSevereTypeField::Unspecified),
      1 => Some(WeatherSevereTypeField::Tornado),
      2 => Some(WeatherSevereTypeField::Tsunami),
      3 => Some(WeatherSevereTypeField::Hurricane),
      4 => Some(WeatherSevereTypeField::ExtremeWind),
      5 => Some(WeatherSevereTypeField::Typhoon),
      6 => Some(WeatherSevereTypeField::InlandHurricane),
      7 => Some(WeatherSevereTypeField::HurricaneForceWind),
      8 => Some(WeatherSevereTypeField::Waterspout),
      9 => Some(WeatherSevereTypeField::SevereThunderstorm),
      10 => Some(WeatherSevereTypeField::WreckhouseWinds),
      11 => Some(WeatherSevereTypeField::LesSuetesWind),
      12 => Some(WeatherSevereTypeField::Avalanche),
      13 => Some(WeatherSevereTypeField::FlashFlood),
      14 => Some(WeatherSevereTypeField::TropicalStorm),
      15 => Some(WeatherSevereTypeField::InlandTropicalStorm),
      16 => Some(WeatherSevereTypeField::Blizzard),
      17 => Some(WeatherSevereTypeField::IceStorm),
      18 => Some(WeatherSevereTypeField::FreezingRain),
      19 => Some(WeatherSevereTypeField::DebrisFlow),
      20 => Some(WeatherSevereTypeField::FlashFreeze),
      21 => Some(WeatherSevereTypeField::DustStorm),
      22 => Some(WeatherSevereTypeField::HighWind),
      23 => Some(WeatherSevereTypeField::WinterStorm),
      24 => Some(WeatherSevereTypeField::HeavyFreezingSpray),
      25 => Some(WeatherSevereTypeField::ExtremeCold),
      26 => Some(WeatherSevereTypeField::WindChill),
      27 => Some(WeatherSevereTypeField::ColdWave),
      28 => Some(WeatherSevereTypeField::HeavySnowAlert),
      29 => Some(WeatherSevereTypeField::LakeEffectBlowingSnow),
      30 => Some(WeatherSevereTypeField::SnowSquall),
      31 => Some(WeatherSevereTypeField::LakeEffectSnow),
      32 => Some(WeatherSevereTypeField::WinterWeather),
      33 => Some(WeatherSevereTypeField::Sleet),
      34 => Some(WeatherSevereTypeField::Snowfall),
      35 => Some(WeatherSevereTypeField::SnowAndBlowingSnow),
      36 => Some(WeatherSevereTypeField::BlowingSnow),
      37 => Some(WeatherSevereTypeField::SnowAlert),
      38 => Some(WeatherSevereTypeField::ArcticOutflow),
      39 => Some(WeatherSevereTypeField::FreezingDrizzle),
      40 => Some(WeatherSevereTypeField::Storm),
      41 => Some(WeatherSevereTypeField::StormSurge),
      42 => Some(WeatherSevereTypeField::Rainfall),
      43 => Some(WeatherSevereTypeField::ArealFlood),
      44 => Some(WeatherSevereTypeField::CoastalFlood),
      45 => Some(WeatherSevereTypeField::LakeshoreFlood),
      46 => Some(WeatherSevereTypeField::ExcessiveHeat),
      47 => Some(WeatherSevereTypeField::Heat),
      48 => Some(WeatherSevereTypeField::Weather),
      49 => Some(WeatherSevereTypeField::HighHeatAndHumidity),
      50 => Some(WeatherSevereTypeField::HumidexAndHealth),
      51 => Some(WeatherSevereTypeField::Humidex),
      52 => Some(WeatherSevereTypeField::Gale),
      53 => Some(WeatherSevereTypeField::FreezingSpray),
      54 => Some(WeatherSevereTypeField::SpecialMarine),
      55 => Some(WeatherSevereTypeField::Squall),
      56 => Some(WeatherSevereTypeField::StrongWind),
      57 => Some(WeatherSevereTypeField::LakeWind),
      58 => Some(WeatherSevereTypeField::MarineWeather),
      59 => Some(WeatherSevereTypeField::Wind),
      60 => Some(WeatherSevereTypeField::SmallCraftHazardousSeas),
      61 => Some(WeatherSevereTypeField::HazardousSeas),
      62 => Some(WeatherSevereTypeField::SmallCraft),
      63 => Some(WeatherSevereTypeField::SmallCraftWinds),
      64 => Some(WeatherSevereTypeField::SmallCraftRoughBar),
      65 => Some(WeatherSevereTypeField::HighWaterLevel),
      66 => Some(WeatherSevereTypeField::Ashfall),
      67 => Some(WeatherSevereTypeField::FreezingFog),
      68 => Some(WeatherSevereTypeField::DenseFog),
      69 => Some(WeatherSevereTypeField::DenseSmoke),
      70 => Some(WeatherSevereTypeField::BlowingDust),
      71 => Some(WeatherSevereTypeField::HardFreeze),
      72 => Some(WeatherSevereTypeField::Freeze),
      73 => Some(WeatherSevereTypeField::Frost),
      74 => Some(WeatherSevereTypeField::FireWeather),
      75 => Some(WeatherSevereTypeField::Flood),
      76 => Some(WeatherSevereTypeField::RipTide),
      77 => Some(WeatherSevereTypeField::HighSurf),
      78 => Some(WeatherSevereTypeField::Smog),
      79 => Some(WeatherSevereTypeField::AirQuality),
      80 => Some(WeatherSevereTypeField::BriskWind),
      81 => Some(WeatherSevereTypeField::AirStagnation),
      82 => Some(WeatherSevereTypeField::LowWater),
      83 => Some(WeatherSevereTypeField::Hydrological),
      84 => Some(WeatherSevereTypeField::SpecialWeather),
      _ => None,
    }
  }
}

/* name: attitude_stage type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AttitudeStageField {
  Failed = 0,
  Aligning = 1,
  Degraded = 2,
  Valid = 3,
}

impl From<AttitudeStageField> for u8 {
  fn from(source: AttitudeStageField) -> u8 {
    match source {
      AttitudeStageField::Failed => 0,
      AttitudeStageField::Aligning => 1,
      AttitudeStageField::Degraded => 2,
      AttitudeStageField::Valid => 3,
    }
  }
}

impl IntoField<u8> for AttitudeStageField {
  fn into_field(value: u8) -> Option<AttitudeStageField> {
    match value {
      0 => Some(AttitudeStageField::Failed),
      1 => Some(AttitudeStageField::Aligning),
      2 => Some(AttitudeStageField::Degraded),
      3 => Some(AttitudeStageField::Valid),
      _ => None,
    }
  }
}

/* name: bike_light_beam_angle_mode type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BikeLightBeamAngleModeField {
  Manual = 0,
  Auto = 1,
}

impl From<BikeLightBeamAngleModeField> for u8 {
  fn from(source: BikeLightBeamAngleModeField) -> u8 {
    match source {
      BikeLightBeamAngleModeField::Manual => 0,
      BikeLightBeamAngleModeField::Auto => 1,
    }
  }
}

impl IntoField<u8> for BikeLightBeamAngleModeField {
  fn into_field(value: u8) -> Option<BikeLightBeamAngleModeField> {
    match value {
      0 => Some(BikeLightBeamAngleModeField::Manual),
      1 => Some(BikeLightBeamAngleModeField::Auto),
      _ => None,
    }
  }
}

/* name: device_index type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DeviceIndexField {
  Creator = 0,
}

impl From<DeviceIndexField> for u8 {
  fn from(source: DeviceIndexField) -> u8 {
    match source {
      DeviceIndexField::Creator => 0,
    }
  }
}

impl IntoField<u8> for DeviceIndexField {
  fn into_field(value: u8) -> Option<DeviceIndexField> {
    match value {
      0 => Some(DeviceIndexField::Creator),
      _ => None,
    }
  }
}

/* name: date_time type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DateTimeField {
  Min = 268435456,
}

impl From<DateTimeField> for u32 {
  fn from(source: DateTimeField) -> u32 {
    match source {
      DateTimeField::Min => 268435456,
    }
  }
}

impl IntoField<u32> for DateTimeField {
  fn into_field(value: u32) -> Option<DateTimeField> {
    match value {
      268435456 => Some(DateTimeField::Min),
      _ => None,
    }
  }
}

/* name: fitness_equipment_state type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum FitnessEquipmentStateField {
  Ready = 0,
  InUse = 1,
  Paused = 2,
  Unknown = 3,
}

impl From<FitnessEquipmentStateField> for u8 {
  fn from(source: FitnessEquipmentStateField) -> u8 {
    match source {
      FitnessEquipmentStateField::Ready => 0,
      FitnessEquipmentStateField::InUse => 1,
      FitnessEquipmentStateField::Paused => 2,
      FitnessEquipmentStateField::Unknown => 3,
    }
  }
}

impl IntoField<u8> for FitnessEquipmentStateField {
  fn into_field(value: u8) -> Option<FitnessEquipmentStateField> {
    match value {
      0 => Some(FitnessEquipmentStateField::Ready),
      1 => Some(FitnessEquipmentStateField::InUse),
      2 => Some(FitnessEquipmentStateField::Paused),
      3 => Some(FitnessEquipmentStateField::Unknown),
      _ => None,
    }
  }
}

/* name: sport_bits_1 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits1Field {
  Tennis = 1,
  AmericanFootball = 2,
  Training = 4,
  Walking = 8,
  CrossCountrySkiing = 16,
  AlpineSkiing = 32,
  Snowboarding = 64,
  Rowing = 128,
}

impl From<SportBits1Field> for u8 {
  fn from(source: SportBits1Field) -> u8 {
    match source {
      SportBits1Field::Tennis => 1,
      SportBits1Field::AmericanFootball => 2,
      SportBits1Field::Training => 4,
      SportBits1Field::Walking => 8,
      SportBits1Field::CrossCountrySkiing => 16,
      SportBits1Field::AlpineSkiing => 32,
      SportBits1Field::Snowboarding => 64,
      SportBits1Field::Rowing => 128,
    }
  }
}

impl IntoField<u8> for SportBits1Field {
  fn into_field(value: u8) -> Option<SportBits1Field> {
    match value {
      1 => Some(SportBits1Field::Tennis),
      2 => Some(SportBits1Field::AmericanFootball),
      4 => Some(SportBits1Field::Training),
      8 => Some(SportBits1Field::Walking),
      16 => Some(SportBits1Field::CrossCountrySkiing),
      32 => Some(SportBits1Field::AlpineSkiing),
      64 => Some(SportBits1Field::Snowboarding),
      128 => Some(SportBits1Field::Rowing),
      _ => None,
    }
  }
}

/* name: workout_capabilities type: uint32z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutCapabilitiesField {
  Interval = 1,
  Custom = 2,
  FitnessEquipment = 4,
  Firstbeat = 8,
  NewLeaf = 16,
  Tcx = 32,
  Speed = 128,
  HeartRate = 256,
  Distance = 512,
  Cadence = 1024,
  Power = 2048,
  Grade = 4096,
  Resistance = 8192,
  Protected = 16384,
}

impl From<WorkoutCapabilitiesField> for u32 {
  fn from(source: WorkoutCapabilitiesField) -> u32 {
    match source {
      WorkoutCapabilitiesField::Interval => 1,
      WorkoutCapabilitiesField::Custom => 2,
      WorkoutCapabilitiesField::FitnessEquipment => 4,
      WorkoutCapabilitiesField::Firstbeat => 8,
      WorkoutCapabilitiesField::NewLeaf => 16,
      WorkoutCapabilitiesField::Tcx => 32,
      WorkoutCapabilitiesField::Speed => 128,
      WorkoutCapabilitiesField::HeartRate => 256,
      WorkoutCapabilitiesField::Distance => 512,
      WorkoutCapabilitiesField::Cadence => 1024,
      WorkoutCapabilitiesField::Power => 2048,
      WorkoutCapabilitiesField::Grade => 4096,
      WorkoutCapabilitiesField::Resistance => 8192,
      WorkoutCapabilitiesField::Protected => 16384,
    }
  }
}

impl IntoField<u32> for WorkoutCapabilitiesField {
  fn into_field(value: u32) -> Option<WorkoutCapabilitiesField> {
    match value {
      1 => Some(WorkoutCapabilitiesField::Interval),
      2 => Some(WorkoutCapabilitiesField::Custom),
      4 => Some(WorkoutCapabilitiesField::FitnessEquipment),
      8 => Some(WorkoutCapabilitiesField::Firstbeat),
      16 => Some(WorkoutCapabilitiesField::NewLeaf),
      32 => Some(WorkoutCapabilitiesField::Tcx),
      128 => Some(WorkoutCapabilitiesField::Speed),
      256 => Some(WorkoutCapabilitiesField::HeartRate),
      512 => Some(WorkoutCapabilitiesField::Distance),
      1024 => Some(WorkoutCapabilitiesField::Cadence),
      2048 => Some(WorkoutCapabilitiesField::Power),
      4096 => Some(WorkoutCapabilitiesField::Grade),
      8192 => Some(WorkoutCapabilitiesField::Resistance),
      16384 => Some(WorkoutCapabilitiesField::Protected),
      _ => None,
    }
  }
}

/* name: segment_delete_status type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SegmentDeleteStatusField {
  DoNotDelete = 0,
  DeleteOne = 1,
  DeleteAll = 2,
}

impl From<SegmentDeleteStatusField> for u8 {
  fn from(source: SegmentDeleteStatusField) -> u8 {
    match source {
      SegmentDeleteStatusField::DoNotDelete => 0,
      SegmentDeleteStatusField::DeleteOne => 1,
      SegmentDeleteStatusField::DeleteAll => 2,
    }
  }
}

impl IntoField<u8> for SegmentDeleteStatusField {
  fn into_field(value: u8) -> Option<SegmentDeleteStatusField> {
    match value {
      0 => Some(SegmentDeleteStatusField::DoNotDelete),
      1 => Some(SegmentDeleteStatusField::DeleteOne),
      2 => Some(SegmentDeleteStatusField::DeleteAll),
      _ => None,
    }
  }
}

/* name: sport_bits_6 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits6Field {
  FloorClimbing = 1,
}

impl From<SportBits6Field> for u8 {
  fn from(source: SportBits6Field) -> u8 {
    match source {
      SportBits6Field::FloorClimbing => 1,
    }
  }
}

impl IntoField<u8> for SportBits6Field {
  fn into_field(value: u8) -> Option<SportBits6Field> {
    match value {
      1 => Some(SportBits6Field::FloorClimbing),
      _ => None,
    }
  }
}

/* name: swim_stroke type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SwimStrokeField {
  Freestyle = 0,
  Backstroke = 1,
  Breaststroke = 2,
  Butterfly = 3,
  Drill = 4,
  Mixed = 5,
  Im = 6,
}

impl From<SwimStrokeField> for u8 {
  fn from(source: SwimStrokeField) -> u8 {
    match source {
      SwimStrokeField::Freestyle => 0,
      SwimStrokeField::Backstroke => 1,
      SwimStrokeField::Breaststroke => 2,
      SwimStrokeField::Butterfly => 3,
      SwimStrokeField::Drill => 4,
      SwimStrokeField::Mixed => 5,
      SwimStrokeField::Im => 6,
    }
  }
}

impl IntoField<u8> for SwimStrokeField {
  fn into_field(value: u8) -> Option<SwimStrokeField> {
    match value {
      0 => Some(SwimStrokeField::Freestyle),
      1 => Some(SwimStrokeField::Backstroke),
      2 => Some(SwimStrokeField::Breaststroke),
      3 => Some(SwimStrokeField::Butterfly),
      4 => Some(SwimStrokeField::Drill),
      5 => Some(SwimStrokeField::Mixed),
      6 => Some(SwimStrokeField::Im),
      _ => None,
    }
  }
}

/* name: comm_timeout_type type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum CommTimeoutTypeField {
  WildcardPairingTimeout = 0,
  PairingTimeout = 1,
  ConnectionLost = 2,
  ConnectionTimeout = 3,
}

impl From<CommTimeoutTypeField> for u16 {
  fn from(source: CommTimeoutTypeField) -> u16 {
    match source {
      CommTimeoutTypeField::WildcardPairingTimeout => 0,
      CommTimeoutTypeField::PairingTimeout => 1,
      CommTimeoutTypeField::ConnectionLost => 2,
      CommTimeoutTypeField::ConnectionTimeout => 3,
    }
  }
}

impl IntoField<u16> for CommTimeoutTypeField {
  fn into_field(value: u16) -> Option<CommTimeoutTypeField> {
    match value {
      0 => Some(CommTimeoutTypeField::WildcardPairingTimeout),
      1 => Some(CommTimeoutTypeField::PairingTimeout),
      2 => Some(CommTimeoutTypeField::ConnectionLost),
      3 => Some(CommTimeoutTypeField::ConnectionTimeout),
      _ => None,
    }
  }
}

/* name: user_local_id type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum UserLocalIdField {
  LocalMin = 0,
  LocalMax = 15,
  StationaryMin = 16,
  StationaryMax = 255,
  PortableMin = 256,
  PortableMax = 65534,
}

impl From<UserLocalIdField> for u16 {
  fn from(source: UserLocalIdField) -> u16 {
    match source {
      UserLocalIdField::LocalMin => 0,
      UserLocalIdField::LocalMax => 15,
      UserLocalIdField::StationaryMin => 16,
      UserLocalIdField::StationaryMax => 255,
      UserLocalIdField::PortableMin => 256,
      UserLocalIdField::PortableMax => 65534,
    }
  }
}

impl IntoField<u16> for UserLocalIdField {
  fn into_field(value: u16) -> Option<UserLocalIdField> {
    match value {
      0 => Some(UserLocalIdField::LocalMin),
      15 => Some(UserLocalIdField::LocalMax),
      16 => Some(UserLocalIdField::StationaryMin),
      255 => Some(UserLocalIdField::StationaryMax),
      256 => Some(UserLocalIdField::PortableMin),
      65534 => Some(UserLocalIdField::PortableMax),
      _ => None,
    }
  }
}

/* name: intensity type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum IntensityField {
  Active = 0,
  Rest = 1,
  Warmup = 2,
  Cooldown = 3,
}

impl From<IntensityField> for u8 {
  fn from(source: IntensityField) -> u8 {
    match source {
      IntensityField::Active => 0,
      IntensityField::Rest => 1,
      IntensityField::Warmup => 2,
      IntensityField::Cooldown => 3,
    }
  }
}

impl IntoField<u8> for IntensityField {
  fn into_field(value: u8) -> Option<IntensityField> {
    match value {
      0 => Some(IntensityField::Active),
      1 => Some(IntensityField::Rest),
      2 => Some(IntensityField::Warmup),
      3 => Some(IntensityField::Cooldown),
      _ => None,
    }
  }
}

/* name: exd_descriptors type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ExdDescriptorsField {
  BikeLightBatteryStatus = 0,
  BeamAngleStatus = 1,
  BateryLevel = 2,
  LightNetworkMode = 3,
  NumberLightsConnected = 4,
  Cadence = 5,
  Distance = 6,
  EstimatedTimeOfArrival = 7,
  Heading = 8,
  Time = 9,
  BatteryLevel = 10,
  TrainerResistance = 11,
  TrainerTargetPower = 12,
  TimeSeated = 13,
  TimeStanding = 14,
  Elevation = 15,
  Grade = 16,
  Ascent = 17,
  Descent = 18,
  VerticalSpeed = 19,
  Di2BatteryLevel = 20,
  FrontGear = 21,
  RearGear = 22,
  GearRatio = 23,
  HeartRate = 24,
  HeartRateZone = 25,
  TimeInHeartRateZone = 26,
  HeartRateReserve = 27,
  Calories = 28,
  GpsAccuracy = 29,
  GpsSignalStrength = 30,
  Temperature = 31,
  TimeOfDay = 32,
  Balance = 33,
  PedalSmoothness = 34,
  Power = 35,
  FunctionalThresholdPower = 36,
  IntensityFactor = 37,
  Work = 38,
  PowerRatio = 39,
  NormalizedPower = 40,
  TrainingStressScore = 41,
  TimeOnZone = 42,
  Speed = 43,
  Laps = 44,
  Reps = 45,
  WorkoutStep = 46,
  CourseDistance = 47,
  NavigationDistance = 48,
  CourseEstimatedTimeOfArrival = 49,
  NavigationEstimatedTimeOfArrival = 50,
  CourseTime = 51,
  NavigationTime = 52,
  CourseHeading = 53,
  NavigationHeading = 54,
  PowerZone = 55,
  TorqueEffectiveness = 56,
  TimerTime = 57,
  PowerWeightRatio = 58,
  LeftPlatformCenterOffset = 59,
  RightPlatformCenterOffset = 60,
  LeftPowerPhaseStartAngle = 61,
  RightPowerPhaseStartAngle = 62,
  LeftPowerPhaseFinishAngle = 63,
  RightPowerPhaseFinishAngle = 64,
  Gears = 65,
  Pace = 66,
  TrainingEffect = 67,
  VerticalOscillation = 68,
  VerticalRatio = 69,
  GroundContactTime = 70,
  LeftGroundContactTimeBalance = 71,
  RightGroundContactTimeBalance = 72,
  StrideLength = 73,
  RunningCadence = 74,
  PerformanceCondition = 75,
  CourseType = 76,
  TimeInPowerZone = 77,
  NavigationTurn = 78,
  CourseLocation = 79,
  NavigationLocation = 80,
  Compass = 81,
  GearCombo = 82,
  MuscleOxygen = 83,
  Icon = 84,
  CompassHeading = 85,
  GpsHeading = 86,
  GpsElevation = 87,
  AnaerobicTrainingEffect = 88,
  Course = 89,
  OffCourse = 90,
  GlideRatio = 91,
  VerticalDistance = 92,
  Vmg = 93,
  AmbientPressure = 94,
  Pressure = 95,
  Vam = 96,
}

impl From<ExdDescriptorsField> for u8 {
  fn from(source: ExdDescriptorsField) -> u8 {
    match source {
      ExdDescriptorsField::BikeLightBatteryStatus => 0,
      ExdDescriptorsField::BeamAngleStatus => 1,
      ExdDescriptorsField::BateryLevel => 2,
      ExdDescriptorsField::LightNetworkMode => 3,
      ExdDescriptorsField::NumberLightsConnected => 4,
      ExdDescriptorsField::Cadence => 5,
      ExdDescriptorsField::Distance => 6,
      ExdDescriptorsField::EstimatedTimeOfArrival => 7,
      ExdDescriptorsField::Heading => 8,
      ExdDescriptorsField::Time => 9,
      ExdDescriptorsField::BatteryLevel => 10,
      ExdDescriptorsField::TrainerResistance => 11,
      ExdDescriptorsField::TrainerTargetPower => 12,
      ExdDescriptorsField::TimeSeated => 13,
      ExdDescriptorsField::TimeStanding => 14,
      ExdDescriptorsField::Elevation => 15,
      ExdDescriptorsField::Grade => 16,
      ExdDescriptorsField::Ascent => 17,
      ExdDescriptorsField::Descent => 18,
      ExdDescriptorsField::VerticalSpeed => 19,
      ExdDescriptorsField::Di2BatteryLevel => 20,
      ExdDescriptorsField::FrontGear => 21,
      ExdDescriptorsField::RearGear => 22,
      ExdDescriptorsField::GearRatio => 23,
      ExdDescriptorsField::HeartRate => 24,
      ExdDescriptorsField::HeartRateZone => 25,
      ExdDescriptorsField::TimeInHeartRateZone => 26,
      ExdDescriptorsField::HeartRateReserve => 27,
      ExdDescriptorsField::Calories => 28,
      ExdDescriptorsField::GpsAccuracy => 29,
      ExdDescriptorsField::GpsSignalStrength => 30,
      ExdDescriptorsField::Temperature => 31,
      ExdDescriptorsField::TimeOfDay => 32,
      ExdDescriptorsField::Balance => 33,
      ExdDescriptorsField::PedalSmoothness => 34,
      ExdDescriptorsField::Power => 35,
      ExdDescriptorsField::FunctionalThresholdPower => 36,
      ExdDescriptorsField::IntensityFactor => 37,
      ExdDescriptorsField::Work => 38,
      ExdDescriptorsField::PowerRatio => 39,
      ExdDescriptorsField::NormalizedPower => 40,
      ExdDescriptorsField::TrainingStressScore => 41,
      ExdDescriptorsField::TimeOnZone => 42,
      ExdDescriptorsField::Speed => 43,
      ExdDescriptorsField::Laps => 44,
      ExdDescriptorsField::Reps => 45,
      ExdDescriptorsField::WorkoutStep => 46,
      ExdDescriptorsField::CourseDistance => 47,
      ExdDescriptorsField::NavigationDistance => 48,
      ExdDescriptorsField::CourseEstimatedTimeOfArrival => 49,
      ExdDescriptorsField::NavigationEstimatedTimeOfArrival => 50,
      ExdDescriptorsField::CourseTime => 51,
      ExdDescriptorsField::NavigationTime => 52,
      ExdDescriptorsField::CourseHeading => 53,
      ExdDescriptorsField::NavigationHeading => 54,
      ExdDescriptorsField::PowerZone => 55,
      ExdDescriptorsField::TorqueEffectiveness => 56,
      ExdDescriptorsField::TimerTime => 57,
      ExdDescriptorsField::PowerWeightRatio => 58,
      ExdDescriptorsField::LeftPlatformCenterOffset => 59,
      ExdDescriptorsField::RightPlatformCenterOffset => 60,
      ExdDescriptorsField::LeftPowerPhaseStartAngle => 61,
      ExdDescriptorsField::RightPowerPhaseStartAngle => 62,
      ExdDescriptorsField::LeftPowerPhaseFinishAngle => 63,
      ExdDescriptorsField::RightPowerPhaseFinishAngle => 64,
      ExdDescriptorsField::Gears => 65,
      ExdDescriptorsField::Pace => 66,
      ExdDescriptorsField::TrainingEffect => 67,
      ExdDescriptorsField::VerticalOscillation => 68,
      ExdDescriptorsField::VerticalRatio => 69,
      ExdDescriptorsField::GroundContactTime => 70,
      ExdDescriptorsField::LeftGroundContactTimeBalance => 71,
      ExdDescriptorsField::RightGroundContactTimeBalance => 72,
      ExdDescriptorsField::StrideLength => 73,
      ExdDescriptorsField::RunningCadence => 74,
      ExdDescriptorsField::PerformanceCondition => 75,
      ExdDescriptorsField::CourseType => 76,
      ExdDescriptorsField::TimeInPowerZone => 77,
      ExdDescriptorsField::NavigationTurn => 78,
      ExdDescriptorsField::CourseLocation => 79,
      ExdDescriptorsField::NavigationLocation => 80,
      ExdDescriptorsField::Compass => 81,
      ExdDescriptorsField::GearCombo => 82,
      ExdDescriptorsField::MuscleOxygen => 83,
      ExdDescriptorsField::Icon => 84,
      ExdDescriptorsField::CompassHeading => 85,
      ExdDescriptorsField::GpsHeading => 86,
      ExdDescriptorsField::GpsElevation => 87,
      ExdDescriptorsField::AnaerobicTrainingEffect => 88,
      ExdDescriptorsField::Course => 89,
      ExdDescriptorsField::OffCourse => 90,
      ExdDescriptorsField::GlideRatio => 91,
      ExdDescriptorsField::VerticalDistance => 92,
      ExdDescriptorsField::Vmg => 93,
      ExdDescriptorsField::AmbientPressure => 94,
      ExdDescriptorsField::Pressure => 95,
      ExdDescriptorsField::Vam => 96,
    }
  }
}

impl IntoField<u8> for ExdDescriptorsField {
  fn into_field(value: u8) -> Option<ExdDescriptorsField> {
    match value {
      0 => Some(ExdDescriptorsField::BikeLightBatteryStatus),
      1 => Some(ExdDescriptorsField::BeamAngleStatus),
      2 => Some(ExdDescriptorsField::BateryLevel),
      3 => Some(ExdDescriptorsField::LightNetworkMode),
      4 => Some(ExdDescriptorsField::NumberLightsConnected),
      5 => Some(ExdDescriptorsField::Cadence),
      6 => Some(ExdDescriptorsField::Distance),
      7 => Some(ExdDescriptorsField::EstimatedTimeOfArrival),
      8 => Some(ExdDescriptorsField::Heading),
      9 => Some(ExdDescriptorsField::Time),
      10 => Some(ExdDescriptorsField::BatteryLevel),
      11 => Some(ExdDescriptorsField::TrainerResistance),
      12 => Some(ExdDescriptorsField::TrainerTargetPower),
      13 => Some(ExdDescriptorsField::TimeSeated),
      14 => Some(ExdDescriptorsField::TimeStanding),
      15 => Some(ExdDescriptorsField::Elevation),
      16 => Some(ExdDescriptorsField::Grade),
      17 => Some(ExdDescriptorsField::Ascent),
      18 => Some(ExdDescriptorsField::Descent),
      19 => Some(ExdDescriptorsField::VerticalSpeed),
      20 => Some(ExdDescriptorsField::Di2BatteryLevel),
      21 => Some(ExdDescriptorsField::FrontGear),
      22 => Some(ExdDescriptorsField::RearGear),
      23 => Some(ExdDescriptorsField::GearRatio),
      24 => Some(ExdDescriptorsField::HeartRate),
      25 => Some(ExdDescriptorsField::HeartRateZone),
      26 => Some(ExdDescriptorsField::TimeInHeartRateZone),
      27 => Some(ExdDescriptorsField::HeartRateReserve),
      28 => Some(ExdDescriptorsField::Calories),
      29 => Some(ExdDescriptorsField::GpsAccuracy),
      30 => Some(ExdDescriptorsField::GpsSignalStrength),
      31 => Some(ExdDescriptorsField::Temperature),
      32 => Some(ExdDescriptorsField::TimeOfDay),
      33 => Some(ExdDescriptorsField::Balance),
      34 => Some(ExdDescriptorsField::PedalSmoothness),
      35 => Some(ExdDescriptorsField::Power),
      36 => Some(ExdDescriptorsField::FunctionalThresholdPower),
      37 => Some(ExdDescriptorsField::IntensityFactor),
      38 => Some(ExdDescriptorsField::Work),
      39 => Some(ExdDescriptorsField::PowerRatio),
      40 => Some(ExdDescriptorsField::NormalizedPower),
      41 => Some(ExdDescriptorsField::TrainingStressScore),
      42 => Some(ExdDescriptorsField::TimeOnZone),
      43 => Some(ExdDescriptorsField::Speed),
      44 => Some(ExdDescriptorsField::Laps),
      45 => Some(ExdDescriptorsField::Reps),
      46 => Some(ExdDescriptorsField::WorkoutStep),
      47 => Some(ExdDescriptorsField::CourseDistance),
      48 => Some(ExdDescriptorsField::NavigationDistance),
      49 => Some(ExdDescriptorsField::CourseEstimatedTimeOfArrival),
      50 => Some(ExdDescriptorsField::NavigationEstimatedTimeOfArrival),
      51 => Some(ExdDescriptorsField::CourseTime),
      52 => Some(ExdDescriptorsField::NavigationTime),
      53 => Some(ExdDescriptorsField::CourseHeading),
      54 => Some(ExdDescriptorsField::NavigationHeading),
      55 => Some(ExdDescriptorsField::PowerZone),
      56 => Some(ExdDescriptorsField::TorqueEffectiveness),
      57 => Some(ExdDescriptorsField::TimerTime),
      58 => Some(ExdDescriptorsField::PowerWeightRatio),
      59 => Some(ExdDescriptorsField::LeftPlatformCenterOffset),
      60 => Some(ExdDescriptorsField::RightPlatformCenterOffset),
      61 => Some(ExdDescriptorsField::LeftPowerPhaseStartAngle),
      62 => Some(ExdDescriptorsField::RightPowerPhaseStartAngle),
      63 => Some(ExdDescriptorsField::LeftPowerPhaseFinishAngle),
      64 => Some(ExdDescriptorsField::RightPowerPhaseFinishAngle),
      65 => Some(ExdDescriptorsField::Gears),
      66 => Some(ExdDescriptorsField::Pace),
      67 => Some(ExdDescriptorsField::TrainingEffect),
      68 => Some(ExdDescriptorsField::VerticalOscillation),
      69 => Some(ExdDescriptorsField::VerticalRatio),
      70 => Some(ExdDescriptorsField::GroundContactTime),
      71 => Some(ExdDescriptorsField::LeftGroundContactTimeBalance),
      72 => Some(ExdDescriptorsField::RightGroundContactTimeBalance),
      73 => Some(ExdDescriptorsField::StrideLength),
      74 => Some(ExdDescriptorsField::RunningCadence),
      75 => Some(ExdDescriptorsField::PerformanceCondition),
      76 => Some(ExdDescriptorsField::CourseType),
      77 => Some(ExdDescriptorsField::TimeInPowerZone),
      78 => Some(ExdDescriptorsField::NavigationTurn),
      79 => Some(ExdDescriptorsField::CourseLocation),
      80 => Some(ExdDescriptorsField::NavigationLocation),
      81 => Some(ExdDescriptorsField::Compass),
      82 => Some(ExdDescriptorsField::GearCombo),
      83 => Some(ExdDescriptorsField::MuscleOxygen),
      84 => Some(ExdDescriptorsField::Icon),
      85 => Some(ExdDescriptorsField::CompassHeading),
      86 => Some(ExdDescriptorsField::GpsHeading),
      87 => Some(ExdDescriptorsField::GpsElevation),
      88 => Some(ExdDescriptorsField::AnaerobicTrainingEffect),
      89 => Some(ExdDescriptorsField::Course),
      90 => Some(ExdDescriptorsField::OffCourse),
      91 => Some(ExdDescriptorsField::GlideRatio),
      92 => Some(ExdDescriptorsField::VerticalDistance),
      93 => Some(ExdDescriptorsField::Vmg),
      94 => Some(ExdDescriptorsField::AmbientPressure),
      95 => Some(ExdDescriptorsField::Pressure),
      96 => Some(ExdDescriptorsField::Vam),
      _ => None,
    }
  }
}

/* name: localtime_into_day type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LocaltimeIntoDayField {
}

impl From<LocaltimeIntoDayField> for u32 {
  fn from(source: LocaltimeIntoDayField) -> u32 {
    match source {
    }
  }
}

impl IntoField<u32> for LocaltimeIntoDayField {
  fn into_field(value: u32) -> Option<LocaltimeIntoDayField> {
    match value {
      _ => None,
    }
  }
}

/* name: language_bits_3 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageBits3Field {
  Bulgarian = 1,
  Romanian = 2,
  Chinese = 4,
  Japanese = 8,
  Korean = 16,
  Taiwanese = 32,
  Thai = 64,
  Hebrew = 128,
}

impl From<LanguageBits3Field> for u8 {
  fn from(source: LanguageBits3Field) -> u8 {
    match source {
      LanguageBits3Field::Bulgarian => 1,
      LanguageBits3Field::Romanian => 2,
      LanguageBits3Field::Chinese => 4,
      LanguageBits3Field::Japanese => 8,
      LanguageBits3Field::Korean => 16,
      LanguageBits3Field::Taiwanese => 32,
      LanguageBits3Field::Thai => 64,
      LanguageBits3Field::Hebrew => 128,
    }
  }
}

impl IntoField<u8> for LanguageBits3Field {
  fn into_field(value: u8) -> Option<LanguageBits3Field> {
    match value {
      1 => Some(LanguageBits3Field::Bulgarian),
      2 => Some(LanguageBits3Field::Romanian),
      4 => Some(LanguageBits3Field::Chinese),
      8 => Some(LanguageBits3Field::Japanese),
      16 => Some(LanguageBits3Field::Korean),
      32 => Some(LanguageBits3Field::Taiwanese),
      64 => Some(LanguageBits3Field::Thai),
      128 => Some(LanguageBits3Field::Hebrew),
      _ => None,
    }
  }
}

/* name: activity_class type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ActivityClassField {
  LevelMax = 100,
  Level = 127,
  Athlete = 128,
}

impl From<ActivityClassField> for u8 {
  fn from(source: ActivityClassField) -> u8 {
    match source {
      ActivityClassField::LevelMax => 100,
      ActivityClassField::Level => 127,
      ActivityClassField::Athlete => 128,
    }
  }
}

impl IntoField<u8> for ActivityClassField {
  fn into_field(value: u8) -> Option<ActivityClassField> {
    match value {
      100 => Some(ActivityClassField::LevelMax),
      127 => Some(ActivityClassField::Level),
      128 => Some(ActivityClassField::Athlete),
      _ => None,
    }
  }
}

/* name: bp_status type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BpStatusField {
  NoError = 0,
  ErrorIncompleteData = 1,
  ErrorNoMeasurement = 2,
  ErrorDataOutOfRange = 3,
  ErrorIrregularHeartRate = 4,
}

impl From<BpStatusField> for u8 {
  fn from(source: BpStatusField) -> u8 {
    match source {
      BpStatusField::NoError => 0,
      BpStatusField::ErrorIncompleteData => 1,
      BpStatusField::ErrorNoMeasurement => 2,
      BpStatusField::ErrorDataOutOfRange => 3,
      BpStatusField::ErrorIrregularHeartRate => 4,
    }
  }
}

impl IntoField<u8> for BpStatusField {
  fn into_field(value: u8) -> Option<BpStatusField> {
    match value {
      0 => Some(BpStatusField::NoError),
      1 => Some(BpStatusField::ErrorIncompleteData),
      2 => Some(BpStatusField::ErrorNoMeasurement),
      3 => Some(BpStatusField::ErrorDataOutOfRange),
      4 => Some(BpStatusField::ErrorIrregularHeartRate),
      _ => None,
    }
  }
}

/* name: exd_display_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ExdDisplayTypeField {
  Numerical = 0,
  Simple = 1,
  Graph = 2,
  Bar = 3,
  CircleGraph = 4,
  VirtualPartner = 5,
  Balance = 6,
  StringList = 7,
  String = 8,
  SimpleDynamicIcon = 9,
  Gauge = 10,
}

impl From<ExdDisplayTypeField> for u8 {
  fn from(source: ExdDisplayTypeField) -> u8 {
    match source {
      ExdDisplayTypeField::Numerical => 0,
      ExdDisplayTypeField::Simple => 1,
      ExdDisplayTypeField::Graph => 2,
      ExdDisplayTypeField::Bar => 3,
      ExdDisplayTypeField::CircleGraph => 4,
      ExdDisplayTypeField::VirtualPartner => 5,
      ExdDisplayTypeField::Balance => 6,
      ExdDisplayTypeField::StringList => 7,
      ExdDisplayTypeField::String => 8,
      ExdDisplayTypeField::SimpleDynamicIcon => 9,
      ExdDisplayTypeField::Gauge => 10,
    }
  }
}

impl IntoField<u8> for ExdDisplayTypeField {
  fn into_field(value: u8) -> Option<ExdDisplayTypeField> {
    match value {
      0 => Some(ExdDisplayTypeField::Numerical),
      1 => Some(ExdDisplayTypeField::Simple),
      2 => Some(ExdDisplayTypeField::Graph),
      3 => Some(ExdDisplayTypeField::Bar),
      4 => Some(ExdDisplayTypeField::CircleGraph),
      5 => Some(ExdDisplayTypeField::VirtualPartner),
      6 => Some(ExdDisplayTypeField::Balance),
      7 => Some(ExdDisplayTypeField::StringList),
      8 => Some(ExdDisplayTypeField::String),
      9 => Some(ExdDisplayTypeField::SimpleDynamicIcon),
      10 => Some(ExdDisplayTypeField::Gauge),
      _ => None,
    }
  }
}

/* name: watchface_mode type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WatchfaceModeField {
  Digital = 0,
  Analog = 1,
  ConnectIq = 2,
  Disabled = 3,
}

impl From<WatchfaceModeField> for u8 {
  fn from(source: WatchfaceModeField) -> u8 {
    match source {
      WatchfaceModeField::Digital => 0,
      WatchfaceModeField::Analog => 1,
      WatchfaceModeField::ConnectIq => 2,
      WatchfaceModeField::Disabled => 3,
    }
  }
}

impl IntoField<u8> for WatchfaceModeField {
  fn into_field(value: u8) -> Option<WatchfaceModeField> {
    match value {
      0 => Some(WatchfaceModeField::Digital),
      1 => Some(WatchfaceModeField::Analog),
      2 => Some(WatchfaceModeField::ConnectIq),
      3 => Some(WatchfaceModeField::Disabled),
      _ => None,
    }
  }
}

/* name: time_mode type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimeModeField {
  Hour12 = 0,
  Hour24 = 1,
  Military = 2,
  Hour12WithSeconds = 3,
  Hour24WithSeconds = 4,
  Utc = 5,
}

impl From<TimeModeField> for u8 {
  fn from(source: TimeModeField) -> u8 {
    match source {
      TimeModeField::Hour12 => 0,
      TimeModeField::Hour24 => 1,
      TimeModeField::Military => 2,
      TimeModeField::Hour12WithSeconds => 3,
      TimeModeField::Hour24WithSeconds => 4,
      TimeModeField::Utc => 5,
    }
  }
}

impl IntoField<u8> for TimeModeField {
  fn into_field(value: u8) -> Option<TimeModeField> {
    match value {
      0 => Some(TimeModeField::Hour12),
      1 => Some(TimeModeField::Hour24),
      2 => Some(TimeModeField::Military),
      3 => Some(TimeModeField::Hour12WithSeconds),
      4 => Some(TimeModeField::Hour24WithSeconds),
      5 => Some(TimeModeField::Utc),
      _ => None,
    }
  }
}

/* name: sport type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportField {
  Generic = 0,
  Running = 1,
  Cycling = 2,
  Transition = 3,
  FitnessEquipment = 4,
  Swimming = 5,
  Basketball = 6,
  Soccer = 7,
  Tennis = 8,
  AmericanFootball = 9,
  Training = 10,
  Walking = 11,
  CrossCountrySkiing = 12,
  AlpineSkiing = 13,
  Snowboarding = 14,
  Rowing = 15,
  Mountaineering = 16,
  Hiking = 17,
  Multisport = 18,
  Paddling = 19,
  Flying = 20,
  EBiking = 21,
  Motorcycling = 22,
  Boating = 23,
  Driving = 24,
  Golf = 25,
  HangGliding = 26,
  HorsebackRiding = 27,
  Hunting = 28,
  Fishing = 29,
  InlineSkating = 30,
  RockClimbing = 31,
  Sailing = 32,
  IceSkating = 33,
  SkyDiving = 34,
  Snowshoeing = 35,
  Snowmobiling = 36,
  StandUpPaddleboarding = 37,
  Surfing = 38,
  Wakeboarding = 39,
  WaterSkiing = 40,
  Kayaking = 41,
  Rafting = 42,
  Windsurfing = 43,
  Kitesurfing = 44,
  Tactical = 45,
  Jumpmaster = 46,
  Boxing = 47,
  FloorClimbing = 48,
  All = 254,
}

impl From<SportField> for u8 {
  fn from(source: SportField) -> u8 {
    match source {
      SportField::Generic => 0,
      SportField::Running => 1,
      SportField::Cycling => 2,
      SportField::Transition => 3,
      SportField::FitnessEquipment => 4,
      SportField::Swimming => 5,
      SportField::Basketball => 6,
      SportField::Soccer => 7,
      SportField::Tennis => 8,
      SportField::AmericanFootball => 9,
      SportField::Training => 10,
      SportField::Walking => 11,
      SportField::CrossCountrySkiing => 12,
      SportField::AlpineSkiing => 13,
      SportField::Snowboarding => 14,
      SportField::Rowing => 15,
      SportField::Mountaineering => 16,
      SportField::Hiking => 17,
      SportField::Multisport => 18,
      SportField::Paddling => 19,
      SportField::Flying => 20,
      SportField::EBiking => 21,
      SportField::Motorcycling => 22,
      SportField::Boating => 23,
      SportField::Driving => 24,
      SportField::Golf => 25,
      SportField::HangGliding => 26,
      SportField::HorsebackRiding => 27,
      SportField::Hunting => 28,
      SportField::Fishing => 29,
      SportField::InlineSkating => 30,
      SportField::RockClimbing => 31,
      SportField::Sailing => 32,
      SportField::IceSkating => 33,
      SportField::SkyDiving => 34,
      SportField::Snowshoeing => 35,
      SportField::Snowmobiling => 36,
      SportField::StandUpPaddleboarding => 37,
      SportField::Surfing => 38,
      SportField::Wakeboarding => 39,
      SportField::WaterSkiing => 40,
      SportField::Kayaking => 41,
      SportField::Rafting => 42,
      SportField::Windsurfing => 43,
      SportField::Kitesurfing => 44,
      SportField::Tactical => 45,
      SportField::Jumpmaster => 46,
      SportField::Boxing => 47,
      SportField::FloorClimbing => 48,
      SportField::All => 254,
    }
  }
}

impl IntoField<u8> for SportField {
  fn into_field(value: u8) -> Option<SportField> {
    match value {
      0 => Some(SportField::Generic),
      1 => Some(SportField::Running),
      2 => Some(SportField::Cycling),
      3 => Some(SportField::Transition),
      4 => Some(SportField::FitnessEquipment),
      5 => Some(SportField::Swimming),
      6 => Some(SportField::Basketball),
      7 => Some(SportField::Soccer),
      8 => Some(SportField::Tennis),
      9 => Some(SportField::AmericanFootball),
      10 => Some(SportField::Training),
      11 => Some(SportField::Walking),
      12 => Some(SportField::CrossCountrySkiing),
      13 => Some(SportField::AlpineSkiing),
      14 => Some(SportField::Snowboarding),
      15 => Some(SportField::Rowing),
      16 => Some(SportField::Mountaineering),
      17 => Some(SportField::Hiking),
      18 => Some(SportField::Multisport),
      19 => Some(SportField::Paddling),
      20 => Some(SportField::Flying),
      21 => Some(SportField::EBiking),
      22 => Some(SportField::Motorcycling),
      23 => Some(SportField::Boating),
      24 => Some(SportField::Driving),
      25 => Some(SportField::Golf),
      26 => Some(SportField::HangGliding),
      27 => Some(SportField::HorsebackRiding),
      28 => Some(SportField::Hunting),
      29 => Some(SportField::Fishing),
      30 => Some(SportField::InlineSkating),
      31 => Some(SportField::RockClimbing),
      32 => Some(SportField::Sailing),
      33 => Some(SportField::IceSkating),
      34 => Some(SportField::SkyDiving),
      35 => Some(SportField::Snowshoeing),
      36 => Some(SportField::Snowmobiling),
      37 => Some(SportField::StandUpPaddleboarding),
      38 => Some(SportField::Surfing),
      39 => Some(SportField::Wakeboarding),
      40 => Some(SportField::WaterSkiing),
      41 => Some(SportField::Kayaking),
      42 => Some(SportField::Rafting),
      43 => Some(SportField::Windsurfing),
      44 => Some(SportField::Kitesurfing),
      45 => Some(SportField::Tactical),
      46 => Some(SportField::Jumpmaster),
      47 => Some(SportField::Boxing),
      48 => Some(SportField::FloorClimbing),
      254 => Some(SportField::All),
      _ => None,
    }
  }
}

/* name: autolap_trigger type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AutolapTriggerField {
  Time = 0,
  Distance = 1,
  PositionStart = 2,
  PositionLap = 3,
  PositionWaypoint = 4,
  PositionMarked = 5,
  Off = 6,
}

impl From<AutolapTriggerField> for u8 {
  fn from(source: AutolapTriggerField) -> u8 {
    match source {
      AutolapTriggerField::Time => 0,
      AutolapTriggerField::Distance => 1,
      AutolapTriggerField::PositionStart => 2,
      AutolapTriggerField::PositionLap => 3,
      AutolapTriggerField::PositionWaypoint => 4,
      AutolapTriggerField::PositionMarked => 5,
      AutolapTriggerField::Off => 6,
    }
  }
}

impl IntoField<u8> for AutolapTriggerField {
  fn into_field(value: u8) -> Option<AutolapTriggerField> {
    match value {
      0 => Some(AutolapTriggerField::Time),
      1 => Some(AutolapTriggerField::Distance),
      2 => Some(AutolapTriggerField::PositionStart),
      3 => Some(AutolapTriggerField::PositionLap),
      4 => Some(AutolapTriggerField::PositionWaypoint),
      5 => Some(AutolapTriggerField::PositionMarked),
      6 => Some(AutolapTriggerField::Off),
      _ => None,
    }
  }
}

/* name: day_of_week type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DayOfWeekField {
  Sunday = 0,
  Monday = 1,
  Tuesday = 2,
  Wednesday = 3,
  Thursday = 4,
  Friday = 5,
  Saturday = 6,
}

impl From<DayOfWeekField> for u8 {
  fn from(source: DayOfWeekField) -> u8 {
    match source {
      DayOfWeekField::Sunday => 0,
      DayOfWeekField::Monday => 1,
      DayOfWeekField::Tuesday => 2,
      DayOfWeekField::Wednesday => 3,
      DayOfWeekField::Thursday => 4,
      DayOfWeekField::Friday => 5,
      DayOfWeekField::Saturday => 6,
    }
  }
}

impl IntoField<u8> for DayOfWeekField {
  fn into_field(value: u8) -> Option<DayOfWeekField> {
    match value {
      0 => Some(DayOfWeekField::Sunday),
      1 => Some(DayOfWeekField::Monday),
      2 => Some(DayOfWeekField::Tuesday),
      3 => Some(DayOfWeekField::Wednesday),
      4 => Some(DayOfWeekField::Thursday),
      5 => Some(DayOfWeekField::Friday),
      6 => Some(DayOfWeekField::Saturday),
      _ => None,
    }
  }
}

/* name: weather_status type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WeatherStatusField {
  Clear = 0,
  PartlyCloudy = 1,
  MostlyCloudy = 2,
  Rain = 3,
  Snow = 4,
  Windy = 5,
  Thunderstorms = 6,
  WintryMix = 7,
  Fog = 8,
  Hazy = 11,
  Hail = 12,
  ScatteredShowers = 13,
  ScatteredThunderstorms = 14,
  UnknownPrecipitation = 15,
  LightRain = 16,
  HeavyRain = 17,
  LightSnow = 18,
  HeavySnow = 19,
  LightRainSnow = 20,
  HeavyRainSnow = 21,
  Cloudy = 22,
}

impl From<WeatherStatusField> for u8 {
  fn from(source: WeatherStatusField) -> u8 {
    match source {
      WeatherStatusField::Clear => 0,
      WeatherStatusField::PartlyCloudy => 1,
      WeatherStatusField::MostlyCloudy => 2,
      WeatherStatusField::Rain => 3,
      WeatherStatusField::Snow => 4,
      WeatherStatusField::Windy => 5,
      WeatherStatusField::Thunderstorms => 6,
      WeatherStatusField::WintryMix => 7,
      WeatherStatusField::Fog => 8,
      WeatherStatusField::Hazy => 11,
      WeatherStatusField::Hail => 12,
      WeatherStatusField::ScatteredShowers => 13,
      WeatherStatusField::ScatteredThunderstorms => 14,
      WeatherStatusField::UnknownPrecipitation => 15,
      WeatherStatusField::LightRain => 16,
      WeatherStatusField::HeavyRain => 17,
      WeatherStatusField::LightSnow => 18,
      WeatherStatusField::HeavySnow => 19,
      WeatherStatusField::LightRainSnow => 20,
      WeatherStatusField::HeavyRainSnow => 21,
      WeatherStatusField::Cloudy => 22,
    }
  }
}

impl IntoField<u8> for WeatherStatusField {
  fn into_field(value: u8) -> Option<WeatherStatusField> {
    match value {
      0 => Some(WeatherStatusField::Clear),
      1 => Some(WeatherStatusField::PartlyCloudy),
      2 => Some(WeatherStatusField::MostlyCloudy),
      3 => Some(WeatherStatusField::Rain),
      4 => Some(WeatherStatusField::Snow),
      5 => Some(WeatherStatusField::Windy),
      6 => Some(WeatherStatusField::Thunderstorms),
      7 => Some(WeatherStatusField::WintryMix),
      8 => Some(WeatherStatusField::Fog),
      11 => Some(WeatherStatusField::Hazy),
      12 => Some(WeatherStatusField::Hail),
      13 => Some(WeatherStatusField::ScatteredShowers),
      14 => Some(WeatherStatusField::ScatteredThunderstorms),
      15 => Some(WeatherStatusField::UnknownPrecipitation),
      16 => Some(WeatherStatusField::LightRain),
      17 => Some(WeatherStatusField::HeavyRain),
      18 => Some(WeatherStatusField::LightSnow),
      19 => Some(WeatherStatusField::HeavySnow),
      20 => Some(WeatherStatusField::LightRainSnow),
      21 => Some(WeatherStatusField::HeavyRainSnow),
      22 => Some(WeatherStatusField::Cloudy),
      _ => None,
    }
  }
}

/* name: exd_data_units type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ExdDataUnitsField {
  NoUnits = 0,
  Laps = 1,
  MilesPerHour = 2,
  KilometersPerHour = 3,
  FeetPerHour = 4,
  MetersPerHour = 5,
  DegreesCelsius = 6,
  DegreesFarenheit = 7,
  Zone = 8,
  Gear = 9,
  Rpm = 10,
  Bpm = 11,
  Degrees = 12,
  Millimeters = 13,
  Meters = 14,
  Kilometers = 15,
  Feet = 16,
  Yards = 17,
  Kilofeet = 18,
  Miles = 19,
  Time = 20,
  EnumTurnType = 21,
  Percent = 22,
  Watts = 23,
  WattsPerKilogram = 24,
  EnumBatteryStatus = 25,
  EnumBikeLightBeamAngleMode = 26,
  EnumBikeLightBatteryStatus = 27,
  EnumBikeLightNetworkConfigType = 28,
  Lights = 29,
  Seconds = 30,
  Minutes = 31,
  Hours = 32,
  Calories = 33,
  Kilojoules = 34,
  Milliseconds = 35,
  SecondPerMile = 36,
  SecondPerKilometer = 37,
  Centimeter = 38,
  EnumCoursePoint = 39,
  Bradians = 40,
  EnumSport = 41,
  InchesHg = 42,
  MmHg = 43,
  Mbars = 44,
  HectoPascals = 45,
  FeetPerMin = 46,
  MetersPerMin = 47,
  MetersPerSec = 48,
  EightCardinal = 49,
}

impl From<ExdDataUnitsField> for u8 {
  fn from(source: ExdDataUnitsField) -> u8 {
    match source {
      ExdDataUnitsField::NoUnits => 0,
      ExdDataUnitsField::Laps => 1,
      ExdDataUnitsField::MilesPerHour => 2,
      ExdDataUnitsField::KilometersPerHour => 3,
      ExdDataUnitsField::FeetPerHour => 4,
      ExdDataUnitsField::MetersPerHour => 5,
      ExdDataUnitsField::DegreesCelsius => 6,
      ExdDataUnitsField::DegreesFarenheit => 7,
      ExdDataUnitsField::Zone => 8,
      ExdDataUnitsField::Gear => 9,
      ExdDataUnitsField::Rpm => 10,
      ExdDataUnitsField::Bpm => 11,
      ExdDataUnitsField::Degrees => 12,
      ExdDataUnitsField::Millimeters => 13,
      ExdDataUnitsField::Meters => 14,
      ExdDataUnitsField::Kilometers => 15,
      ExdDataUnitsField::Feet => 16,
      ExdDataUnitsField::Yards => 17,
      ExdDataUnitsField::Kilofeet => 18,
      ExdDataUnitsField::Miles => 19,
      ExdDataUnitsField::Time => 20,
      ExdDataUnitsField::EnumTurnType => 21,
      ExdDataUnitsField::Percent => 22,
      ExdDataUnitsField::Watts => 23,
      ExdDataUnitsField::WattsPerKilogram => 24,
      ExdDataUnitsField::EnumBatteryStatus => 25,
      ExdDataUnitsField::EnumBikeLightBeamAngleMode => 26,
      ExdDataUnitsField::EnumBikeLightBatteryStatus => 27,
      ExdDataUnitsField::EnumBikeLightNetworkConfigType => 28,
      ExdDataUnitsField::Lights => 29,
      ExdDataUnitsField::Seconds => 30,
      ExdDataUnitsField::Minutes => 31,
      ExdDataUnitsField::Hours => 32,
      ExdDataUnitsField::Calories => 33,
      ExdDataUnitsField::Kilojoules => 34,
      ExdDataUnitsField::Milliseconds => 35,
      ExdDataUnitsField::SecondPerMile => 36,
      ExdDataUnitsField::SecondPerKilometer => 37,
      ExdDataUnitsField::Centimeter => 38,
      ExdDataUnitsField::EnumCoursePoint => 39,
      ExdDataUnitsField::Bradians => 40,
      ExdDataUnitsField::EnumSport => 41,
      ExdDataUnitsField::InchesHg => 42,
      ExdDataUnitsField::MmHg => 43,
      ExdDataUnitsField::Mbars => 44,
      ExdDataUnitsField::HectoPascals => 45,
      ExdDataUnitsField::FeetPerMin => 46,
      ExdDataUnitsField::MetersPerMin => 47,
      ExdDataUnitsField::MetersPerSec => 48,
      ExdDataUnitsField::EightCardinal => 49,
    }
  }
}

impl IntoField<u8> for ExdDataUnitsField {
  fn into_field(value: u8) -> Option<ExdDataUnitsField> {
    match value {
      0 => Some(ExdDataUnitsField::NoUnits),
      1 => Some(ExdDataUnitsField::Laps),
      2 => Some(ExdDataUnitsField::MilesPerHour),
      3 => Some(ExdDataUnitsField::KilometersPerHour),
      4 => Some(ExdDataUnitsField::FeetPerHour),
      5 => Some(ExdDataUnitsField::MetersPerHour),
      6 => Some(ExdDataUnitsField::DegreesCelsius),
      7 => Some(ExdDataUnitsField::DegreesFarenheit),
      8 => Some(ExdDataUnitsField::Zone),
      9 => Some(ExdDataUnitsField::Gear),
      10 => Some(ExdDataUnitsField::Rpm),
      11 => Some(ExdDataUnitsField::Bpm),
      12 => Some(ExdDataUnitsField::Degrees),
      13 => Some(ExdDataUnitsField::Millimeters),
      14 => Some(ExdDataUnitsField::Meters),
      15 => Some(ExdDataUnitsField::Kilometers),
      16 => Some(ExdDataUnitsField::Feet),
      17 => Some(ExdDataUnitsField::Yards),
      18 => Some(ExdDataUnitsField::Kilofeet),
      19 => Some(ExdDataUnitsField::Miles),
      20 => Some(ExdDataUnitsField::Time),
      21 => Some(ExdDataUnitsField::EnumTurnType),
      22 => Some(ExdDataUnitsField::Percent),
      23 => Some(ExdDataUnitsField::Watts),
      24 => Some(ExdDataUnitsField::WattsPerKilogram),
      25 => Some(ExdDataUnitsField::EnumBatteryStatus),
      26 => Some(ExdDataUnitsField::EnumBikeLightBeamAngleMode),
      27 => Some(ExdDataUnitsField::EnumBikeLightBatteryStatus),
      28 => Some(ExdDataUnitsField::EnumBikeLightNetworkConfigType),
      29 => Some(ExdDataUnitsField::Lights),
      30 => Some(ExdDataUnitsField::Seconds),
      31 => Some(ExdDataUnitsField::Minutes),
      32 => Some(ExdDataUnitsField::Hours),
      33 => Some(ExdDataUnitsField::Calories),
      34 => Some(ExdDataUnitsField::Kilojoules),
      35 => Some(ExdDataUnitsField::Milliseconds),
      36 => Some(ExdDataUnitsField::SecondPerMile),
      37 => Some(ExdDataUnitsField::SecondPerKilometer),
      38 => Some(ExdDataUnitsField::Centimeter),
      39 => Some(ExdDataUnitsField::EnumCoursePoint),
      40 => Some(ExdDataUnitsField::Bradians),
      41 => Some(ExdDataUnitsField::EnumSport),
      42 => Some(ExdDataUnitsField::InchesHg),
      43 => Some(ExdDataUnitsField::MmHg),
      44 => Some(ExdDataUnitsField::Mbars),
      45 => Some(ExdDataUnitsField::HectoPascals),
      46 => Some(ExdDataUnitsField::FeetPerMin),
      47 => Some(ExdDataUnitsField::MetersPerMin),
      48 => Some(ExdDataUnitsField::MetersPerSec),
      49 => Some(ExdDataUnitsField::EightCardinal),
      _ => None,
    }
  }
}

/* name: garmin_product type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum GarminProductField {
  Hrm1 = 1,
  Axh01 = 2,
  Axb01 = 3,
  Axb02 = 4,
  Hrm2ss = 5,
  DsiAlf02 = 6,
  Hrm3ss = 7,
  HrmRunSingleByteProductId = 8,
  Bsm = 9,
  Bcm = 10,
  Axs01 = 11,
  HrmTriSingleByteProductId = 12,
  Fr225SingleByteProductId = 14,
  Fr301China = 473,
  Fr301Japan = 474,
  Fr301Korea = 475,
  Fr301Taiwan = 494,
  Fr405 = 717,
  Fr50 = 782,
  Fr405Japan = 987,
  Fr60 = 988,
  DsiAlf01 = 1011,
  Fr310xt = 1018,
  Edge500 = 1036,
  Fr110 = 1124,
  Edge800 = 1169,
  Edge500Taiwan = 1199,
  Edge500Japan = 1213,
  Chirp = 1253,
  Fr110Japan = 1274,
  Edge200 = 1325,
  Fr910xt = 1328,
  Edge800Taiwan = 1333,
  Edge800Japan = 1334,
  Alf04 = 1341,
  Fr610 = 1345,
  Fr210Japan = 1360,
  VectorSs = 1380,
  VectorCp = 1381,
  Edge800China = 1386,
  Edge500China = 1387,
  Fr610Japan = 1410,
  Edge500Korea = 1422,
  Fr70 = 1436,
  Fr310xt4t = 1446,
  Amx = 1461,
  Fr10 = 1482,
  Edge800Korea = 1497,
  Swim = 1499,
  Fr910xtChina = 1537,
  Fenix = 1551,
  Edge200Taiwan = 1555,
  Edge510 = 1561,
  Edge810 = 1567,
  Tempe = 1570,
  Fr910xtJapan = 1600,
  Fr620 = 1623,
  Fr220 = 1632,
  Fr910xtKorea = 1664,
  Fr10Japan = 1688,
  Edge810Japan = 1721,
  VirbElite = 1735,
  EdgeTouring = 1736,
  Edge510Japan = 1742,
  HrmTri = 1743,
  HrmRun = 1752,
  Fr920xt = 1765,
  Edge510Asia = 1821,
  Edge810China = 1822,
  Edge810Taiwan = 1823,
  Edge1000 = 1836,
  VivoFit = 1837,
  VirbRemote = 1853,
  VivoKi = 1885,
  Fr15 = 1903,
  VivoActive = 1907,
  Edge510Korea = 1918,
  Fr620Japan = 1928,
  Fr620China = 1929,
  Fr220Japan = 1930,
  Fr220China = 1931,
  ApproachS6 = 1936,
  VivoSmart = 1956,
  Fenix2 = 1967,
  Epix = 1988,
  Fenix3 = 2050,
  Edge1000Taiwan = 2052,
  Edge1000Japan = 2053,
  Fr15Japan = 2061,
  Edge520 = 2067,
  Edge1000China = 2070,
  Fr620Russia = 2072,
  Fr220Russia = 2073,
  VectorS = 2079,
  Edge1000Korea = 2100,
  Fr920xtTaiwan = 2130,
  Fr920xtChina = 2131,
  Fr920xtJapan = 2132,
  Virbx = 2134,
  VivoSmartApac = 2135,
  EtrexTouch = 2140,
  Edge25 = 2147,
  Fr25 = 2148,
  VivoFit2 = 2150,
  Fr225 = 2153,
  Fr630 = 2156,
  Fr230 = 2157,
  VivoActiveApac = 2160,
  Vector2 = 2161,
  Vector2s = 2162,
  Virbxe = 2172,
  Fr620Taiwan = 2173,
  Fr220Taiwan = 2174,
  Truswing = 2175,
  Fenix3China = 2188,
  Fenix3Twn = 2189,
  VariaHeadlight = 2192,
  VariaTaillightOld = 2193,
  EdgeExplore1000 = 2204,
  Fr225Asia = 2219,
  VariaRadarTaillight = 2225,
  VariaRadarDisplay = 2226,
  Edge20 = 2238,
  D2Bravo = 2262,
  ApproachS20 = 2266,
  VariaRemote = 2276,
  Hrm4Run = 2327,
  VivoActiveHr = 2337,
  VivoSmartGpsHr = 2347,
  VivoSmartHr = 2348,
  VivoMove = 2368,
  VariaVision = 2398,
  VivoFit3 = 2406,
  Fenix3Hr = 2413,
  VirbUltra30 = 2417,
  IndexSmartScale = 2429,
  Fr235 = 2431,
  Fenix3Chronos = 2432,
  Oregon7xx = 2441,
  Rino7xx = 2444,
  Nautix = 2496,
  Edge820 = 2530,
  EdgeExplore820 = 2531,
  Fenix5s = 2544,
  D2BravoTitanium = 2547,
  RunningDynamicsPod = 2593,
  Fenix5x = 2604,
  VivoFitJr = 2606,
  Fr935 = 2691,
  Fenix5 = 2697,
  Sdm4 = 10007,
  EdgeRemote = 10014,
  TrainingCenter = 20119,
  ConnectiqSimulator = 65531,
  AndroidAntplusPlugin = 65532,
  Connect = 65534,
}

impl From<GarminProductField> for u16 {
  fn from(source: GarminProductField) -> u16 {
    match source {
      GarminProductField::Hrm1 => 1,
      GarminProductField::Axh01 => 2,
      GarminProductField::Axb01 => 3,
      GarminProductField::Axb02 => 4,
      GarminProductField::Hrm2ss => 5,
      GarminProductField::DsiAlf02 => 6,
      GarminProductField::Hrm3ss => 7,
      GarminProductField::HrmRunSingleByteProductId => 8,
      GarminProductField::Bsm => 9,
      GarminProductField::Bcm => 10,
      GarminProductField::Axs01 => 11,
      GarminProductField::HrmTriSingleByteProductId => 12,
      GarminProductField::Fr225SingleByteProductId => 14,
      GarminProductField::Fr301China => 473,
      GarminProductField::Fr301Japan => 474,
      GarminProductField::Fr301Korea => 475,
      GarminProductField::Fr301Taiwan => 494,
      GarminProductField::Fr405 => 717,
      GarminProductField::Fr50 => 782,
      GarminProductField::Fr405Japan => 987,
      GarminProductField::Fr60 => 988,
      GarminProductField::DsiAlf01 => 1011,
      GarminProductField::Fr310xt => 1018,
      GarminProductField::Edge500 => 1036,
      GarminProductField::Fr110 => 1124,
      GarminProductField::Edge800 => 1169,
      GarminProductField::Edge500Taiwan => 1199,
      GarminProductField::Edge500Japan => 1213,
      GarminProductField::Chirp => 1253,
      GarminProductField::Fr110Japan => 1274,
      GarminProductField::Edge200 => 1325,
      GarminProductField::Fr910xt => 1328,
      GarminProductField::Edge800Taiwan => 1333,
      GarminProductField::Edge800Japan => 1334,
      GarminProductField::Alf04 => 1341,
      GarminProductField::Fr610 => 1345,
      GarminProductField::Fr210Japan => 1360,
      GarminProductField::VectorSs => 1380,
      GarminProductField::VectorCp => 1381,
      GarminProductField::Edge800China => 1386,
      GarminProductField::Edge500China => 1387,
      GarminProductField::Fr610Japan => 1410,
      GarminProductField::Edge500Korea => 1422,
      GarminProductField::Fr70 => 1436,
      GarminProductField::Fr310xt4t => 1446,
      GarminProductField::Amx => 1461,
      GarminProductField::Fr10 => 1482,
      GarminProductField::Edge800Korea => 1497,
      GarminProductField::Swim => 1499,
      GarminProductField::Fr910xtChina => 1537,
      GarminProductField::Fenix => 1551,
      GarminProductField::Edge200Taiwan => 1555,
      GarminProductField::Edge510 => 1561,
      GarminProductField::Edge810 => 1567,
      GarminProductField::Tempe => 1570,
      GarminProductField::Fr910xtJapan => 1600,
      GarminProductField::Fr620 => 1623,
      GarminProductField::Fr220 => 1632,
      GarminProductField::Fr910xtKorea => 1664,
      GarminProductField::Fr10Japan => 1688,
      GarminProductField::Edge810Japan => 1721,
      GarminProductField::VirbElite => 1735,
      GarminProductField::EdgeTouring => 1736,
      GarminProductField::Edge510Japan => 1742,
      GarminProductField::HrmTri => 1743,
      GarminProductField::HrmRun => 1752,
      GarminProductField::Fr920xt => 1765,
      GarminProductField::Edge510Asia => 1821,
      GarminProductField::Edge810China => 1822,
      GarminProductField::Edge810Taiwan => 1823,
      GarminProductField::Edge1000 => 1836,
      GarminProductField::VivoFit => 1837,
      GarminProductField::VirbRemote => 1853,
      GarminProductField::VivoKi => 1885,
      GarminProductField::Fr15 => 1903,
      GarminProductField::VivoActive => 1907,
      GarminProductField::Edge510Korea => 1918,
      GarminProductField::Fr620Japan => 1928,
      GarminProductField::Fr620China => 1929,
      GarminProductField::Fr220Japan => 1930,
      GarminProductField::Fr220China => 1931,
      GarminProductField::ApproachS6 => 1936,
      GarminProductField::VivoSmart => 1956,
      GarminProductField::Fenix2 => 1967,
      GarminProductField::Epix => 1988,
      GarminProductField::Fenix3 => 2050,
      GarminProductField::Edge1000Taiwan => 2052,
      GarminProductField::Edge1000Japan => 2053,
      GarminProductField::Fr15Japan => 2061,
      GarminProductField::Edge520 => 2067,
      GarminProductField::Edge1000China => 2070,
      GarminProductField::Fr620Russia => 2072,
      GarminProductField::Fr220Russia => 2073,
      GarminProductField::VectorS => 2079,
      GarminProductField::Edge1000Korea => 2100,
      GarminProductField::Fr920xtTaiwan => 2130,
      GarminProductField::Fr920xtChina => 2131,
      GarminProductField::Fr920xtJapan => 2132,
      GarminProductField::Virbx => 2134,
      GarminProductField::VivoSmartApac => 2135,
      GarminProductField::EtrexTouch => 2140,
      GarminProductField::Edge25 => 2147,
      GarminProductField::Fr25 => 2148,
      GarminProductField::VivoFit2 => 2150,
      GarminProductField::Fr225 => 2153,
      GarminProductField::Fr630 => 2156,
      GarminProductField::Fr230 => 2157,
      GarminProductField::VivoActiveApac => 2160,
      GarminProductField::Vector2 => 2161,
      GarminProductField::Vector2s => 2162,
      GarminProductField::Virbxe => 2172,
      GarminProductField::Fr620Taiwan => 2173,
      GarminProductField::Fr220Taiwan => 2174,
      GarminProductField::Truswing => 2175,
      GarminProductField::Fenix3China => 2188,
      GarminProductField::Fenix3Twn => 2189,
      GarminProductField::VariaHeadlight => 2192,
      GarminProductField::VariaTaillightOld => 2193,
      GarminProductField::EdgeExplore1000 => 2204,
      GarminProductField::Fr225Asia => 2219,
      GarminProductField::VariaRadarTaillight => 2225,
      GarminProductField::VariaRadarDisplay => 2226,
      GarminProductField::Edge20 => 2238,
      GarminProductField::D2Bravo => 2262,
      GarminProductField::ApproachS20 => 2266,
      GarminProductField::VariaRemote => 2276,
      GarminProductField::Hrm4Run => 2327,
      GarminProductField::VivoActiveHr => 2337,
      GarminProductField::VivoSmartGpsHr => 2347,
      GarminProductField::VivoSmartHr => 2348,
      GarminProductField::VivoMove => 2368,
      GarminProductField::VariaVision => 2398,
      GarminProductField::VivoFit3 => 2406,
      GarminProductField::Fenix3Hr => 2413,
      GarminProductField::VirbUltra30 => 2417,
      GarminProductField::IndexSmartScale => 2429,
      GarminProductField::Fr235 => 2431,
      GarminProductField::Fenix3Chronos => 2432,
      GarminProductField::Oregon7xx => 2441,
      GarminProductField::Rino7xx => 2444,
      GarminProductField::Nautix => 2496,
      GarminProductField::Edge820 => 2530,
      GarminProductField::EdgeExplore820 => 2531,
      GarminProductField::Fenix5s => 2544,
      GarminProductField::D2BravoTitanium => 2547,
      GarminProductField::RunningDynamicsPod => 2593,
      GarminProductField::Fenix5x => 2604,
      GarminProductField::VivoFitJr => 2606,
      GarminProductField::Fr935 => 2691,
      GarminProductField::Fenix5 => 2697,
      GarminProductField::Sdm4 => 10007,
      GarminProductField::EdgeRemote => 10014,
      GarminProductField::TrainingCenter => 20119,
      GarminProductField::ConnectiqSimulator => 65531,
      GarminProductField::AndroidAntplusPlugin => 65532,
      GarminProductField::Connect => 65534,
    }
  }
}

impl IntoField<u16> for GarminProductField {
  fn into_field(value: u16) -> Option<GarminProductField> {
    match value {
      1 => Some(GarminProductField::Hrm1),
      2 => Some(GarminProductField::Axh01),
      3 => Some(GarminProductField::Axb01),
      4 => Some(GarminProductField::Axb02),
      5 => Some(GarminProductField::Hrm2ss),
      6 => Some(GarminProductField::DsiAlf02),
      7 => Some(GarminProductField::Hrm3ss),
      8 => Some(GarminProductField::HrmRunSingleByteProductId),
      9 => Some(GarminProductField::Bsm),
      10 => Some(GarminProductField::Bcm),
      11 => Some(GarminProductField::Axs01),
      12 => Some(GarminProductField::HrmTriSingleByteProductId),
      14 => Some(GarminProductField::Fr225SingleByteProductId),
      473 => Some(GarminProductField::Fr301China),
      474 => Some(GarminProductField::Fr301Japan),
      475 => Some(GarminProductField::Fr301Korea),
      494 => Some(GarminProductField::Fr301Taiwan),
      717 => Some(GarminProductField::Fr405),
      782 => Some(GarminProductField::Fr50),
      987 => Some(GarminProductField::Fr405Japan),
      988 => Some(GarminProductField::Fr60),
      1011 => Some(GarminProductField::DsiAlf01),
      1018 => Some(GarminProductField::Fr310xt),
      1036 => Some(GarminProductField::Edge500),
      1124 => Some(GarminProductField::Fr110),
      1169 => Some(GarminProductField::Edge800),
      1199 => Some(GarminProductField::Edge500Taiwan),
      1213 => Some(GarminProductField::Edge500Japan),
      1253 => Some(GarminProductField::Chirp),
      1274 => Some(GarminProductField::Fr110Japan),
      1325 => Some(GarminProductField::Edge200),
      1328 => Some(GarminProductField::Fr910xt),
      1333 => Some(GarminProductField::Edge800Taiwan),
      1334 => Some(GarminProductField::Edge800Japan),
      1341 => Some(GarminProductField::Alf04),
      1345 => Some(GarminProductField::Fr610),
      1360 => Some(GarminProductField::Fr210Japan),
      1380 => Some(GarminProductField::VectorSs),
      1381 => Some(GarminProductField::VectorCp),
      1386 => Some(GarminProductField::Edge800China),
      1387 => Some(GarminProductField::Edge500China),
      1410 => Some(GarminProductField::Fr610Japan),
      1422 => Some(GarminProductField::Edge500Korea),
      1436 => Some(GarminProductField::Fr70),
      1446 => Some(GarminProductField::Fr310xt4t),
      1461 => Some(GarminProductField::Amx),
      1482 => Some(GarminProductField::Fr10),
      1497 => Some(GarminProductField::Edge800Korea),
      1499 => Some(GarminProductField::Swim),
      1537 => Some(GarminProductField::Fr910xtChina),
      1551 => Some(GarminProductField::Fenix),
      1555 => Some(GarminProductField::Edge200Taiwan),
      1561 => Some(GarminProductField::Edge510),
      1567 => Some(GarminProductField::Edge810),
      1570 => Some(GarminProductField::Tempe),
      1600 => Some(GarminProductField::Fr910xtJapan),
      1623 => Some(GarminProductField::Fr620),
      1632 => Some(GarminProductField::Fr220),
      1664 => Some(GarminProductField::Fr910xtKorea),
      1688 => Some(GarminProductField::Fr10Japan),
      1721 => Some(GarminProductField::Edge810Japan),
      1735 => Some(GarminProductField::VirbElite),
      1736 => Some(GarminProductField::EdgeTouring),
      1742 => Some(GarminProductField::Edge510Japan),
      1743 => Some(GarminProductField::HrmTri),
      1752 => Some(GarminProductField::HrmRun),
      1765 => Some(GarminProductField::Fr920xt),
      1821 => Some(GarminProductField::Edge510Asia),
      1822 => Some(GarminProductField::Edge810China),
      1823 => Some(GarminProductField::Edge810Taiwan),
      1836 => Some(GarminProductField::Edge1000),
      1837 => Some(GarminProductField::VivoFit),
      1853 => Some(GarminProductField::VirbRemote),
      1885 => Some(GarminProductField::VivoKi),
      1903 => Some(GarminProductField::Fr15),
      1907 => Some(GarminProductField::VivoActive),
      1918 => Some(GarminProductField::Edge510Korea),
      1928 => Some(GarminProductField::Fr620Japan),
      1929 => Some(GarminProductField::Fr620China),
      1930 => Some(GarminProductField::Fr220Japan),
      1931 => Some(GarminProductField::Fr220China),
      1936 => Some(GarminProductField::ApproachS6),
      1956 => Some(GarminProductField::VivoSmart),
      1967 => Some(GarminProductField::Fenix2),
      1988 => Some(GarminProductField::Epix),
      2050 => Some(GarminProductField::Fenix3),
      2052 => Some(GarminProductField::Edge1000Taiwan),
      2053 => Some(GarminProductField::Edge1000Japan),
      2061 => Some(GarminProductField::Fr15Japan),
      2067 => Some(GarminProductField::Edge520),
      2070 => Some(GarminProductField::Edge1000China),
      2072 => Some(GarminProductField::Fr620Russia),
      2073 => Some(GarminProductField::Fr220Russia),
      2079 => Some(GarminProductField::VectorS),
      2100 => Some(GarminProductField::Edge1000Korea),
      2130 => Some(GarminProductField::Fr920xtTaiwan),
      2131 => Some(GarminProductField::Fr920xtChina),
      2132 => Some(GarminProductField::Fr920xtJapan),
      2134 => Some(GarminProductField::Virbx),
      2135 => Some(GarminProductField::VivoSmartApac),
      2140 => Some(GarminProductField::EtrexTouch),
      2147 => Some(GarminProductField::Edge25),
      2148 => Some(GarminProductField::Fr25),
      2150 => Some(GarminProductField::VivoFit2),
      2153 => Some(GarminProductField::Fr225),
      2156 => Some(GarminProductField::Fr630),
      2157 => Some(GarminProductField::Fr230),
      2160 => Some(GarminProductField::VivoActiveApac),
      2161 => Some(GarminProductField::Vector2),
      2162 => Some(GarminProductField::Vector2s),
      2172 => Some(GarminProductField::Virbxe),
      2173 => Some(GarminProductField::Fr620Taiwan),
      2174 => Some(GarminProductField::Fr220Taiwan),
      2175 => Some(GarminProductField::Truswing),
      2188 => Some(GarminProductField::Fenix3China),
      2189 => Some(GarminProductField::Fenix3Twn),
      2192 => Some(GarminProductField::VariaHeadlight),
      2193 => Some(GarminProductField::VariaTaillightOld),
      2204 => Some(GarminProductField::EdgeExplore1000),
      2219 => Some(GarminProductField::Fr225Asia),
      2225 => Some(GarminProductField::VariaRadarTaillight),
      2226 => Some(GarminProductField::VariaRadarDisplay),
      2238 => Some(GarminProductField::Edge20),
      2262 => Some(GarminProductField::D2Bravo),
      2266 => Some(GarminProductField::ApproachS20),
      2276 => Some(GarminProductField::VariaRemote),
      2327 => Some(GarminProductField::Hrm4Run),
      2337 => Some(GarminProductField::VivoActiveHr),
      2347 => Some(GarminProductField::VivoSmartGpsHr),
      2348 => Some(GarminProductField::VivoSmartHr),
      2368 => Some(GarminProductField::VivoMove),
      2398 => Some(GarminProductField::VariaVision),
      2406 => Some(GarminProductField::VivoFit3),
      2413 => Some(GarminProductField::Fenix3Hr),
      2417 => Some(GarminProductField::VirbUltra30),
      2429 => Some(GarminProductField::IndexSmartScale),
      2431 => Some(GarminProductField::Fr235),
      2432 => Some(GarminProductField::Fenix3Chronos),
      2441 => Some(GarminProductField::Oregon7xx),
      2444 => Some(GarminProductField::Rino7xx),
      2496 => Some(GarminProductField::Nautix),
      2530 => Some(GarminProductField::Edge820),
      2531 => Some(GarminProductField::EdgeExplore820),
      2544 => Some(GarminProductField::Fenix5s),
      2547 => Some(GarminProductField::D2BravoTitanium),
      2593 => Some(GarminProductField::RunningDynamicsPod),
      2604 => Some(GarminProductField::Fenix5x),
      2606 => Some(GarminProductField::VivoFitJr),
      2691 => Some(GarminProductField::Fr935),
      2697 => Some(GarminProductField::Fenix5),
      10007 => Some(GarminProductField::Sdm4),
      10014 => Some(GarminProductField::EdgeRemote),
      20119 => Some(GarminProductField::TrainingCenter),
      65531 => Some(GarminProductField::ConnectiqSimulator),
      65532 => Some(GarminProductField::AndroidAntplusPlugin),
      65534 => Some(GarminProductField::Connect),
      _ => None,
    }
  }
}

/* name: sport_bits_3 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits3Field {
  Driving = 1,
  Golf = 2,
  HangGliding = 4,
  HorsebackRiding = 8,
  Hunting = 16,
  Fishing = 32,
  InlineSkating = 64,
  RockClimbing = 128,
}

impl From<SportBits3Field> for u8 {
  fn from(source: SportBits3Field) -> u8 {
    match source {
      SportBits3Field::Driving => 1,
      SportBits3Field::Golf => 2,
      SportBits3Field::HangGliding => 4,
      SportBits3Field::HorsebackRiding => 8,
      SportBits3Field::Hunting => 16,
      SportBits3Field::Fishing => 32,
      SportBits3Field::InlineSkating => 64,
      SportBits3Field::RockClimbing => 128,
    }
  }
}

impl IntoField<u8> for SportBits3Field {
  fn into_field(value: u8) -> Option<SportBits3Field> {
    match value {
      1 => Some(SportBits3Field::Driving),
      2 => Some(SportBits3Field::Golf),
      4 => Some(SportBits3Field::HangGliding),
      8 => Some(SportBits3Field::HorsebackRiding),
      16 => Some(SportBits3Field::Hunting),
      32 => Some(SportBits3Field::Fishing),
      64 => Some(SportBits3Field::InlineSkating),
      128 => Some(SportBits3Field::RockClimbing),
      _ => None,
    }
  }
}

/* name: exd_qualifiers type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum ExdQualifiersField {
  NoQualifier = 0,
  Instantaneous = 1,
  Average = 2,
  Lap = 3,
  Maximum = 4,
  MaximumAverage = 5,
  MaximumLap = 6,
  LastLap = 7,
  AverageLap = 8,
  ToDestination = 9,
  ToGo = 10,
  ToNext = 11,
  NextCoursePoint = 12,
  Total = 13,
  ThreeSecondAverage = 14,
  TenSecondAverage = 15,
  ThirtySecondAverage = 16,
  PercentMaximum = 17,
  PercentMaximumAverage = 18,
  LapPercentMaximum = 19,
  Elapsed = 20,
  Sunrise = 21,
  Sunset = 22,
  ComparedToVirtualPartner = 23,
  Maximum24h = 24,
  Minimum24h = 25,
  Minimum = 26,
  First = 27,
  Second = 28,
  Third = 29,
  Shifter = 30,
  LastSport = 31,
  Moving = 32,
  Stopped = 33,
  Zone9 = 242,
  Zone8 = 243,
  Zone7 = 244,
  Zone6 = 245,
  Zone5 = 246,
  Zone4 = 247,
  Zone3 = 248,
  Zone2 = 249,
  Zone1 = 250,
}

impl From<ExdQualifiersField> for u8 {
  fn from(source: ExdQualifiersField) -> u8 {
    match source {
      ExdQualifiersField::NoQualifier => 0,
      ExdQualifiersField::Instantaneous => 1,
      ExdQualifiersField::Average => 2,
      ExdQualifiersField::Lap => 3,
      ExdQualifiersField::Maximum => 4,
      ExdQualifiersField::MaximumAverage => 5,
      ExdQualifiersField::MaximumLap => 6,
      ExdQualifiersField::LastLap => 7,
      ExdQualifiersField::AverageLap => 8,
      ExdQualifiersField::ToDestination => 9,
      ExdQualifiersField::ToGo => 10,
      ExdQualifiersField::ToNext => 11,
      ExdQualifiersField::NextCoursePoint => 12,
      ExdQualifiersField::Total => 13,
      ExdQualifiersField::ThreeSecondAverage => 14,
      ExdQualifiersField::TenSecondAverage => 15,
      ExdQualifiersField::ThirtySecondAverage => 16,
      ExdQualifiersField::PercentMaximum => 17,
      ExdQualifiersField::PercentMaximumAverage => 18,
      ExdQualifiersField::LapPercentMaximum => 19,
      ExdQualifiersField::Elapsed => 20,
      ExdQualifiersField::Sunrise => 21,
      ExdQualifiersField::Sunset => 22,
      ExdQualifiersField::ComparedToVirtualPartner => 23,
      ExdQualifiersField::Maximum24h => 24,
      ExdQualifiersField::Minimum24h => 25,
      ExdQualifiersField::Minimum => 26,
      ExdQualifiersField::First => 27,
      ExdQualifiersField::Second => 28,
      ExdQualifiersField::Third => 29,
      ExdQualifiersField::Shifter => 30,
      ExdQualifiersField::LastSport => 31,
      ExdQualifiersField::Moving => 32,
      ExdQualifiersField::Stopped => 33,
      ExdQualifiersField::Zone9 => 242,
      ExdQualifiersField::Zone8 => 243,
      ExdQualifiersField::Zone7 => 244,
      ExdQualifiersField::Zone6 => 245,
      ExdQualifiersField::Zone5 => 246,
      ExdQualifiersField::Zone4 => 247,
      ExdQualifiersField::Zone3 => 248,
      ExdQualifiersField::Zone2 => 249,
      ExdQualifiersField::Zone1 => 250,
    }
  }
}

impl IntoField<u8> for ExdQualifiersField {
  fn into_field(value: u8) -> Option<ExdQualifiersField> {
    match value {
      0 => Some(ExdQualifiersField::NoQualifier),
      1 => Some(ExdQualifiersField::Instantaneous),
      2 => Some(ExdQualifiersField::Average),
      3 => Some(ExdQualifiersField::Lap),
      4 => Some(ExdQualifiersField::Maximum),
      5 => Some(ExdQualifiersField::MaximumAverage),
      6 => Some(ExdQualifiersField::MaximumLap),
      7 => Some(ExdQualifiersField::LastLap),
      8 => Some(ExdQualifiersField::AverageLap),
      9 => Some(ExdQualifiersField::ToDestination),
      10 => Some(ExdQualifiersField::ToGo),
      11 => Some(ExdQualifiersField::ToNext),
      12 => Some(ExdQualifiersField::NextCoursePoint),
      13 => Some(ExdQualifiersField::Total),
      14 => Some(ExdQualifiersField::ThreeSecondAverage),
      15 => Some(ExdQualifiersField::TenSecondAverage),
      16 => Some(ExdQualifiersField::ThirtySecondAverage),
      17 => Some(ExdQualifiersField::PercentMaximum),
      18 => Some(ExdQualifiersField::PercentMaximumAverage),
      19 => Some(ExdQualifiersField::LapPercentMaximum),
      20 => Some(ExdQualifiersField::Elapsed),
      21 => Some(ExdQualifiersField::Sunrise),
      22 => Some(ExdQualifiersField::Sunset),
      23 => Some(ExdQualifiersField::ComparedToVirtualPartner),
      24 => Some(ExdQualifiersField::Maximum24h),
      25 => Some(ExdQualifiersField::Minimum24h),
      26 => Some(ExdQualifiersField::Minimum),
      27 => Some(ExdQualifiersField::First),
      28 => Some(ExdQualifiersField::Second),
      29 => Some(ExdQualifiersField::Third),
      30 => Some(ExdQualifiersField::Shifter),
      31 => Some(ExdQualifiersField::LastSport),
      32 => Some(ExdQualifiersField::Moving),
      33 => Some(ExdQualifiersField::Stopped),
      242 => Some(ExdQualifiersField::Zone9),
      243 => Some(ExdQualifiersField::Zone8),
      244 => Some(ExdQualifiersField::Zone7),
      245 => Some(ExdQualifiersField::Zone6),
      246 => Some(ExdQualifiersField::Zone5),
      247 => Some(ExdQualifiersField::Zone4),
      248 => Some(ExdQualifiersField::Zone3),
      249 => Some(ExdQualifiersField::Zone2),
      250 => Some(ExdQualifiersField::Zone1),
      _ => None,
    }
  }
}

/* name: stroke_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum StrokeTypeField {
  NoEvent = 0,
  Other = 1,
  Serve = 2,
  Forehand = 3,
  Backhand = 4,
  Smash = 5,
}

impl From<StrokeTypeField> for u8 {
  fn from(source: StrokeTypeField) -> u8 {
    match source {
      StrokeTypeField::NoEvent => 0,
      StrokeTypeField::Other => 1,
      StrokeTypeField::Serve => 2,
      StrokeTypeField::Forehand => 3,
      StrokeTypeField::Backhand => 4,
      StrokeTypeField::Smash => 5,
    }
  }
}

impl IntoField<u8> for StrokeTypeField {
  fn into_field(value: u8) -> Option<StrokeTypeField> {
    match value {
      0 => Some(StrokeTypeField::NoEvent),
      1 => Some(StrokeTypeField::Other),
      2 => Some(StrokeTypeField::Serve),
      3 => Some(StrokeTypeField::Forehand),
      4 => Some(StrokeTypeField::Backhand),
      5 => Some(StrokeTypeField::Smash),
      _ => None,
    }
  }
}

/* name: length_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LengthTypeField {
  Idle = 0,
  Active = 1,
}

impl From<LengthTypeField> for u8 {
  fn from(source: LengthTypeField) -> u8 {
    match source {
      LengthTypeField::Idle => 0,
      LengthTypeField::Active => 1,
    }
  }
}

impl IntoField<u8> for LengthTypeField {
  fn into_field(value: u8) -> Option<LengthTypeField> {
    match value {
      0 => Some(LengthTypeField::Idle),
      1 => Some(LengthTypeField::Active),
      _ => None,
    }
  }
}

/* name: file type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum FileField {
  Device = 1,
  Settings = 2,
  Sport = 3,
  Activity = 4,
  Workout = 5,
  Course = 6,
  Schedules = 7,
  Weight = 9,
  Totals = 10,
  Goals = 11,
  BloodPressure = 14,
  MonitoringA = 15,
  ActivitySummary = 20,
  MonitoringDaily = 28,
  MonitoringB = 32,
  Segment = 34,
  SegmentList = 35,
  ExdConfiguration = 40,
  MfgRangeMin = 247,
  MfgRangeMax = 254,
}

impl From<FileField> for u8 {
  fn from(source: FileField) -> u8 {
    match source {
      FileField::Device => 1,
      FileField::Settings => 2,
      FileField::Sport => 3,
      FileField::Activity => 4,
      FileField::Workout => 5,
      FileField::Course => 6,
      FileField::Schedules => 7,
      FileField::Weight => 9,
      FileField::Totals => 10,
      FileField::Goals => 11,
      FileField::BloodPressure => 14,
      FileField::MonitoringA => 15,
      FileField::ActivitySummary => 20,
      FileField::MonitoringDaily => 28,
      FileField::MonitoringB => 32,
      FileField::Segment => 34,
      FileField::SegmentList => 35,
      FileField::ExdConfiguration => 40,
      FileField::MfgRangeMin => 247,
      FileField::MfgRangeMax => 254,
    }
  }
}

impl IntoField<u8> for FileField {
  fn into_field(value: u8) -> Option<FileField> {
    match value {
      1 => Some(FileField::Device),
      2 => Some(FileField::Settings),
      3 => Some(FileField::Sport),
      4 => Some(FileField::Activity),
      5 => Some(FileField::Workout),
      6 => Some(FileField::Course),
      7 => Some(FileField::Schedules),
      9 => Some(FileField::Weight),
      10 => Some(FileField::Totals),
      11 => Some(FileField::Goals),
      14 => Some(FileField::BloodPressure),
      15 => Some(FileField::MonitoringA),
      20 => Some(FileField::ActivitySummary),
      28 => Some(FileField::MonitoringDaily),
      32 => Some(FileField::MonitoringB),
      34 => Some(FileField::Segment),
      35 => Some(FileField::SegmentList),
      40 => Some(FileField::ExdConfiguration),
      247 => Some(FileField::MfgRangeMin),
      254 => Some(FileField::MfgRangeMax),
      _ => None,
    }
  }
}

/* name: timer_trigger type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum TimerTriggerField {
  Manual = 0,
  Auto = 1,
  FitnessEquipment = 2,
}

impl From<TimerTriggerField> for u8 {
  fn from(source: TimerTriggerField) -> u8 {
    match source {
      TimerTriggerField::Manual => 0,
      TimerTriggerField::Auto => 1,
      TimerTriggerField::FitnessEquipment => 2,
    }
  }
}

impl IntoField<u8> for TimerTriggerField {
  fn into_field(value: u8) -> Option<TimerTriggerField> {
    match value {
      0 => Some(TimerTriggerField::Manual),
      1 => Some(TimerTriggerField::Auto),
      2 => Some(TimerTriggerField::FitnessEquipment),
      _ => None,
    }
  }
}

/* name: workout_equipment type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutEquipmentField {
  None = 0,
  SwimFins = 1,
  SwimKickboard = 2,
  SwimPaddles = 3,
  SwimPullBuoy = 4,
  SwimSnorkel = 5,
}

impl From<WorkoutEquipmentField> for u8 {
  fn from(source: WorkoutEquipmentField) -> u8 {
    match source {
      WorkoutEquipmentField::None => 0,
      WorkoutEquipmentField::SwimFins => 1,
      WorkoutEquipmentField::SwimKickboard => 2,
      WorkoutEquipmentField::SwimPaddles => 3,
      WorkoutEquipmentField::SwimPullBuoy => 4,
      WorkoutEquipmentField::SwimSnorkel => 5,
    }
  }
}

impl IntoField<u8> for WorkoutEquipmentField {
  fn into_field(value: u8) -> Option<WorkoutEquipmentField> {
    match value {
      0 => Some(WorkoutEquipmentField::None),
      1 => Some(WorkoutEquipmentField::SwimFins),
      2 => Some(WorkoutEquipmentField::SwimKickboard),
      3 => Some(WorkoutEquipmentField::SwimPaddles),
      4 => Some(WorkoutEquipmentField::SwimPullBuoy),
      5 => Some(WorkoutEquipmentField::SwimSnorkel),
      _ => None,
    }
  }
}

/* name: sport_bits_2 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits2Field {
  Mountaineering = 1,
  Hiking = 2,
  Multisport = 4,
  Paddling = 8,
  Flying = 16,
  EBiking = 32,
  Motorcycling = 64,
  Boating = 128,
}

impl From<SportBits2Field> for u8 {
  fn from(source: SportBits2Field) -> u8 {
    match source {
      SportBits2Field::Mountaineering => 1,
      SportBits2Field::Hiking => 2,
      SportBits2Field::Multisport => 4,
      SportBits2Field::Paddling => 8,
      SportBits2Field::Flying => 16,
      SportBits2Field::EBiking => 32,
      SportBits2Field::Motorcycling => 64,
      SportBits2Field::Boating => 128,
    }
  }
}

impl IntoField<u8> for SportBits2Field {
  fn into_field(value: u8) -> Option<SportBits2Field> {
    match value {
      1 => Some(SportBits2Field::Mountaineering),
      2 => Some(SportBits2Field::Hiking),
      4 => Some(SportBits2Field::Multisport),
      8 => Some(SportBits2Field::Paddling),
      16 => Some(SportBits2Field::Flying),
      32 => Some(SportBits2Field::EBiking),
      64 => Some(SportBits2Field::Motorcycling),
      128 => Some(SportBits2Field::Boating),
      _ => None,
    }
  }
}

/* name: workout_power type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutPowerField {
  WattsOffset = 1000,
}

impl From<WorkoutPowerField> for u32 {
  fn from(source: WorkoutPowerField) -> u32 {
    match source {
      WorkoutPowerField::WattsOffset => 1000,
    }
  }
}

impl IntoField<u32> for WorkoutPowerField {
  fn into_field(value: u32) -> Option<WorkoutPowerField> {
    match value {
      1000 => Some(WorkoutPowerField::WattsOffset),
      _ => None,
    }
  }
}

/* name: wkt_step_duration type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WktStepDurationField {
  Time = 0,
  Distance = 1,
  HrLessThan = 2,
  HrGreaterThan = 3,
  Calories = 4,
  Open = 5,
  RepeatUntilStepsCmplt = 6,
  RepeatUntilTime = 7,
  RepeatUntilDistance = 8,
  RepeatUntilCalories = 9,
  RepeatUntilHrLessThan = 10,
  RepeatUntilHrGreaterThan = 11,
  RepeatUntilPowerLessThan = 12,
  RepeatUntilPowerGreaterThan = 13,
  PowerLessThan = 14,
  PowerGreaterThan = 15,
  TrainingPeaksTss = 16,
  RepeatUntilPowerLastLapLessThan = 17,
  RepeatUntilMaxPowerLastLapLessThan = 18,
  Power3sLessThan = 19,
  Power10sLessThan = 20,
  Power30sLessThan = 21,
  Power3sGreaterThan = 22,
  Power10sGreaterThan = 23,
  Power30sGreaterThan = 24,
  PowerLapLessThan = 25,
  PowerLapGreaterThan = 26,
  RepeatUntilTrainingPeaksTss = 27,
  RepetitionTime = 28,
}

impl From<WktStepDurationField> for u8 {
  fn from(source: WktStepDurationField) -> u8 {
    match source {
      WktStepDurationField::Time => 0,
      WktStepDurationField::Distance => 1,
      WktStepDurationField::HrLessThan => 2,
      WktStepDurationField::HrGreaterThan => 3,
      WktStepDurationField::Calories => 4,
      WktStepDurationField::Open => 5,
      WktStepDurationField::RepeatUntilStepsCmplt => 6,
      WktStepDurationField::RepeatUntilTime => 7,
      WktStepDurationField::RepeatUntilDistance => 8,
      WktStepDurationField::RepeatUntilCalories => 9,
      WktStepDurationField::RepeatUntilHrLessThan => 10,
      WktStepDurationField::RepeatUntilHrGreaterThan => 11,
      WktStepDurationField::RepeatUntilPowerLessThan => 12,
      WktStepDurationField::RepeatUntilPowerGreaterThan => 13,
      WktStepDurationField::PowerLessThan => 14,
      WktStepDurationField::PowerGreaterThan => 15,
      WktStepDurationField::TrainingPeaksTss => 16,
      WktStepDurationField::RepeatUntilPowerLastLapLessThan => 17,
      WktStepDurationField::RepeatUntilMaxPowerLastLapLessThan => 18,
      WktStepDurationField::Power3sLessThan => 19,
      WktStepDurationField::Power10sLessThan => 20,
      WktStepDurationField::Power30sLessThan => 21,
      WktStepDurationField::Power3sGreaterThan => 22,
      WktStepDurationField::Power10sGreaterThan => 23,
      WktStepDurationField::Power30sGreaterThan => 24,
      WktStepDurationField::PowerLapLessThan => 25,
      WktStepDurationField::PowerLapGreaterThan => 26,
      WktStepDurationField::RepeatUntilTrainingPeaksTss => 27,
      WktStepDurationField::RepetitionTime => 28,
    }
  }
}

impl IntoField<u8> for WktStepDurationField {
  fn into_field(value: u8) -> Option<WktStepDurationField> {
    match value {
      0 => Some(WktStepDurationField::Time),
      1 => Some(WktStepDurationField::Distance),
      2 => Some(WktStepDurationField::HrLessThan),
      3 => Some(WktStepDurationField::HrGreaterThan),
      4 => Some(WktStepDurationField::Calories),
      5 => Some(WktStepDurationField::Open),
      6 => Some(WktStepDurationField::RepeatUntilStepsCmplt),
      7 => Some(WktStepDurationField::RepeatUntilTime),
      8 => Some(WktStepDurationField::RepeatUntilDistance),
      9 => Some(WktStepDurationField::RepeatUntilCalories),
      10 => Some(WktStepDurationField::RepeatUntilHrLessThan),
      11 => Some(WktStepDurationField::RepeatUntilHrGreaterThan),
      12 => Some(WktStepDurationField::RepeatUntilPowerLessThan),
      13 => Some(WktStepDurationField::RepeatUntilPowerGreaterThan),
      14 => Some(WktStepDurationField::PowerLessThan),
      15 => Some(WktStepDurationField::PowerGreaterThan),
      16 => Some(WktStepDurationField::TrainingPeaksTss),
      17 => Some(WktStepDurationField::RepeatUntilPowerLastLapLessThan),
      18 => Some(WktStepDurationField::RepeatUntilMaxPowerLastLapLessThan),
      19 => Some(WktStepDurationField::Power3sLessThan),
      20 => Some(WktStepDurationField::Power10sLessThan),
      21 => Some(WktStepDurationField::Power30sLessThan),
      22 => Some(WktStepDurationField::Power3sGreaterThan),
      23 => Some(WktStepDurationField::Power10sGreaterThan),
      24 => Some(WktStepDurationField::Power30sGreaterThan),
      25 => Some(WktStepDurationField::PowerLapLessThan),
      26 => Some(WktStepDurationField::PowerLapGreaterThan),
      27 => Some(WktStepDurationField::RepeatUntilTrainingPeaksTss),
      28 => Some(WktStepDurationField::RepetitionTime),
      _ => None,
    }
  }
}

/* name: weather_report type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WeatherReportField {
  Current = 0,
  HourlyForecast = 1,
  DailyForecast = 2,
}

impl From<WeatherReportField> for u8 {
  fn from(source: WeatherReportField) -> u8 {
    match source {
      WeatherReportField::Current => 0,
      WeatherReportField::HourlyForecast => 1,
      WeatherReportField::DailyForecast => 2,
    }
  }
}

impl IntoField<u8> for WeatherReportField {
  fn into_field(value: u8) -> Option<WeatherReportField> {
    match value {
      0 => Some(WeatherReportField::Current),
      1 => Some(WeatherReportField::HourlyForecast),
      2 => Some(WeatherReportField::DailyForecast),
      _ => None,
    }
  }
}

/* name: segment_leaderboard_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SegmentLeaderboardTypeField {
  Overall = 0,
  PersonalBest = 1,
  Connections = 2,
  Group = 3,
  Challenger = 4,
  Kom = 5,
  Qom = 6,
  Pr = 7,
  Goal = 8,
  Rival = 9,
  ClubLeader = 10,
}

impl From<SegmentLeaderboardTypeField> for u8 {
  fn from(source: SegmentLeaderboardTypeField) -> u8 {
    match source {
      SegmentLeaderboardTypeField::Overall => 0,
      SegmentLeaderboardTypeField::PersonalBest => 1,
      SegmentLeaderboardTypeField::Connections => 2,
      SegmentLeaderboardTypeField::Group => 3,
      SegmentLeaderboardTypeField::Challenger => 4,
      SegmentLeaderboardTypeField::Kom => 5,
      SegmentLeaderboardTypeField::Qom => 6,
      SegmentLeaderboardTypeField::Pr => 7,
      SegmentLeaderboardTypeField::Goal => 8,
      SegmentLeaderboardTypeField::Rival => 9,
      SegmentLeaderboardTypeField::ClubLeader => 10,
    }
  }
}

impl IntoField<u8> for SegmentLeaderboardTypeField {
  fn into_field(value: u8) -> Option<SegmentLeaderboardTypeField> {
    match value {
      0 => Some(SegmentLeaderboardTypeField::Overall),
      1 => Some(SegmentLeaderboardTypeField::PersonalBest),
      2 => Some(SegmentLeaderboardTypeField::Connections),
      3 => Some(SegmentLeaderboardTypeField::Group),
      4 => Some(SegmentLeaderboardTypeField::Challenger),
      5 => Some(SegmentLeaderboardTypeField::Kom),
      6 => Some(SegmentLeaderboardTypeField::Qom),
      7 => Some(SegmentLeaderboardTypeField::Pr),
      8 => Some(SegmentLeaderboardTypeField::Goal),
      9 => Some(SegmentLeaderboardTypeField::Rival),
      10 => Some(SegmentLeaderboardTypeField::ClubLeader),
      _ => None,
    }
  }
}

/* name: language_bits_2 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LanguageBits2Field {
  Slovenian = 1,
  Swedish = 2,
  Russian = 4,
  Turkish = 8,
  Latvian = 16,
  Ukrainian = 32,
  Arabic = 64,
  Farsi = 128,
}

impl From<LanguageBits2Field> for u8 {
  fn from(source: LanguageBits2Field) -> u8 {
    match source {
      LanguageBits2Field::Slovenian => 1,
      LanguageBits2Field::Swedish => 2,
      LanguageBits2Field::Russian => 4,
      LanguageBits2Field::Turkish => 8,
      LanguageBits2Field::Latvian => 16,
      LanguageBits2Field::Ukrainian => 32,
      LanguageBits2Field::Arabic => 64,
      LanguageBits2Field::Farsi => 128,
    }
  }
}

impl IntoField<u8> for LanguageBits2Field {
  fn into_field(value: u8) -> Option<LanguageBits2Field> {
    match value {
      1 => Some(LanguageBits2Field::Slovenian),
      2 => Some(LanguageBits2Field::Swedish),
      4 => Some(LanguageBits2Field::Russian),
      8 => Some(LanguageBits2Field::Turkish),
      16 => Some(LanguageBits2Field::Latvian),
      32 => Some(LanguageBits2Field::Ukrainian),
      64 => Some(LanguageBits2Field::Arabic),
      128 => Some(LanguageBits2Field::Farsi),
      _ => None,
    }
  }
}

/* name: display_heart type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DisplayHeartField {
  Bpm = 0,
  Max = 1,
  Reserve = 2,
}

impl From<DisplayHeartField> for u8 {
  fn from(source: DisplayHeartField) -> u8 {
    match source {
      DisplayHeartField::Bpm => 0,
      DisplayHeartField::Max => 1,
      DisplayHeartField::Reserve => 2,
    }
  }
}

impl IntoField<u8> for DisplayHeartField {
  fn into_field(value: u8) -> Option<DisplayHeartField> {
    match value {
      0 => Some(DisplayHeartField::Bpm),
      1 => Some(DisplayHeartField::Max),
      2 => Some(DisplayHeartField::Reserve),
      _ => None,
    }
  }
}

/* name: date_mode type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DateModeField {
  DayMonth = 0,
  MonthDay = 1,
}

impl From<DateModeField> for u8 {
  fn from(source: DateModeField) -> u8 {
    match source {
      DateModeField::DayMonth => 0,
      DateModeField::MonthDay => 1,
    }
  }
}

impl IntoField<u8> for DateModeField {
  fn into_field(value: u8) -> Option<DateModeField> {
    match value {
      0 => Some(DateModeField::DayMonth),
      1 => Some(DateModeField::MonthDay),
      _ => None,
    }
  }
}

/* name: display_power type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum DisplayPowerField {
  Watts = 0,
  PercentFtp = 1,
}

impl From<DisplayPowerField> for u8 {
  fn from(source: DisplayPowerField) -> u8 {
    match source {
      DisplayPowerField::Watts => 0,
      DisplayPowerField::PercentFtp => 1,
    }
  }
}

impl IntoField<u8> for DisplayPowerField {
  fn into_field(value: u8) -> Option<DisplayPowerField> {
    match value {
      0 => Some(DisplayPowerField::Watts),
      1 => Some(DisplayPowerField::PercentFtp),
      _ => None,
    }
  }
}

/* name: analog_watchface_layout type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AnalogWatchfaceLayoutField {
  Minimal = 0,
  Traditional = 1,
  Modern = 2,
}

impl From<AnalogWatchfaceLayoutField> for u8 {
  fn from(source: AnalogWatchfaceLayoutField) -> u8 {
    match source {
      AnalogWatchfaceLayoutField::Minimal => 0,
      AnalogWatchfaceLayoutField::Traditional => 1,
      AnalogWatchfaceLayoutField::Modern => 2,
    }
  }
}

impl IntoField<u8> for AnalogWatchfaceLayoutField {
  fn into_field(value: u8) -> Option<AnalogWatchfaceLayoutField> {
    match value {
      0 => Some(AnalogWatchfaceLayoutField::Minimal),
      1 => Some(AnalogWatchfaceLayoutField::Traditional),
      2 => Some(AnalogWatchfaceLayoutField::Modern),
      _ => None,
    }
  }
}

/* name: backlight_mode type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum BacklightModeField {
  Off = 0,
  Manual = 1,
  KeyAndMessages = 2,
  AutoBrightness = 3,
  SmartNotifications = 4,
  KeyAndMessagesNight = 5,
  KeyAndMessagesAndSmartNotifications = 6,
}

impl From<BacklightModeField> for u8 {
  fn from(source: BacklightModeField) -> u8 {
    match source {
      BacklightModeField::Off => 0,
      BacklightModeField::Manual => 1,
      BacklightModeField::KeyAndMessages => 2,
      BacklightModeField::AutoBrightness => 3,
      BacklightModeField::SmartNotifications => 4,
      BacklightModeField::KeyAndMessagesNight => 5,
      BacklightModeField::KeyAndMessagesAndSmartNotifications => 6,
    }
  }
}

impl IntoField<u8> for BacklightModeField {
  fn into_field(value: u8) -> Option<BacklightModeField> {
    match value {
      0 => Some(BacklightModeField::Off),
      1 => Some(BacklightModeField::Manual),
      2 => Some(BacklightModeField::KeyAndMessages),
      3 => Some(BacklightModeField::AutoBrightness),
      4 => Some(BacklightModeField::SmartNotifications),
      5 => Some(BacklightModeField::KeyAndMessagesNight),
      6 => Some(BacklightModeField::KeyAndMessagesAndSmartNotifications),
      _ => None,
    }
  }
}

/* name: sport_bits_4 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits4Field {
  Sailing = 1,
  IceSkating = 2,
  SkyDiving = 4,
  Snowshoeing = 8,
  Snowmobiling = 16,
  StandUpPaddleboarding = 32,
  Surfing = 64,
  Wakeboarding = 128,
}

impl From<SportBits4Field> for u8 {
  fn from(source: SportBits4Field) -> u8 {
    match source {
      SportBits4Field::Sailing => 1,
      SportBits4Field::IceSkating => 2,
      SportBits4Field::SkyDiving => 4,
      SportBits4Field::Snowshoeing => 8,
      SportBits4Field::Snowmobiling => 16,
      SportBits4Field::StandUpPaddleboarding => 32,
      SportBits4Field::Surfing => 64,
      SportBits4Field::Wakeboarding => 128,
    }
  }
}

impl IntoField<u8> for SportBits4Field {
  fn into_field(value: u8) -> Option<SportBits4Field> {
    match value {
      1 => Some(SportBits4Field::Sailing),
      2 => Some(SportBits4Field::IceSkating),
      4 => Some(SportBits4Field::SkyDiving),
      8 => Some(SportBits4Field::Snowshoeing),
      16 => Some(SportBits4Field::Snowmobiling),
      32 => Some(SportBits4Field::StandUpPaddleboarding),
      64 => Some(SportBits4Field::Surfing),
      128 => Some(SportBits4Field::Wakeboarding),
      _ => None,
    }
  }
}

/* name: session_trigger type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SessionTriggerField {
  ActivityEnd = 0,
  Manual = 1,
  AutoMultiSport = 2,
  FitnessEquipment = 3,
}

impl From<SessionTriggerField> for u8 {
  fn from(source: SessionTriggerField) -> u8 {
    match source {
      SessionTriggerField::ActivityEnd => 0,
      SessionTriggerField::Manual => 1,
      SessionTriggerField::AutoMultiSport => 2,
      SessionTriggerField::FitnessEquipment => 3,
    }
  }
}

impl IntoField<u8> for SessionTriggerField {
  fn into_field(value: u8) -> Option<SessionTriggerField> {
    match value {
      0 => Some(SessionTriggerField::ActivityEnd),
      1 => Some(SessionTriggerField::Manual),
      2 => Some(SessionTriggerField::AutoMultiSport),
      3 => Some(SessionTriggerField::FitnessEquipment),
      _ => None,
    }
  }
}

/* name: sport_bits_0 type: uint8z */
#[allow(dead_code)]
#[derive(Debug)]
pub enum SportBits0Field {
  Generic = 1,
  Running = 2,
  Cycling = 4,
  Transition = 8,
  FitnessEquipment = 16,
  Swimming = 32,
  Basketball = 64,
  Soccer = 128,
}

impl From<SportBits0Field> for u8 {
  fn from(source: SportBits0Field) -> u8 {
    match source {
      SportBits0Field::Generic => 1,
      SportBits0Field::Running => 2,
      SportBits0Field::Cycling => 4,
      SportBits0Field::Transition => 8,
      SportBits0Field::FitnessEquipment => 16,
      SportBits0Field::Swimming => 32,
      SportBits0Field::Basketball => 64,
      SportBits0Field::Soccer => 128,
    }
  }
}

impl IntoField<u8> for SportBits0Field {
  fn into_field(value: u8) -> Option<SportBits0Field> {
    match value {
      1 => Some(SportBits0Field::Generic),
      2 => Some(SportBits0Field::Running),
      4 => Some(SportBits0Field::Cycling),
      8 => Some(SportBits0Field::Transition),
      16 => Some(SportBits0Field::FitnessEquipment),
      32 => Some(SportBits0Field::Swimming),
      64 => Some(SportBits0Field::Basketball),
      128 => Some(SportBits0Field::Soccer),
      _ => None,
    }
  }
}

/* name: event_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum EventTypeField {
  Start = 0,
  Stop = 1,
  ConsecutiveDepreciated = 2,
  Marker = 3,
  StopAll = 4,
  BeginDepreciated = 5,
  EndDepreciated = 6,
  EndAllDepreciated = 7,
  StopDisable = 8,
  StopDisableAll = 9,
}

impl From<EventTypeField> for u8 {
  fn from(source: EventTypeField) -> u8 {
    match source {
      EventTypeField::Start => 0,
      EventTypeField::Stop => 1,
      EventTypeField::ConsecutiveDepreciated => 2,
      EventTypeField::Marker => 3,
      EventTypeField::StopAll => 4,
      EventTypeField::BeginDepreciated => 5,
      EventTypeField::EndDepreciated => 6,
      EventTypeField::EndAllDepreciated => 7,
      EventTypeField::StopDisable => 8,
      EventTypeField::StopDisableAll => 9,
    }
  }
}

impl IntoField<u8> for EventTypeField {
  fn into_field(value: u8) -> Option<EventTypeField> {
    match value {
      0 => Some(EventTypeField::Start),
      1 => Some(EventTypeField::Stop),
      2 => Some(EventTypeField::ConsecutiveDepreciated),
      3 => Some(EventTypeField::Marker),
      4 => Some(EventTypeField::StopAll),
      5 => Some(EventTypeField::BeginDepreciated),
      6 => Some(EventTypeField::EndDepreciated),
      7 => Some(EventTypeField::EndAllDepreciated),
      8 => Some(EventTypeField::StopDisable),
      9 => Some(EventTypeField::StopDisableAll),
      _ => None,
    }
  }
}

/* name: workout_hr type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WorkoutHrField {
  BpmOffset = 100,
}

impl From<WorkoutHrField> for u32 {
  fn from(source: WorkoutHrField) -> u32 {
    match source {
      WorkoutHrField::BpmOffset => 100,
    }
  }
}

impl IntoField<u32> for WorkoutHrField {
  fn into_field(value: u32) -> Option<WorkoutHrField> {
    match value {
      100 => Some(WorkoutHrField::BpmOffset),
      _ => None,
    }
  }
}

/* name: left_right_balance type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LeftRightBalanceField {
  Mask = 127,
  Right = 128,
}

impl From<LeftRightBalanceField> for u8 {
  fn from(source: LeftRightBalanceField) -> u8 {
    match source {
      LeftRightBalanceField::Mask => 127,
      LeftRightBalanceField::Right => 128,
    }
  }
}

impl IntoField<u8> for LeftRightBalanceField {
  fn into_field(value: u8) -> Option<LeftRightBalanceField> {
    match value {
      127 => Some(LeftRightBalanceField::Mask),
      128 => Some(LeftRightBalanceField::Right),
      _ => None,
    }
  }
}

/* name: weather_severity type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum WeatherSeverityField {
  Unknown = 0,
  Warning = 1,
  Watch = 2,
  Advisory = 3,
  Statement = 4,
}

impl From<WeatherSeverityField> for u8 {
  fn from(source: WeatherSeverityField) -> u8 {
    match source {
      WeatherSeverityField::Unknown => 0,
      WeatherSeverityField::Warning => 1,
      WeatherSeverityField::Watch => 2,
      WeatherSeverityField::Advisory => 3,
      WeatherSeverityField::Statement => 4,
    }
  }
}

impl IntoField<u8> for WeatherSeverityField {
  fn into_field(value: u8) -> Option<WeatherSeverityField> {
    match value {
      0 => Some(WeatherSeverityField::Unknown),
      1 => Some(WeatherSeverityField::Warning),
      2 => Some(WeatherSeverityField::Watch),
      3 => Some(WeatherSeverityField::Advisory),
      4 => Some(WeatherSeverityField::Statement),
      _ => None,
    }
  }
}

/* name: goal_source type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum GoalSourceField {
  Auto = 0,
  Community = 1,
  User = 2,
}

impl From<GoalSourceField> for u8 {
  fn from(source: GoalSourceField) -> u8 {
    match source {
      GoalSourceField::Auto => 0,
      GoalSourceField::Community => 1,
      GoalSourceField::User => 2,
    }
  }
}

impl IntoField<u8> for GoalSourceField {
  fn into_field(value: u8) -> Option<GoalSourceField> {
    match value {
      0 => Some(GoalSourceField::Auto),
      1 => Some(GoalSourceField::Community),
      2 => Some(GoalSourceField::User),
      _ => None,
    }
  }
}

/* name: local_date_time type: uint32 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum LocalDateTimeField {
  Min = 268435456,
}

impl From<LocalDateTimeField> for u32 {
  fn from(source: LocalDateTimeField) -> u32 {
    match source {
      LocalDateTimeField::Min => 268435456,
    }
  }
}

impl IntoField<u32> for LocalDateTimeField {
  fn into_field(value: u32) -> Option<LocalDateTimeField> {
    match value {
      268435456 => Some(LocalDateTimeField::Min),
      _ => None,
    }
  }
}

/* name: hr_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum HrTypeField {
  Normal = 0,
  Irregular = 1,
}

impl From<HrTypeField> for u8 {
  fn from(source: HrTypeField) -> u8 {
    match source {
      HrTypeField::Normal => 0,
      HrTypeField::Irregular => 1,
    }
  }
}

impl IntoField<u8> for HrTypeField {
  fn into_field(value: u8) -> Option<HrTypeField> {
    match value {
      0 => Some(HrTypeField::Normal),
      1 => Some(HrTypeField::Irregular),
      _ => None,
    }
  }
}

/* name: ant_network type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum AntNetworkField {
  Public = 0,
  Antplus = 1,
  Antfs = 2,
  Private = 3,
}

impl From<AntNetworkField> for u8 {
  fn from(source: AntNetworkField) -> u8 {
    match source {
      AntNetworkField::Public => 0,
      AntNetworkField::Antplus => 1,
      AntNetworkField::Antfs => 2,
      AntNetworkField::Private => 3,
    }
  }
}

impl IntoField<u8> for AntNetworkField {
  fn into_field(value: u8) -> Option<AntNetworkField> {
    match value {
      0 => Some(AntNetworkField::Public),
      1 => Some(AntNetworkField::Antplus),
      2 => Some(AntNetworkField::Antfs),
      3 => Some(AntNetworkField::Private),
      _ => None,
    }
  }
}

/* name: message_index type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum MessageIndexField {
  Mask = 4095,
  Reserved = 28672,
  Selected = 32768,
}

impl From<MessageIndexField> for u16 {
  fn from(source: MessageIndexField) -> u16 {
    match source {
      MessageIndexField::Mask => 4095,
      MessageIndexField::Reserved => 28672,
      MessageIndexField::Selected => 32768,
    }
  }
}

impl IntoField<u16> for MessageIndexField {
  fn into_field(value: u16) -> Option<MessageIndexField> {
    match value {
      4095 => Some(MessageIndexField::Mask),
      28672 => Some(MessageIndexField::Reserved),
      32768 => Some(MessageIndexField::Selected),
      _ => None,
    }
  }
}

/* name: rider_position_type type: enum */
#[allow(dead_code)]
#[derive(Debug)]
pub enum RiderPositionTypeField {
  Seated = 0,
  Standing = 1,
  TransitionToSeated = 2,
  TransitionToStanding = 3,
}

impl From<RiderPositionTypeField> for u8 {
  fn from(source: RiderPositionTypeField) -> u8 {
    match source {
      RiderPositionTypeField::Seated => 0,
      RiderPositionTypeField::Standing => 1,
      RiderPositionTypeField::TransitionToSeated => 2,
      RiderPositionTypeField::TransitionToStanding => 3,
    }
  }
}

impl IntoField<u8> for RiderPositionTypeField {
  fn into_field(value: u8) -> Option<RiderPositionTypeField> {
    match value {
      0 => Some(RiderPositionTypeField::Seated),
      1 => Some(RiderPositionTypeField::Standing),
      2 => Some(RiderPositionTypeField::TransitionToSeated),
      3 => Some(RiderPositionTypeField::TransitionToStanding),
      _ => None,
    }
  }
}

/* name: fit_base_type type: uint8 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum FitBaseTypeField {
  Enum = 0,
  Sint8 = 1,
  Uint8 = 2,
  String = 7,
  Uint8z = 10,
  Byte = 13,
  Sint16 = 131,
  Uint16 = 132,
  Sint32 = 133,
  Uint32 = 134,
  Float32 = 136,
  Float64 = 137,
  Uint16z = 139,
  Uint32z = 140,
  Sint64 = 142,
  Uint64 = 143,
  Uint64z = 144,
}

impl From<FitBaseTypeField> for u8 {
  fn from(source: FitBaseTypeField) -> u8 {
    match source {
      FitBaseTypeField::Enum => 0,
      FitBaseTypeField::Sint8 => 1,
      FitBaseTypeField::Uint8 => 2,
      FitBaseTypeField::String => 7,
      FitBaseTypeField::Uint8z => 10,
      FitBaseTypeField::Byte => 13,
      FitBaseTypeField::Sint16 => 131,
      FitBaseTypeField::Uint16 => 132,
      FitBaseTypeField::Sint32 => 133,
      FitBaseTypeField::Uint32 => 134,
      FitBaseTypeField::Float32 => 136,
      FitBaseTypeField::Float64 => 137,
      FitBaseTypeField::Uint16z => 139,
      FitBaseTypeField::Uint32z => 140,
      FitBaseTypeField::Sint64 => 142,
      FitBaseTypeField::Uint64 => 143,
      FitBaseTypeField::Uint64z => 144,
    }
  }
}

impl IntoField<u8> for FitBaseTypeField {
  fn into_field(value: u8) -> Option<FitBaseTypeField> {
    match value {
      0 => Some(FitBaseTypeField::Enum),
      1 => Some(FitBaseTypeField::Sint8),
      2 => Some(FitBaseTypeField::Uint8),
      7 => Some(FitBaseTypeField::String),
      10 => Some(FitBaseTypeField::Uint8z),
      13 => Some(FitBaseTypeField::Byte),
      131 => Some(FitBaseTypeField::Sint16),
      132 => Some(FitBaseTypeField::Uint16),
      133 => Some(FitBaseTypeField::Sint32),
      134 => Some(FitBaseTypeField::Uint32),
      136 => Some(FitBaseTypeField::Float32),
      137 => Some(FitBaseTypeField::Float64),
      139 => Some(FitBaseTypeField::Uint16z),
      140 => Some(FitBaseTypeField::Uint32z),
      142 => Some(FitBaseTypeField::Sint64),
      143 => Some(FitBaseTypeField::Uint64),
      144 => Some(FitBaseTypeField::Uint64z),
      _ => None,
    }
  }
}

/* name: fit_base_unit type: uint16 */
#[allow(dead_code)]
#[derive(Debug)]
pub enum FitBaseUnitField {
  Other = 0,
  Kilogram = 1,
  Pound = 2,
}

impl From<FitBaseUnitField> for u16 {
  fn from(source: FitBaseUnitField) -> u16 {
    match source {
      FitBaseUnitField::Other => 0,
      FitBaseUnitField::Kilogram => 1,
      FitBaseUnitField::Pound => 2,
    }
  }
}

impl IntoField<u16> for FitBaseUnitField {
  fn into_field(value: u16) -> Option<FitBaseUnitField> {
    match value {
      0 => Some(FitBaseUnitField::Other),
      1 => Some(FitBaseUnitField::Kilogram),
      2 => Some(FitBaseUnitField::Pound),
      _ => None,
    }
  }
}
pub fn get_message_name(id: u16) -> Option<&'static str> {
  match id {
    0 => Some("file_id"),
    1 => Some("capabilities"),
    2 => Some("device_settings"),
    3 => Some("user_profile"),
    4 => Some("hrm_profile"),
    5 => Some("sdm_profile"),
    6 => Some("bike_profile"),
    7 => Some("zones_target"),
    8 => Some("hr_zone"),
    9 => Some("power_zone"),
    10 => Some("met_zone"),
    12 => Some("sport"),
    15 => Some("goal"),
    18 => Some("session"),
    19 => Some("lap"),
    20 => Some("record"),
    21 => Some("event"),
    23 => Some("device_info"),
    26 => Some("workout"),
    27 => Some("workout_step"),
    28 => Some("schedule"),
    30 => Some("weight_scale"),
    31 => Some("course"),
    32 => Some("course_point"),
    33 => Some("totals"),
    34 => Some("activity"),
    35 => Some("software"),
    37 => Some("file_capabilities"),
    38 => Some("mesg_capabilities"),
    39 => Some("field_capabilities"),
    49 => Some("file_creator"),
    51 => Some("blood_pressure"),
    53 => Some("speed_zone"),
    55 => Some("monitoring"),
    72 => Some("training_file"),
    78 => Some("hrv"),
    80 => Some("ant_rx"),
    81 => Some("ant_tx"),
    82 => Some("ant_channel_id"),
    101 => Some("length"),
    103 => Some("monitoring_info"),
    105 => Some("pad"),
    106 => Some("slave_device"),
    127 => Some("connectivity"),
    128 => Some("weather_conditions"),
    129 => Some("weather_alert"),
    131 => Some("cadence_zone"),
    132 => Some("hr"),
    142 => Some("segment_lap"),
    145 => Some("memo_glob"),
    148 => Some("segment_id"),
    149 => Some("segment_leaderboard_entry"),
    150 => Some("segment_point"),
    151 => Some("segment_file"),
    158 => Some("workout_session"),
    159 => Some("watchface_settings"),
    160 => Some("gps_metadata"),
    161 => Some("camera_event"),
    162 => Some("timestamp_correlation"),
    164 => Some("gyroscope_data"),
    165 => Some("accelerometer_data"),
    167 => Some("three_d_sensor_calibration"),
    169 => Some("video_frame"),
    174 => Some("obdii_data"),
    177 => Some("nmea_sentence"),
    178 => Some("aviation_attitude"),
    184 => Some("video"),
    185 => Some("video_title"),
    186 => Some("video_description"),
    187 => Some("video_clip"),
    188 => Some("ohr_settings"),
    200 => Some("exd_screen_configuration"),
    201 => Some("exd_data_field_configuration"),
    202 => Some("exd_data_concept_configuration"),
    206 => Some("field_description"),
    207 => Some("developer_data_id"),
    208 => Some("magnetometer_data"),
    209 => Some("barometer_data"),
    210 => Some("one_d_sensor_calibration"),
    227 => Some("stress_level"),
    65280 => Some("mfg_range_min"),
    65534 => Some("mfg_range_max"),
    _ => None,
  }
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct FileIdMessage {
  /* id == 0 */
  _type: u8, /* 00 Some(FitType { name: "file", value_type: "enum", values: {"activity": 4, "segment_list": 35, "sport": 3, "workout": 5, "monitoring_a": 15, "exd_configuration": 40, "monitoring_daily": 28, "settings": 2, "device": 1, "course": 6, "totals": 10, "goals": 11, "weight": 9, "blood_pressure": 14, "schedules": 7, "mfg_range_max": 254, "segment": 34, "monitoring_b": 32, "activity_summary": 20, "mfg_range_min": 247} }) */
  manufacturer: u16, /* 01 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
  product: u16, /* 02 None */
  serial_number: u32, /* 03 None */
  time_created: u32, /* 04 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  number: u16, /* 05 None */
  product_name: String, /* 08 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FileCreatorMessage {
  /* id == 49 */
  software_version: u16, /* 00 None */
  hardware_version: u8, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TimestampCorrelationMessage {
  /* id == 162 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  fractional_timestamp: u16, /* 00 None */
  system_timestamp: u32, /* 01 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  fractional_system_timestamp: u16, /* 02 None */
  local_timestamp: u32, /* 03 Some(FitType { name: "local_date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 04 None */
  system_timestamp_ms: u16, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SoftwareMessage {
  /* id == 35 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  version: u16, /* 03 None */
  part_number: String, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SlaveDeviceMessage {
  /* id == 106 */
  manufacturer: u16, /* 00 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
  product: u16, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CapabilitiesMessage {
  /* id == 1 */
  languages: u8, /* 00 None */
  sports: u8, /* 01 Some(FitType { name: "sport_bits_0", value_type: "uint8z", values: {"basketball": 64, "transition": 8, "swimming": 32, "generic": 1, "cycling": 4, "fitness_equipment": 16, "running": 2, "soccer": 128} }) */
  workouts_supported: u32, /* 15 Some(FitType { name: "workout_capabilities", value_type: "uint32z", values: {"firstbeat": 8, "grade": 4096, "speed": 128, "resistance": 8192, "tcx": 32, "fitness_equipment": 4, "protected": 16384, "new_leaf": 16, "distance": 512, "interval": 1, "power": 2048, "custom": 2, "heart_rate": 256, "cadence": 1024} }) */
  connectivity_supported: u32, /* 17 Some(FitType { name: "connectivity_capabilities", value_type: "uint32z", values: {"weather_alerts": 256, "connect_iq_app_managment": 1048576, "workout_download": 32, "weather_conditions": 128, "explicit_archive": 1024, "find_my_watch": 134217728, "audio_prompts": 16777216, "course_download": 16, "gps_ephemeris_download": 512, "true_up": 67108864, "incident_detection": 8388608, "golf_course_download": 16384, "live_track_messaging": 1073741824, "live_track": 64, "activity_upload": 8, "connect_iq_app_download": 8192, "connect_iq_data_field_download": 524288, "live_track_auto_start": 536870912, "connect_iq_watch_app_download": 65536, "swing_sensor_remote": 4194304, "remote_manual_sync": 268435456, "setup_incomplete": 2048, "bluetooth_le": 2, "ant": 4, "wifi_verification": 33554432, "bluetooth": 1, "connect_iq_watch_face_download": 262144, "continue_sync_after_software_update": 4096, "device_initiates_sync": 32768, "instant_input": 2147483648, "swing_sensor": 2097152, "connect_iq_widget_download": 131072} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FileCapabilitiesMessage {
  /* id == 37 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  _type: u8, /* 00 Some(FitType { name: "file", value_type: "enum", values: {"activity": 4, "segment_list": 35, "sport": 3, "workout": 5, "monitoring_a": 15, "exd_configuration": 40, "monitoring_daily": 28, "settings": 2, "device": 1, "course": 6, "totals": 10, "goals": 11, "weight": 9, "blood_pressure": 14, "schedules": 7, "mfg_range_max": 254, "segment": 34, "monitoring_b": 32, "activity_summary": 20, "mfg_range_min": 247} }) */
  flags: u8, /* 01 Some(FitType { name: "file_flags", value_type: "uint8z", values: {"erase": 8, "read": 2, "write": 4} }) */
  directory: String, /* 02 None */
  max_count: u16, /* 03 None */
  max_size: u32, /* 04 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MesgCapabilitiesMessage {
  /* id == 38 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  file: u8, /* 00 Some(FitType { name: "file", value_type: "enum", values: {"activity": 4, "segment_list": 35, "sport": 3, "workout": 5, "monitoring_a": 15, "exd_configuration": 40, "monitoring_daily": 28, "settings": 2, "device": 1, "course": 6, "totals": 10, "goals": 11, "weight": 9, "blood_pressure": 14, "schedules": 7, "mfg_range_max": 254, "segment": 34, "monitoring_b": 32, "activity_summary": 20, "mfg_range_min": 247} }) */
  mesg_num: u16, /* 01 Some(FitType { name: "mesg_num", value_type: "uint16", values: {"speed_zone": 53, "training_file": 72, "segment_leaderboard_entry": 149, "three_d_sensor_calibration": 167, "exd_data_concept_configuration": 202, "weather_conditions": 128, "camera_event": 161, "weather_alert": 129, "stress_level": 227, "gps_metadata": 160, "capabilities": 1, "file_id": 0, "blood_pressure": 51, "hrm_profile": 4, "cadence_zone": 131, "monitoring_info": 103, "user_profile": 3, "memo_glob": 145, "aviation_attitude": 178, "exd_screen_configuration": 200, "mfg_range_min": 65280, "activity": 34, "weight_scale": 30, "video_title": 185, "session": 18, "monitoring": 55, "hrv": 78, "developer_data_id": 207, "workout_session": 158, "bike_profile": 6, "ohr_settings": 188, "field_capabilities": 39, "totals": 33, "event": 21, "video_clip": 187, "workout_step": 27, "course_point": 32, "exd_data_field_configuration": 201, "segment_id": 148, "ant_channel_id": 82, "hr": 132, "segment_point": 150, "segment_file": 151, "sport": 12, "ant_rx": 80, "met_zone": 10, "accelerometer_data": 165, "lap": 19, "schedule": 28, "video_frame": 169, "zones_target": 7, "course": 31, "ant_tx": 81, "segment_lap": 142, "obdii_data": 174, "magnetometer_data": 208, "power_zone": 9, "device_info": 23, "mfg_range_max": 65534, "length": 101, "slave_device": 106, "one_d_sensor_calibration": 210, "mesg_capabilities": 38, "watchface_settings": 159, "field_description": 206, "barometer_data": 209, "pad": 105, "workout": 26, "sdm_profile": 5, "software": 35, "gyroscope_data": 164, "connectivity": 127, "file_capabilities": 37, "record": 20, "goal": 15, "video_description": 186, "nmea_sentence": 177, "video": 184, "timestamp_correlation": 162, "hr_zone": 8, "file_creator": 49, "device_settings": 2} }) */
  count_type: u8, /* 02 Some(FitType { name: "mesg_count", value_type: "enum", values: {"max_per_file": 1, "max_per_file_type": 2, "num_per_file": 0} }) */
  count: u16, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FieldCapabilitiesMessage {
  /* id == 39 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  file: u8, /* 00 Some(FitType { name: "file", value_type: "enum", values: {"activity": 4, "segment_list": 35, "sport": 3, "workout": 5, "monitoring_a": 15, "exd_configuration": 40, "monitoring_daily": 28, "settings": 2, "device": 1, "course": 6, "totals": 10, "goals": 11, "weight": 9, "blood_pressure": 14, "schedules": 7, "mfg_range_max": 254, "segment": 34, "monitoring_b": 32, "activity_summary": 20, "mfg_range_min": 247} }) */
  mesg_num: u16, /* 01 Some(FitType { name: "mesg_num", value_type: "uint16", values: {"speed_zone": 53, "training_file": 72, "segment_leaderboard_entry": 149, "three_d_sensor_calibration": 167, "exd_data_concept_configuration": 202, "weather_conditions": 128, "camera_event": 161, "weather_alert": 129, "stress_level": 227, "gps_metadata": 160, "capabilities": 1, "file_id": 0, "blood_pressure": 51, "hrm_profile": 4, "cadence_zone": 131, "monitoring_info": 103, "user_profile": 3, "memo_glob": 145, "aviation_attitude": 178, "exd_screen_configuration": 200, "mfg_range_min": 65280, "activity": 34, "weight_scale": 30, "video_title": 185, "session": 18, "monitoring": 55, "hrv": 78, "developer_data_id": 207, "workout_session": 158, "bike_profile": 6, "ohr_settings": 188, "field_capabilities": 39, "totals": 33, "event": 21, "video_clip": 187, "workout_step": 27, "course_point": 32, "exd_data_field_configuration": 201, "segment_id": 148, "ant_channel_id": 82, "hr": 132, "segment_point": 150, "segment_file": 151, "sport": 12, "ant_rx": 80, "met_zone": 10, "accelerometer_data": 165, "lap": 19, "schedule": 28, "video_frame": 169, "zones_target": 7, "course": 31, "ant_tx": 81, "segment_lap": 142, "obdii_data": 174, "magnetometer_data": 208, "power_zone": 9, "device_info": 23, "mfg_range_max": 65534, "length": 101, "slave_device": 106, "one_d_sensor_calibration": 210, "mesg_capabilities": 38, "watchface_settings": 159, "field_description": 206, "barometer_data": 209, "pad": 105, "workout": 26, "sdm_profile": 5, "software": 35, "gyroscope_data": 164, "connectivity": 127, "file_capabilities": 37, "record": 20, "goal": 15, "video_description": 186, "nmea_sentence": 177, "video": 184, "timestamp_correlation": 162, "hr_zone": 8, "file_creator": 49, "device_settings": 2} }) */
  field_num: u8, /* 02 None */
  count: u16, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DeviceSettingsMessage {
  /* id == 2 */
  active_time_zone: u8, /* 00 None */
  utc_offset: u32, /* 01 None */
  time_offset: u32, /* 02 None */
  time_mode: u8, /* 04 Some(FitType { name: "time_mode", value_type: "enum", values: {"hour_12_with_seconds": 3, "hour_24_with_seconds": 4, "utc": 5, "hour24": 1, "hour12": 0, "military": 2} }) */
  time_zone_offset: i8, /* 05 None */
  backlight_mode: u8, /* 0c Some(FitType { name: "backlight_mode", value_type: "enum", values: {"smart_notifications": 4, "key_and_messages": 2, "auto_brightness": 3, "key_and_messages_night": 5, "key_and_messages_and_smart_notifications": 6, "manual": 1, "off": 0} }) */
  activity_tracker_enabled: u8, /* 24 None */
  clock_time: u32, /* 27 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  pages_enabled: u16, /* 28 None */
  move_alert_enabled: u8, /* 2e None */
  date_mode: u8, /* 2f Some(FitType { name: "date_mode", value_type: "enum", values: {"day_month": 0, "month_day": 1} }) */
  display_orientation: u8, /* 37 Some(FitType { name: "display_orientation", value_type: "enum", values: {"portrait": 1, "portrait_flipped": 3, "landscape": 2, "landscape_flipped": 4, "auto": 0} }) */
  mounting_side: u8, /* 38 Some(FitType { name: "side", value_type: "enum", values: {"right": 0, "left": 1} }) */
  default_page: u16, /* 39 None */
  autosync_min_steps: u16, /* 3a None */
  autosync_min_time: u16, /* 3b None */
  lactate_threshold_autodetect_enabled: u8, /* 50 None */
  ble_auto_upload_enabled: u8, /* 56 None */
  auto_sync_frequency: u8, /* 59 Some(FitType { name: "auto_sync_frequency", value_type: "enum", values: {"never": 0, "remote": 4, "once_a_day": 3, "occasionally": 1, "frequent": 2} }) */
  auto_activity_detect: u32, /* 5a Some(FitType { name: "auto_activity_detect", value_type: "uint32", values: {"walking": 8, "elliptical": 32, "sedentary": 1024, "cycling": 2, "none": 0, "swimming": 4, "running": 1} }) */
  number_of_screens: u8, /* 5e None */
  smart_notification_display_orientation: u8, /* 5f Some(FitType { name: "display_orientation", value_type: "enum", values: {"portrait": 1, "portrait_flipped": 3, "landscape": 2, "landscape_flipped": 4, "auto": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct UserProfileMessage {
  /* id == 3 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  friendly_name: String, /* 00 None */
  gender: u8, /* 01 Some(FitType { name: "gender", value_type: "enum", values: {"male": 1, "female": 0} }) */
  age: u8, /* 02 None */
  height: u8, /* 03 None */
  weight: u16, /* 04 None */
  language: u8, /* 05 Some(FitType { name: "language", value_type: "enum", values: {"english": 0, "burmese": 36, "arabic": 22, "malaysian": 34, "spanish": 4, "bulgarian": 24, "japanese": 27, "ukrainian": 21, "turkish": 19, "hebrew": 31, "taiwanese": 29, "indonesian": 33, "dutch": 8, "portuguese": 14, "greek": 10, "slovakian": 15, "finnish": 9, "farsi": 23, "custom": 254, "thai": 30, "italian": 2, "romanian": 25, "slovenian": 16, "german": 3, "russian": 18, "brazilian_portuguese": 32, "chinese": 26, "czech": 6, "hungarian": 11, "latvian": 20, "mongolian": 37, "norwegian": 12, "french": 1, "danish": 7, "croatian": 5, "polish": 13, "korean": 28, "swedish": 17, "vietnamese": 35} }) */
  elev_setting: u8, /* 06 Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  weight_setting: u8, /* 07 Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  resting_heart_rate: u8, /* 08 None */
  default_max_running_heart_rate: u8, /* 09 None */
  default_max_biking_heart_rate: u8, /* 0a None */
  default_max_heart_rate: u8, /* 0b None */
  hr_setting: u8, /* 0c Some(FitType { name: "display_heart", value_type: "enum", values: {"bpm": 0, "max": 1, "reserve": 2} }) */
  speed_setting: u8, /* 0d Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  dist_setting: u8, /* 0e Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  power_setting: u8, /* 10 Some(FitType { name: "display_power", value_type: "enum", values: {"percent_ftp": 1, "watts": 0} }) */
  activity_class: u8, /* 11 Some(FitType { name: "activity_class", value_type: "enum", values: {"level_max": 100, "level": 127, "athlete": 128} }) */
  position_setting: u8, /* 12 Some(FitType { name: "display_position", value_type: "enum", values: {"icelandic_grid": 9, "dutch_grid": 5, "irish_grid": 23, "india_zone_IIIA": 18, "estonian_grid": 39, "mgrs_grid": 26, "british_grid": 4, "south_african_grid": 32, "swiss_grid": 33, "degree": 0, "loran": 24, "india_zone_IIA": 16, "india_zone_IA": 14, "borneo_rso": 38, "new_zealand_grid": 27, "austrian_grid": 3, "qatar_grid": 29, "swedish_grid": 31, "india_zone_0": 13, "india_zone_IB": 15, "swedish_ref_99_grid": 41, "degree_minute": 1, "indonesian_irian": 11, "india_zone_IVB": 21, "taiwan_grid": 34, "india_zone_IIB": 17, "maidenhead_grid": 25, "indonesian_southern": 12, "irish_transverse": 22, "indonesian_equatorial": 10, "west_malayan": 37, "hungarian_grid": 6, "india_zone_IIIB": 19, "united_states_grid": 35, "india_zone_IVA": 20, "utm_ups_grid": 36, "modified_swedish_grid": 30, "degree_minute_second": 2, "finnish_grid": 7, "new_zealand_transverse": 28, "german_grid": 8, "latvian_grid": 40} }) */
  temperature_setting: u8, /* 15 Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  local_id: u16, /* 16 Some(FitType { name: "user_local_id", value_type: "uint16", values: {"portable_max": 65534, "local_max": 15, "local_min": 0, "stationary_max": 255, "stationary_min": 16, "portable_min": 256} }) */
  global_id: Vec<u8>, /* 17 None */
  wake_time: u32, /* 1c Some(FitType { name: "localtime_into_day", value_type: "uint32", values: {} }) */
  sleep_time: u32, /* 1d Some(FitType { name: "localtime_into_day", value_type: "uint32", values: {} }) */
  height_setting: u8, /* 1e Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  user_running_step_length: u16, /* 1f None */
  user_walking_step_length: u16, /* 20 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HrmProfileMessage {
  /* id == 4 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  enabled: u8, /* 00 None */
  hrm_ant_id: u16, /* 01 None */
  log_hrv: u8, /* 02 None */
  hrm_ant_id_trans_type: u8, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SdmProfileMessage {
  /* id == 5 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  enabled: u8, /* 00 None */
  sdm_ant_id: u16, /* 01 None */
  sdm_cal_factor: u16, /* 02 None */
  odometer: u32, /* 03 None */
  speed_source: u8, /* 04 None */
  sdm_ant_id_trans_type: u8, /* 05 None */
  odometer_rollover: u8, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BikeProfileMessage {
  /* id == 6 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  name: String, /* 00 None */
  sport: u8, /* 01 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  sub_sport: u8, /* 02 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  odometer: u32, /* 03 None */
  bike_spd_ant_id: u16, /* 04 None */
  bike_cad_ant_id: u16, /* 05 None */
  bike_spdcad_ant_id: u16, /* 06 None */
  bike_power_ant_id: u16, /* 07 None */
  custom_wheelsize: u16, /* 08 None */
  auto_wheelsize: u16, /* 09 None */
  bike_weight: u16, /* 0a None */
  power_cal_factor: u16, /* 0b None */
  auto_wheel_cal: u8, /* 0c None */
  auto_power_zero: u8, /* 0d None */
  id: u8, /* 0e None */
  spd_enabled: u8, /* 0f None */
  cad_enabled: u8, /* 10 None */
  spdcad_enabled: u8, /* 11 None */
  power_enabled: u8, /* 12 None */
  crank_length: u8, /* 13 None */
  enabled: u8, /* 14 None */
  bike_spd_ant_id_trans_type: u8, /* 15 None */
  bike_cad_ant_id_trans_type: u8, /* 16 None */
  bike_spdcad_ant_id_trans_type: u8, /* 17 None */
  bike_power_ant_id_trans_type: u8, /* 18 None */
  odometer_rollover: u8, /* 25 None */
  front_gear_num: u8, /* 26 None */
  front_gear: u8, /* 27 None */
  rear_gear_num: u8, /* 28 None */
  rear_gear: u8, /* 29 None */
  shimano_di2_enabled: u8, /* 2c None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ConnectivityMessage {
  /* id == 127 */
  bluetooth_enabled: u8, /* 00 None */
  bluetooth_le_enabled: u8, /* 01 None */
  ant_enabled: u8, /* 02 None */
  name: String, /* 03 None */
  live_tracking_enabled: u8, /* 04 None */
  weather_conditions_enabled: u8, /* 05 None */
  weather_alerts_enabled: u8, /* 06 None */
  auto_activity_upload_enabled: u8, /* 07 None */
  course_download_enabled: u8, /* 08 None */
  workout_download_enabled: u8, /* 09 None */
  gps_ephemeris_download_enabled: u8, /* 0a None */
  incident_detection_enabled: u8, /* 0b None */
  grouptrack_enabled: u8, /* 0c None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WatchfaceSettingsMessage {
  /* id == 159 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  mode: u8, /* 00 Some(FitType { name: "watchface_mode", value_type: "enum", values: {"analog": 1, "digital": 0, "disabled": 3, "connect_iq": 2} }) */
  layout: Vec<u8>, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct OhrSettingsMessage {
  /* id == 188 */
  enabled: u8, /* 00 Some(FitType { name: "switch", value_type: "enum", values: {"on": 1, "auto": 2, "off": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ZonesTargetMessage {
  /* id == 7 */
  max_heart_rate: u8, /* 01 None */
  threshold_heart_rate: u8, /* 02 None */
  functional_threshold_power: u16, /* 03 None */
  hr_calc_type: u8, /* 05 Some(FitType { name: "hr_zone_calc", value_type: "enum", values: {"percent_max_hr": 1, "custom": 0, "percent_hrr": 2} }) */
  pwr_calc_type: u8, /* 07 Some(FitType { name: "pwr_zone_calc", value_type: "enum", values: {"custom": 0, "percent_ftp": 1} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SportMessage {
  /* id == 12 */
  sport: u8, /* 00 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  sub_sport: u8, /* 01 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  name: String, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HrZoneMessage {
  /* id == 8 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  high_bpm: u8, /* 01 None */
  name: String, /* 02 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SpeedZoneMessage {
  /* id == 53 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  high_value: u16, /* 00 None */
  name: String, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CadenceZoneMessage {
  /* id == 131 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  high_value: u8, /* 00 None */
  name: String, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PowerZoneMessage {
  /* id == 9 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  high_value: u16, /* 01 None */
  name: String, /* 02 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MetZoneMessage {
  /* id == 10 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  high_bpm: u8, /* 01 None */
  calories: u16, /* 02 None */
  fat_calories: u8, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GoalMessage {
  /* id == 15 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  sport: u8, /* 00 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  sub_sport: u8, /* 01 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  start_date: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  end_date: u32, /* 03 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  _type: u8, /* 04 Some(FitType { name: "goal", value_type: "enum", values: {"calories": 2, "distance": 1, "frequency": 3, "steps": 4, "active_minutes": 6, "ascent": 5, "time": 0} }) */
  value: u32, /* 05 None */
  repeat: u8, /* 06 None */
  target_value: u32, /* 07 None */
  recurrence: u8, /* 08 Some(FitType { name: "goal_recurrence", value_type: "enum", values: {"daily": 1, "monthly": 3, "weekly": 2, "custom": 5, "yearly": 4, "off": 0} }) */
  recurrence_value: u16, /* 09 None */
  enabled: u8, /* 0a None */
  source: u8, /* 0b Some(FitType { name: "goal_source", value_type: "enum", values: {"user": 2, "auto": 0, "community": 1} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ActivityMessage {
  /* id == 34 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  total_timer_time: u32, /* 00 None */
  num_sessions: u16, /* 01 None */
  _type: u8, /* 02 Some(FitType { name: "activity", value_type: "enum", values: {"auto_multi_sport": 1, "manual": 0} }) */
  event: u8, /* 03 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 04 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  local_timestamp: u32, /* 05 Some(FitType { name: "local_date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event_group: u8, /* 06 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SessionMessage {
  /* id == 18 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event: u8, /* 00 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 01 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  start_time: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  start_position_lat: i32, /* 03 None */
  start_position_long: i32, /* 04 None */
  sport: u8, /* 05 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  sub_sport: u8, /* 06 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  total_elapsed_time: u32, /* 07 None */
  total_timer_time: u32, /* 08 None */
  total_distance: u32, /* 09 None */
  total_cycles: u32, /* 0a None */
  total_calories: u16, /* 0b None */
  total_fat_calories: u16, /* 0d None */
  avg_speed: u16, /* 0e None */
  max_speed: u16, /* 0f None */
  avg_heart_rate: u8, /* 10 None */
  max_heart_rate: u8, /* 11 None */
  avg_cadence: u8, /* 12 None */
  max_cadence: u8, /* 13 None */
  avg_power: u16, /* 14 None */
  max_power: u16, /* 15 None */
  total_ascent: u16, /* 16 None */
  total_descent: u16, /* 17 None */
  total_training_effect: u8, /* 18 None */
  first_lap_index: u16, /* 19 None */
  num_laps: u16, /* 1a None */
  event_group: u8, /* 1b None */
  trigger: u8, /* 1c Some(FitType { name: "session_trigger", value_type: "enum", values: {"activity_end": 0, "auto_multi_sport": 2, "fitness_equipment": 3, "manual": 1} }) */
  nec_lat: i32, /* 1d None */
  nec_long: i32, /* 1e None */
  swc_lat: i32, /* 1f None */
  swc_long: i32, /* 20 None */
  normalized_power: u16, /* 22 None */
  training_stress_score: u16, /* 23 None */
  intensity_factor: u16, /* 24 None */
  left_right_balance: u16, /* 25 Some(FitType { name: "left_right_balance_100", value_type: "uint16", values: {"right": 32768, "mask": 16383} }) */
  avg_stroke_count: u32, /* 29 None */
  avg_stroke_distance: u16, /* 2a None */
  swim_stroke: u8, /* 2b Some(FitType { name: "swim_stroke", value_type: "enum", values: {"butterfly": 3, "breaststroke": 2, "backstroke": 1, "mixed": 5, "drill": 4, "freestyle": 0, "im": 6} }) */
  pool_length: u16, /* 2c None */
  threshold_power: u16, /* 2d None */
  pool_length_unit: u8, /* 2e Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
  num_active_lengths: u16, /* 2f None */
  total_work: u32, /* 30 None */
  avg_altitude: u16, /* 31 None */
  max_altitude: u16, /* 32 None */
  gps_accuracy: u8, /* 33 None */
  avg_grade: i16, /* 34 None */
  avg_pos_grade: i16, /* 35 None */
  avg_neg_grade: i16, /* 36 None */
  max_pos_grade: i16, /* 37 None */
  max_neg_grade: i16, /* 38 None */
  avg_temperature: i8, /* 39 None */
  max_temperature: i8, /* 3a None */
  total_moving_time: u32, /* 3b None */
  avg_pos_vertical_speed: i16, /* 3c None */
  avg_neg_vertical_speed: i16, /* 3d None */
  max_pos_vertical_speed: i16, /* 3e None */
  max_neg_vertical_speed: i16, /* 3f None */
  min_heart_rate: u8, /* 40 None */
  time_in_hr_zone: u32, /* 41 None */
  time_in_speed_zone: u32, /* 42 None */
  time_in_cadence_zone: u32, /* 43 None */
  time_in_power_zone: u32, /* 44 None */
  avg_lap_time: u32, /* 45 None */
  best_lap_index: u16, /* 46 None */
  min_altitude: u16, /* 47 None */
  player_score: u16, /* 52 None */
  opponent_score: u16, /* 53 None */
  opponent_name: String, /* 54 None */
  stroke_count: u16, /* 55 None */
  zone_count: u16, /* 56 None */
  max_ball_speed: u16, /* 57 None */
  avg_ball_speed: u16, /* 58 None */
  avg_vertical_oscillation: u16, /* 59 None */
  avg_stance_time_percent: u16, /* 5a None */
  avg_stance_time: u16, /* 5b None */
  avg_fractional_cadence: u8, /* 5c None */
  max_fractional_cadence: u8, /* 5d None */
  total_fractional_cycles: u8, /* 5e None */
  avg_total_hemoglobin_conc: u16, /* 5f None */
  min_total_hemoglobin_conc: u16, /* 60 None */
  max_total_hemoglobin_conc: u16, /* 61 None */
  avg_saturated_hemoglobin_percent: u16, /* 62 None */
  min_saturated_hemoglobin_percent: u16, /* 63 None */
  max_saturated_hemoglobin_percent: u16, /* 64 None */
  avg_left_torque_effectiveness: u8, /* 65 None */
  avg_right_torque_effectiveness: u8, /* 66 None */
  avg_left_pedal_smoothness: u8, /* 67 None */
  avg_right_pedal_smoothness: u8, /* 68 None */
  avg_combined_pedal_smoothness: u8, /* 69 None */
  sport_index: u8, /* 6f None */
  time_standing: u32, /* 70 None */
  stand_count: u16, /* 71 None */
  avg_left_pco: i8, /* 72 None */
  avg_right_pco: i8, /* 73 None */
  avg_left_power_phase: u8, /* 74 None */
  avg_left_power_phase_peak: u8, /* 75 None */
  avg_right_power_phase: u8, /* 76 None */
  avg_right_power_phase_peak: u8, /* 77 None */
  avg_power_position: u16, /* 78 None */
  max_power_position: u16, /* 79 None */
  avg_cadence_position: u8, /* 7a None */
  max_cadence_position: u8, /* 7b None */
  enhanced_avg_speed: u32, /* 7c None */
  enhanced_max_speed: u32, /* 7d None */
  enhanced_avg_altitude: u32, /* 7e None */
  enhanced_min_altitude: u32, /* 7f None */
  enhanced_max_altitude: u32, /* 80 None */
  avg_lev_motor_power: u16, /* 81 None */
  max_lev_motor_power: u16, /* 82 None */
  lev_battery_consumption: u8, /* 83 None */
  avg_vertical_ratio: u16, /* 84 None */
  avg_stance_time_balance: u16, /* 85 None */
  avg_step_length: u16, /* 86 None */
  total_anaerobic_training_effect: u8, /* 89 None */
  avg_vam: u16, /* 8b None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LapMessage {
  /* id == 19 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event: u8, /* 00 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 01 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  start_time: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  start_position_lat: i32, /* 03 None */
  start_position_long: i32, /* 04 None */
  end_position_lat: i32, /* 05 None */
  end_position_long: i32, /* 06 None */
  total_elapsed_time: u32, /* 07 None */
  total_timer_time: u32, /* 08 None */
  total_distance: u32, /* 09 None */
  total_cycles: u32, /* 0a None */
  total_calories: u16, /* 0b None */
  total_fat_calories: u16, /* 0c None */
  avg_speed: u16, /* 0d None */
  max_speed: u16, /* 0e None */
  avg_heart_rate: u8, /* 0f None */
  max_heart_rate: u8, /* 10 None */
  avg_cadence: u8, /* 11 None */
  max_cadence: u8, /* 12 None */
  avg_power: u16, /* 13 None */
  max_power: u16, /* 14 None */
  total_ascent: u16, /* 15 None */
  total_descent: u16, /* 16 None */
  intensity: u8, /* 17 Some(FitType { name: "intensity", value_type: "enum", values: {"cooldown": 3, "rest": 1, "warmup": 2, "active": 0} }) */
  lap_trigger: u8, /* 18 Some(FitType { name: "lap_trigger", value_type: "enum", values: {"position_lap": 4, "distance": 2, "position_marked": 6, "fitness_equipment": 8, "position_start": 3, "manual": 0, "position_waypoint": 5, "session_end": 7, "time": 1} }) */
  sport: u8, /* 19 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  event_group: u8, /* 1a None */
  num_lengths: u16, /* 20 None */
  normalized_power: u16, /* 21 None */
  left_right_balance: u16, /* 22 Some(FitType { name: "left_right_balance_100", value_type: "uint16", values: {"right": 32768, "mask": 16383} }) */
  first_length_index: u16, /* 23 None */
  avg_stroke_distance: u16, /* 25 None */
  swim_stroke: u8, /* 26 Some(FitType { name: "swim_stroke", value_type: "enum", values: {"butterfly": 3, "breaststroke": 2, "backstroke": 1, "mixed": 5, "drill": 4, "freestyle": 0, "im": 6} }) */
  sub_sport: u8, /* 27 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  num_active_lengths: u16, /* 28 None */
  total_work: u32, /* 29 None */
  avg_altitude: u16, /* 2a None */
  max_altitude: u16, /* 2b None */
  gps_accuracy: u8, /* 2c None */
  avg_grade: i16, /* 2d None */
  avg_pos_grade: i16, /* 2e None */
  avg_neg_grade: i16, /* 2f None */
  max_pos_grade: i16, /* 30 None */
  max_neg_grade: i16, /* 31 None */
  avg_temperature: i8, /* 32 None */
  max_temperature: i8, /* 33 None */
  total_moving_time: u32, /* 34 None */
  avg_pos_vertical_speed: i16, /* 35 None */
  avg_neg_vertical_speed: i16, /* 36 None */
  max_pos_vertical_speed: i16, /* 37 None */
  max_neg_vertical_speed: i16, /* 38 None */
  time_in_hr_zone: u32, /* 39 None */
  time_in_speed_zone: u32, /* 3a None */
  time_in_cadence_zone: u32, /* 3b None */
  time_in_power_zone: u32, /* 3c None */
  repetition_num: u16, /* 3d None */
  min_altitude: u16, /* 3e None */
  min_heart_rate: u8, /* 3f None */
  wkt_step_index: u16, /* 47 Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  opponent_score: u16, /* 4a None */
  stroke_count: u16, /* 4b None */
  zone_count: u16, /* 4c None */
  avg_vertical_oscillation: u16, /* 4d None */
  avg_stance_time_percent: u16, /* 4e None */
  avg_stance_time: u16, /* 4f None */
  avg_fractional_cadence: u8, /* 50 None */
  max_fractional_cadence: u8, /* 51 None */
  total_fractional_cycles: u8, /* 52 None */
  player_score: u16, /* 53 None */
  avg_total_hemoglobin_conc: u16, /* 54 None */
  min_total_hemoglobin_conc: u16, /* 55 None */
  max_total_hemoglobin_conc: u16, /* 56 None */
  avg_saturated_hemoglobin_percent: u16, /* 57 None */
  min_saturated_hemoglobin_percent: u16, /* 58 None */
  max_saturated_hemoglobin_percent: u16, /* 59 None */
  avg_left_torque_effectiveness: u8, /* 5b None */
  avg_right_torque_effectiveness: u8, /* 5c None */
  avg_left_pedal_smoothness: u8, /* 5d None */
  avg_right_pedal_smoothness: u8, /* 5e None */
  avg_combined_pedal_smoothness: u8, /* 5f None */
  time_standing: u32, /* 62 None */
  stand_count: u16, /* 63 None */
  avg_left_pco: i8, /* 64 None */
  avg_right_pco: i8, /* 65 None */
  avg_left_power_phase: u8, /* 66 None */
  avg_left_power_phase_peak: u8, /* 67 None */
  avg_right_power_phase: u8, /* 68 None */
  avg_right_power_phase_peak: u8, /* 69 None */
  avg_power_position: u16, /* 6a None */
  max_power_position: u16, /* 6b None */
  avg_cadence_position: u8, /* 6c None */
  max_cadence_position: u8, /* 6d None */
  enhanced_avg_speed: u32, /* 6e None */
  enhanced_max_speed: u32, /* 6f None */
  enhanced_avg_altitude: u32, /* 70 None */
  enhanced_min_altitude: u32, /* 71 None */
  enhanced_max_altitude: u32, /* 72 None */
  avg_lev_motor_power: u16, /* 73 None */
  max_lev_motor_power: u16, /* 74 None */
  lev_battery_consumption: u8, /* 75 None */
  avg_vertical_ratio: u16, /* 76 None */
  avg_stance_time_balance: u16, /* 77 None */
  avg_step_length: u16, /* 78 None */
  avg_vam: u16, /* 79 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct LengthMessage {
  /* id == 101 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event: u8, /* 00 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 01 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  start_time: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  total_elapsed_time: u32, /* 03 None */
  total_timer_time: u32, /* 04 None */
  total_strokes: u16, /* 05 None */
  avg_speed: u16, /* 06 None */
  swim_stroke: u8, /* 07 Some(FitType { name: "swim_stroke", value_type: "enum", values: {"butterfly": 3, "breaststroke": 2, "backstroke": 1, "mixed": 5, "drill": 4, "freestyle": 0, "im": 6} }) */
  avg_swimming_cadence: u8, /* 09 None */
  event_group: u8, /* 0a None */
  total_calories: u16, /* 0b None */
  length_type: u8, /* 0c Some(FitType { name: "length_type", value_type: "enum", values: {"active": 1, "idle": 0} }) */
  player_score: u16, /* 12 None */
  opponent_score: u16, /* 13 None */
  stroke_count: u16, /* 14 None */
  zone_count: u16, /* 15 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RecordMessage {
  /* id == 20 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  position_lat: i32, /* 00 None */
  position_long: i32, /* 01 None */
  altitude: u16, /* 02 None */
  heart_rate: u8, /* 03 None */
  cadence: u8, /* 04 None */
  distance: u32, /* 05 None */
  speed: u16, /* 06 None */
  power: u16, /* 07 None */
  compressed_speed_distance: Vec<u8>, /* 08 None */
  grade: i16, /* 09 None */
  resistance: u8, /* 0a None */
  time_from_course: i32, /* 0b None */
  cycle_length: u8, /* 0c None */
  temperature: i8, /* 0d None */
  speed_1s: u8, /* 11 None */
  cycles: u8, /* 12 None */
  total_cycles: u32, /* 13 None */
  compressed_accumulated_power: u16, /* 1c None */
  accumulated_power: u32, /* 1d None */
  left_right_balance: u8, /* 1e Some(FitType { name: "left_right_balance", value_type: "uint8", values: {"right": 128, "mask": 127} }) */
  gps_accuracy: u8, /* 1f None */
  vertical_speed: i16, /* 20 None */
  calories: u16, /* 21 None */
  vertical_oscillation: u16, /* 27 None */
  stance_time_percent: u16, /* 28 None */
  stance_time: u16, /* 29 None */
  activity_type: u8, /* 2a Some(FitType { name: "activity_type", value_type: "enum", values: {"walking": 6, "all": 254, "sedentary": 8, "generic": 0, "cycling": 2, "fitness_equipment": 4, "transition": 3, "swimming": 5, "running": 1} }) */
  left_torque_effectiveness: u8, /* 2b None */
  right_torque_effectiveness: u8, /* 2c None */
  left_pedal_smoothness: u8, /* 2d None */
  right_pedal_smoothness: u8, /* 2e None */
  combined_pedal_smoothness: u8, /* 2f None */
  time128: u8, /* 30 None */
  stroke_type: u8, /* 31 Some(FitType { name: "stroke_type", value_type: "enum", values: {"smash": 5, "serve": 2, "no_event": 0, "other": 1, "forehand": 3, "backhand": 4} }) */
  zone: u8, /* 32 None */
  ball_speed: u16, /* 33 None */
  cadence256: u16, /* 34 None */
  fractional_cadence: u8, /* 35 None */
  total_hemoglobin_conc: u16, /* 36 None */
  total_hemoglobin_conc_min: u16, /* 37 None */
  total_hemoglobin_conc_max: u16, /* 38 None */
  saturated_hemoglobin_percent: u16, /* 39 None */
  saturated_hemoglobin_percent_min: u16, /* 3a None */
  saturated_hemoglobin_percent_max: u16, /* 3b None */
  device_index: u8, /* 3e Some(FitType { name: "device_index", value_type: "uint8", values: {"creator": 0} }) */
  left_pco: i8, /* 43 None */
  right_pco: i8, /* 44 None */
  left_power_phase: u8, /* 45 None */
  left_power_phase_peak: u8, /* 46 None */
  right_power_phase: u8, /* 47 None */
  right_power_phase_peak: u8, /* 48 None */
  enhanced_speed: u32, /* 49 None */
  enhanced_altitude: u32, /* 4e None */
  battery_soc: u8, /* 51 None */
  motor_power: u16, /* 52 None */
  vertical_ratio: u16, /* 53 None */
  stance_time_balance: u16, /* 54 None */
  step_length: u16, /* 55 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventMessage {
  /* id == 21 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event: u8, /* 00 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 01 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  data16: u16, /* 02 None */
  data: u32, /* 03 None */
  event_group: u8, /* 04 None */
  score: u16, /* 07 None */
  opponent_score: u16, /* 08 None */
  front_gear_num: u8, /* 09 None */
  front_gear: u8, /* 0a None */
  rear_gear_num: u8, /* 0b None */
  rear_gear: u8, /* 0c None */
  device_index: u8, /* 0d Some(FitType { name: "device_index", value_type: "uint8", values: {"creator": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DeviceInfoMessage {
  /* id == 23 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  device_index: u8, /* 00 Some(FitType { name: "device_index", value_type: "uint8", values: {"creator": 0} }) */
  device_type: u8, /* 01 None */
  manufacturer: u16, /* 02 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
  serial_number: u32, /* 03 None */
  product: u16, /* 04 None */
  software_version: u16, /* 05 None */
  hardware_version: u8, /* 06 None */
  cum_operating_time: u32, /* 07 None */
  battery_voltage: u16, /* 0a None */
  battery_status: u8, /* 0b Some(FitType { name: "battery_status", value_type: "uint8", values: {"good": 2, "charging": 6, "new": 1, "low": 4, "critical": 5, "ok": 3, "unknown": 7} }) */
  sensor_position: u8, /* 12 Some(FitType { name: "body_location", value_type: "enum", values: {"left_lower_back": 13, "left_leg": 0, "left_forearm_extensors": 27, "right_shoulder": 29, "right_brachioradialis": 32, "right_quad": 10, "right_bicep": 30, "waist_right": 39, "left_shin": 2, "left_tricep": 25, "right_calf": 7, "right_glute": 11, "right_hamstring": 9, "left_quad": 4, "left_calf": 1, "right_chest": 21, "left_bicep": 24, "right_forearm_extensors": 33, "left_chest": 19, "left_shoulder": 23, "throat": 35, "torso_front": 17, "left_glute": 5, "left_arm": 22, "left_upper_back": 14, "left_hamstring": 3, "waist_left": 38, "waist_front": 37, "right_upper_back": 16, "left_abdomen": 18, "neck": 34, "right_tricep": 31, "right_leg": 6, "right_lower_back": 15, "right_arm": 28, "right_shin": 8, "right_abdomen": 20, "left_brachioradialis": 26, "waist_mid_back": 36, "torso_back": 12} }) */
  descriptor: String, /* 13 None */
  ant_transmission_type: u8, /* 14 None */
  ant_device_number: u16, /* 15 None */
  ant_network: u8, /* 16 Some(FitType { name: "ant_network", value_type: "enum", values: {"public": 0, "antplus": 1, "private": 3, "antfs": 2} }) */
  source_type: u8, /* 19 Some(FitType { name: "source_type", value_type: "enum", values: {"antplus": 1, "bluetooth_low_energy": 3, "wifi": 4, "local": 5, "ant": 0, "bluetooth": 2} }) */
  product_name: String, /* 1b None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TrainingFileMessage {
  /* id == 72 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  _type: u8, /* 00 Some(FitType { name: "file", value_type: "enum", values: {"activity": 4, "segment_list": 35, "sport": 3, "workout": 5, "monitoring_a": 15, "exd_configuration": 40, "monitoring_daily": 28, "settings": 2, "device": 1, "course": 6, "totals": 10, "goals": 11, "weight": 9, "blood_pressure": 14, "schedules": 7, "mfg_range_max": 254, "segment": 34, "monitoring_b": 32, "activity_summary": 20, "mfg_range_min": 247} }) */
  manufacturer: u16, /* 01 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
  product: u16, /* 02 None */
  serial_number: u32, /* 03 None */
  time_created: u32, /* 04 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HrvMessage {
  /* id == 78 */
  time: u16, /* 00 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WeatherConditionsMessage {
  /* id == 128 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  weather_report: u8, /* 00 Some(FitType { name: "weather_report", value_type: "enum", values: {"forecast": 1, "current": 0, "hourly_forecast": 1, "daily_forecast": 2} }) */
  temperature: i8, /* 01 None */
  condition: u8, /* 02 Some(FitType { name: "weather_status", value_type: "enum", values: {"light_rain_snow": 20, "light_rain": 16, "heavy_rain": 17, "snow": 4, "light_snow": 18, "mostly_cloudy": 2, "fog": 8, "partly_cloudy": 1, "heavy_rain_snow": 21, "heavy_snow": 19, "rain": 3, "hazy": 11, "unknown_precipitation": 15, "scattered_showers": 13, "hail": 12, "wintry_mix": 7, "thunderstorms": 6, "clear": 0, "cloudy": 22, "scattered_thunderstorms": 14, "windy": 5} }) */
  wind_direction: u16, /* 03 None */
  wind_speed: u16, /* 04 None */
  precipitation_probability: u8, /* 05 None */
  temperature_feels_like: i8, /* 06 None */
  relative_humidity: u8, /* 07 None */
  location: String, /* 08 None */
  observed_at_time: u32, /* 09 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  observed_location_lat: i32, /* 0a None */
  observed_location_long: i32, /* 0b None */
  day_of_week: u8, /* 0c Some(FitType { name: "day_of_week", value_type: "enum", values: {"monday": 1, "sunday": 0, "friday": 5, "thursday": 4, "tuesday": 2, "saturday": 6, "wednesday": 3} }) */
  high_temperature: i8, /* 0d None */
  low_temperature: i8, /* 0e None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WeatherAlertMessage {
  /* id == 129 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  report_id: String, /* 00 None */
  issue_time: u32, /* 01 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  expire_time: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  severity: u8, /* 03 Some(FitType { name: "weather_severity", value_type: "enum", values: {"statement": 4, "advisory": 3, "watch": 2, "unknown": 0, "warning": 1} }) */
  _type: u8, /* 04 Some(FitType { name: "weather_severe_type", value_type: "enum", values: {"storm": 40, "flood": 75, "humidex_and_health": 50, "lake_effect_blowing_snow": 29, "inland_tropical_storm": 15, "inland_hurricane": 6, "snow_and_blowing_snow": 35, "flash_freeze": 20, "special_weather": 84, "marine_weather": 58, "small_craft_winds": 63, "high_surf": 77, "areal_flood": 43, "lake_wind": 57, "ashfall": 66, "heavy_freezing_spray": 24, "blowing_snow": 36, "lake_effect_snow": 31, "ice_storm": 17, "freezing_spray": 53, "dense_fog": 68, "dense_smoke": 69, "freeze": 72, "small_craft": 62, "severe_thunderstorm": 9, "rip_tide": 76, "wreckhouse_winds": 10, "snow_alert": 37, "rainfall": 42, "low_water": 82, "debris_flow": 19, "tornado": 1, "wind_chill": 26, "arctic_outflow": 38, "blizzard": 16, "blowing_dust": 70, "lakeshore_flood": 45, "snow_squall": 30, "hazardous_seas": 61, "extreme_wind": 4, "freezing_fog": 67, "hard_freeze": 71, "fire_weather": 74, "flash_flood": 13, "small_craft_rough_bar": 64, "heavy_snow_alert": 28, "winter_weather": 32, "snowfall": 34, "freezing_rain": 18, "extreme_cold": 25, "hydrological": 83, "cold_wave": 27, "high_water_level": 65, "tropical_storm": 14, "strong_wind": 56, "brisk_wind": 80, "sleet": 33, "waterspout": 8, "tsunami": 2, "humidex": 51, "high_wind": 22, "freezing_drizzle": 39, "air_stagnation": 81, "hurricane": 3, "wind": 59, "unspecified": 0, "storm_surge": 41, "avalanche": 12, "weather": 48, "high_heat_and_humidity": 49, "heat": 47, "air_quality": 79, "hurricane_force_wind": 7, "special_marine": 54, "winter_storm": 23, "small_craft_hazardous_seas": 60, "dust_storm": 21, "squall": 55, "gale": 52, "les_suetes_wind": 11, "typhoon": 5, "coastal_flood": 44, "excessive_heat": 46, "frost": 73, "smog": 78} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GpsMetadataMessage {
  /* id == 160 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  position_lat: i32, /* 01 None */
  position_long: i32, /* 02 None */
  enhanced_altitude: u32, /* 03 None */
  enhanced_speed: u32, /* 04 None */
  heading: u16, /* 05 None */
  utc_timestamp: u32, /* 06 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  velocity: i16, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CameraEventMessage {
  /* id == 161 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  camera_event_type: u8, /* 01 Some(FitType { name: "camera_event_type", value_type: "enum", values: {"video_split": 1, "video_second_stream_split": 5, "video_resume": 13, "video_end": 2, "video_second_stream_pause": 12, "video_second_stream_split_start": 8, "video_start": 0, "photo_taken": 3, "video_split_start": 7, "video_second_stream_end": 6, "video_second_stream_start": 4, "video_second_stream_resume": 14, "video_pause": 11} }) */
  camera_file_uuid: String, /* 02 None */
  camera_orientation: u8, /* 03 Some(FitType { name: "camera_orientation_type", value_type: "enum", values: {"camera_orientation_90": 1, "camera_orientation_270": 3, "camera_orientation_180": 2, "camera_orientation_0": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GyroscopeDataMessage {
  /* id == 164 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  sample_time_offset: u16, /* 01 None */
  gyro_x: u16, /* 02 None */
  gyro_y: u16, /* 03 None */
  gyro_z: u16, /* 04 None */
  calibrated_gyro_x: f32, /* 05 None */
  calibrated_gyro_y: f32, /* 06 None */
  calibrated_gyro_z: f32, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccelerometerDataMessage {
  /* id == 165 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  sample_time_offset: u16, /* 01 None */
  accel_x: u16, /* 02 None */
  accel_y: u16, /* 03 None */
  accel_z: u16, /* 04 None */
  calibrated_accel_x: f32, /* 05 None */
  calibrated_accel_y: f32, /* 06 None */
  calibrated_accel_z: f32, /* 07 None */
  compressed_calibrated_accel_x: i16, /* 08 None */
  compressed_calibrated_accel_y: i16, /* 09 None */
  compressed_calibrated_accel_z: i16, /* 0a None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MagnetometerDataMessage {
  /* id == 208 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  sample_time_offset: u16, /* 01 None */
  mag_x: u16, /* 02 None */
  mag_y: u16, /* 03 None */
  mag_z: u16, /* 04 None */
  calibrated_mag_x: f32, /* 05 None */
  calibrated_mag_y: f32, /* 06 None */
  calibrated_mag_z: f32, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BarometerDataMessage {
  /* id == 209 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  sample_time_offset: u16, /* 01 None */
  baro_pres: u32, /* 02 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ThreeDSensorCalibrationMessage {
  /* id == 167 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  sensor_type: u8, /* 00 Some(FitType { name: "sensor_type", value_type: "enum", values: {"barometer": 3, "compass": 2, "gyroscope": 1, "accelerometer": 0} }) */
  calibration_factor: u32, /* 01 None */
  calibration_divisor: u32, /* 02 None */
  level_shift: u32, /* 03 None */
  offset_cal: i32, /* 04 None */
  orientation_matrix: i32, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct OneDSensorCalibrationMessage {
  /* id == 210 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  sensor_type: u8, /* 00 Some(FitType { name: "sensor_type", value_type: "enum", values: {"barometer": 3, "compass": 2, "gyroscope": 1, "accelerometer": 0} }) */
  calibration_factor: u32, /* 01 None */
  calibration_divisor: u32, /* 02 None */
  level_shift: u32, /* 03 None */
  offset_cal: i32, /* 04 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoFrameMessage {
  /* id == 169 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  frame_number: u32, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ObdiiDataMessage {
  /* id == 174 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  time_offset: u16, /* 01 None */
  pid: Vec<u8>, /* 02 None */
  raw_data: Vec<u8>, /* 03 None */
  pid_data_size: u8, /* 04 None */
  system_time: u32, /* 05 None */
  start_timestamp: u32, /* 06 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  start_timestamp_ms: u16, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct NmeaSentenceMessage {
  /* id == 177 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  sentence: String, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AviationAttitudeMessage {
  /* id == 178 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timestamp_ms: u16, /* 00 None */
  system_time: u32, /* 01 None */
  pitch: i16, /* 02 None */
  roll: i16, /* 03 None */
  accel_lateral: i16, /* 04 None */
  accel_normal: i16, /* 05 None */
  turn_rate: i16, /* 06 None */
  stage: u8, /* 07 Some(FitType { name: "attitude_stage", value_type: "enum", values: {"failed": 0, "degraded": 2, "valid": 3, "aligning": 1} }) */
  attitude_stage_complete: u8, /* 08 None */
  track: u16, /* 09 None */
  validity: u16, /* 0a Some(FitType { name: "attitude_validity", value_type: "uint16", values: {"hw_fail": 64, "no_gps": 256, "normal_body_accel_valid": 16, "solution_coasting": 1024, "track_angle_heading_valid": 1, "mag_invalid": 128, "pitch_valid": 2, "magnetic_heading": 4096, "turn_rate_valid": 32, "roll_valid": 4, "gps_invalid": 512, "lateral_body_accel_valid": 8, "true_track_angle": 2048} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoMessage {
  /* id == 184 */
  url: String, /* 00 None */
  hosting_provider: String, /* 01 None */
  duration: u32, /* 02 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoTitleMessage {
  /* id == 185 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  message_count: u16, /* 00 None */
  text: String, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoDescriptionMessage {
  /* id == 186 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  message_count: u16, /* 00 None */
  text: String, /* 01 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct VideoClipMessage {
  /* id == 187 */
  clip_number: u16, /* 00 None */
  start_timestamp: u32, /* 01 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  start_timestamp_ms: u16, /* 02 None */
  end_timestamp: u32, /* 03 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  end_timestamp_ms: u16, /* 04 None */
  clip_start: u32, /* 06 None */
  clip_end: u32, /* 07 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CourseMessage {
  /* id == 31 */
  sport: u8, /* 04 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  name: String, /* 05 None */
  capabilities: u32, /* 06 Some(FitType { name: "course_capabilities", value_type: "uint32z", values: {"processed": 1, "time": 4, "position": 16, "distance": 8, "power": 64, "navigation": 512, "bikeway": 1024, "training": 256, "valid": 2, "heart_rate": 32, "cadence": 128} }) */
  sub_sport: u8, /* 07 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct CoursePointMessage {
  /* id == 32 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* 01 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  position_lat: i32, /* 02 None */
  position_long: i32, /* 03 None */
  distance: u32, /* 04 None */
  _type: u8, /* 05 Some(FitType { name: "course_point", value_type: "enum", values: {"water": 3, "right": 7, "middle_fork": 18, "sharp_right": 22, "right_fork": 17, "danger": 5, "first_aid": 9, "left": 6, "fourth_category": 10, "segment_end": 25, "sprint": 15, "sharp_left": 20, "third_category": 11, "hors_category": 14, "straight": 8, "generic": 0, "food": 4, "u_turn": 23, "valley": 2, "summit": 1, "second_category": 12, "first_category": 13, "left_fork": 16, "segment_start": 24, "slight_right": 21, "slight_left": 19} }) */
  name: String, /* 06 None */
  favorite: u8, /* 08 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SegmentIdMessage {
  /* id == 148 */
  name: String, /* 00 None */
  uuid: String, /* 01 None */
  sport: u8, /* 02 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  enabled: u8, /* 03 None */
  user_profile_primary_key: u32, /* 04 None */
  device_id: u32, /* 05 None */
  default_race_leader: u8, /* 06 None */
  delete_status: u8, /* 07 Some(FitType { name: "segment_delete_status", value_type: "enum", values: {"delete_one": 1, "delete_all": 2, "do_not_delete": 0} }) */
  selection_type: u8, /* 08 Some(FitType { name: "segment_selection_type", value_type: "enum", values: {"suggested": 1, "starred": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SegmentLeaderboardEntryMessage {
  /* id == 149 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  name: String, /* 00 None */
  _type: u8, /* 01 Some(FitType { name: "segment_leaderboard_type", value_type: "enum", values: {"connections": 2, "overall": 0, "group": 3, "kom": 5, "qom": 6, "pr": 7, "rival": 9, "club_leader": 10, "personal_best": 1, "challenger": 4, "goal": 8} }) */
  group_primary_key: u32, /* 02 None */
  activity_id: u32, /* 03 None */
  segment_time: u32, /* 04 None */
  activity_id_string: String, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SegmentPointMessage {
  /* id == 150 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  position_lat: i32, /* 01 None */
  position_long: i32, /* 02 None */
  distance: u32, /* 03 None */
  altitude: u16, /* 04 None */
  leader_time: u32, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SegmentLapMessage {
  /* id == 142 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  event: u8, /* 00 Some(FitType { name: "event", value_type: "enum", values: {"hr_low_alert": 14, "distance_duration_alert": 24, "rear_gear_change": 43, "recovery_hr": 21, "rider_position_change": 44, "time_duration_alert": 23, "elev_high_alert": 45, "front_gear_change": 42, "activity": 26, "session": 8, "sport_point": 33, "off_course": 7, "calibration": 36, "hr_high_alert": 13, "cad_low_alert": 18, "workout_step": 4, "course_point": 10, "power_low_alert": 20, "timer": 0, "user_marker": 32, "virtual_partner_pace": 12, "comm_timeout": 47, "lap": 9, "battery": 11, "speed_high_alert": 15, "length": 28, "elev_low_alert": 46, "workout": 3, "power_up": 6, "speed_low_alert": 16, "power_down": 5, "fitness_equipment": 27, "calorie_duration_alert": 25, "power_high_alert": 19, "cad_high_alert": 17, "battery_low": 22} }) */
  event_type: u8, /* 01 Some(FitType { name: "event_type", value_type: "enum", values: {"consecutive_depreciated": 2, "end_depreciated": 6, "stop": 1, "end_all_depreciated": 7, "marker": 3, "stop_disable_all": 9, "stop_disable": 8, "start": 0, "stop_all": 4, "begin_depreciated": 5} }) */
  start_time: u32, /* 02 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  start_position_lat: i32, /* 03 None */
  start_position_long: i32, /* 04 None */
  end_position_lat: i32, /* 05 None */
  end_position_long: i32, /* 06 None */
  total_elapsed_time: u32, /* 07 None */
  total_timer_time: u32, /* 08 None */
  total_distance: u32, /* 09 None */
  total_cycles: u32, /* 0a None */
  total_calories: u16, /* 0b None */
  total_fat_calories: u16, /* 0c None */
  avg_speed: u16, /* 0d None */
  max_speed: u16, /* 0e None */
  avg_heart_rate: u8, /* 0f None */
  max_heart_rate: u8, /* 10 None */
  avg_cadence: u8, /* 11 None */
  max_cadence: u8, /* 12 None */
  avg_power: u16, /* 13 None */
  max_power: u16, /* 14 None */
  total_ascent: u16, /* 15 None */
  total_descent: u16, /* 16 None */
  sport: u8, /* 17 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  event_group: u8, /* 18 None */
  nec_lat: i32, /* 19 None */
  nec_long: i32, /* 1a None */
  swc_lat: i32, /* 1b None */
  swc_long: i32, /* 1c None */
  name: String, /* 1d None */
  normalized_power: u16, /* 1e None */
  left_right_balance: u16, /* 1f Some(FitType { name: "left_right_balance_100", value_type: "uint16", values: {"right": 32768, "mask": 16383} }) */
  sub_sport: u8, /* 20 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  total_work: u32, /* 21 None */
  avg_altitude: u16, /* 22 None */
  max_altitude: u16, /* 23 None */
  gps_accuracy: u8, /* 24 None */
  avg_grade: i16, /* 25 None */
  avg_pos_grade: i16, /* 26 None */
  avg_neg_grade: i16, /* 27 None */
  max_pos_grade: i16, /* 28 None */
  max_neg_grade: i16, /* 29 None */
  avg_temperature: i8, /* 2a None */
  max_temperature: i8, /* 2b None */
  total_moving_time: u32, /* 2c None */
  avg_pos_vertical_speed: i16, /* 2d None */
  avg_neg_vertical_speed: i16, /* 2e None */
  max_pos_vertical_speed: i16, /* 2f None */
  max_neg_vertical_speed: i16, /* 30 None */
  time_in_hr_zone: u32, /* 31 None */
  time_in_speed_zone: u32, /* 32 None */
  time_in_cadence_zone: u32, /* 33 None */
  time_in_power_zone: u32, /* 34 None */
  repetition_num: u16, /* 35 None */
  min_altitude: u16, /* 36 None */
  min_heart_rate: u8, /* 37 None */
  active_time: u32, /* 38 None */
  wkt_step_index: u16, /* 39 Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  sport_event: u8, /* 3a Some(FitType { name: "sport_event", value_type: "enum", values: {"touring": 8, "geocaching": 1, "training": 6, "uncategorized": 0, "special_event": 5, "fitness": 2, "race": 4, "transportation": 7, "recreation": 3} }) */
  avg_left_torque_effectiveness: u8, /* 3b None */
  avg_right_torque_effectiveness: u8, /* 3c None */
  avg_left_pedal_smoothness: u8, /* 3d None */
  avg_right_pedal_smoothness: u8, /* 3e None */
  avg_combined_pedal_smoothness: u8, /* 3f None */
  status: u8, /* 40 Some(FitType { name: "segment_lap_status", value_type: "enum", values: {"end": 0, "fail": 1} }) */
  uuid: String, /* 41 None */
  avg_fractional_cadence: u8, /* 42 None */
  max_fractional_cadence: u8, /* 43 None */
  total_fractional_cycles: u8, /* 44 None */
  front_gear_shift_count: u16, /* 45 None */
  rear_gear_shift_count: u16, /* 46 None */
  time_standing: u32, /* 47 None */
  stand_count: u16, /* 48 None */
  avg_left_pco: i8, /* 49 None */
  avg_right_pco: i8, /* 4a None */
  avg_left_power_phase: u8, /* 4b None */
  avg_left_power_phase_peak: u8, /* 4c None */
  avg_right_power_phase: u8, /* 4d None */
  avg_right_power_phase_peak: u8, /* 4e None */
  avg_power_position: u16, /* 4f None */
  max_power_position: u16, /* 50 None */
  avg_cadence_position: u8, /* 51 None */
  max_cadence_position: u8, /* 52 None */
  manufacturer: u16, /* 53 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct SegmentFileMessage {
  /* id == 151 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  file_uuid: String, /* 01 None */
  enabled: u8, /* 03 None */
  user_profile_primary_key: u32, /* 04 None */
  leader_type: u8, /* 07 Some(FitType { name: "segment_leaderboard_type", value_type: "enum", values: {"connections": 2, "overall": 0, "group": 3, "kom": 5, "qom": 6, "pr": 7, "rival": 9, "club_leader": 10, "personal_best": 1, "challenger": 4, "goal": 8} }) */
  leader_group_primary_key: u32, /* 08 None */
  leader_activity_id: u32, /* 09 None */
  leader_activity_id_string: String, /* 0a None */
  default_race_leader: u8, /* 0b None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WorkoutMessage {
  /* id == 26 */
  sport: u8, /* 04 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  capabilities: u32, /* 05 Some(FitType { name: "workout_capabilities", value_type: "uint32z", values: {"firstbeat": 8, "grade": 4096, "speed": 128, "resistance": 8192, "tcx": 32, "fitness_equipment": 4, "protected": 16384, "new_leaf": 16, "distance": 512, "interval": 1, "power": 2048, "custom": 2, "heart_rate": 256, "cadence": 1024} }) */
  num_valid_steps: u16, /* 06 None */
  wkt_name: String, /* 08 None */
  sub_sport: u8, /* 0b Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  pool_length: u16, /* 0e None */
  pool_length_unit: u8, /* 0f Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WorkoutSessionMessage {
  /* id == 158 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  sport: u8, /* 00 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  sub_sport: u8, /* 01 Some(FitType { name: "sub_sport", value_type: "enum", values: {"recumbent": 10, "indoor_rowing": 14, "elliptical": 15, "casual_walking": 30, "atv": 35, "swim_to_bike_transition": 34, "commuting": 48, "navigate": 50, "mountain": 8, "whitewater": 41, "track_cycling": 13, "yoga": 43, "hand_cycling": 12, "cyclocross": 11, "obstacle": 59, "lap_swimming": 17, "e_bike_mountain": 47, "mixed_surface": 49, "e_bike_fitness": 28, "virtual_activity": 58, "pilates": 44, "track": 4, "indoor_skiing": 25, "track_me": 51, "indoor_cycling": 6, "exercise": 23, "wingsuit": 40, "rc_drone": 39, "spin": 5, "motocross": 36, "skate_skiing": 42, "stair_climbing": 16, "road": 7, "all": 254, "backcountry": 37, "run_to_bike_transition": 33, "strength_training": 20, "indoor_walking": 27, "cardio_training": 26, "generic": 0, "speed_walking": 31, "challenge": 24, "indoor_running": 45, "resort": 38, "flexibility_training": 19, "warm_up": 21, "bike_to_run_transition": 32, "bmx": 29, "treadmill": 1, "downhill": 9, "open_water": 18, "match": 22, "gravel_cycling": 46, "map": 52, "trail": 3, "street": 2} }) */
  num_valid_steps: u16, /* 02 None */
  first_step_index: u16, /* 03 None */
  pool_length: u16, /* 04 None */
  pool_length_unit: u8, /* 05 Some(FitType { name: "display_measure", value_type: "enum", values: {"nautical": 2, "metric": 0, "statute": 1} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WorkoutStepMessage {
  /* id == 27 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  wkt_step_name: String, /* 00 None */
  duration_type: u8, /* 01 Some(FitType { name: "wkt_step_duration", value_type: "enum", values: {"calories": 4, "distance": 1, "power_less_than": 14, "power_greater_than": 15, "training_peaks_tss": 16, "repeat_until_power_greater_than": 13, "repeat_until_steps_cmplt": 6, "power_30s_greater_than": 24, "repeat_until_power_last_lap_less_than": 17, "power_lap_less_than": 25, "open": 5, "hr_less_than": 2, "repeat_until_hr_less_than": 10, "repeat_until_calories": 9, "hr_greater_than": 3, "power_10s_greater_than": 23, "power_10s_less_than": 20, "repeat_until_training_peaks_tss": 27, "power_3s_less_than": 19, "power_lap_greater_than": 26, "repeat_until_time": 7, "repeat_until_distance": 8, "repeat_until_hr_greater_than": 11, "repeat_until_max_power_last_lap_less_than": 18, "power_30s_less_than": 21, "repetition_time": 28, "power_3s_greater_than": 22, "time": 0, "repeat_until_power_less_than": 12} }) */
  duration_value: u32, /* 02 None */
  target_type: u8, /* 03 Some(FitType { name: "wkt_step_target", value_type: "enum", values: {"grade": 5, "heart_rate_lap": 13, "power": 4, "swim_stroke": 11, "open": 2, "power_30s": 9, "speed": 0, "speed_lap": 12, "power_3s": 7, "resistance": 6, "power_lap": 10, "heart_rate": 1, "power_10s": 8, "cadence": 3} }) */
  target_value: u32, /* 04 None */
  custom_target_value_low: u32, /* 05 None */
  custom_target_value_high: u32, /* 06 None */
  intensity: u8, /* 07 Some(FitType { name: "intensity", value_type: "enum", values: {"cooldown": 3, "rest": 1, "warmup": 2, "active": 0} }) */
  notes: String, /* 08 None */
  equipment: u8, /* 09 Some(FitType { name: "workout_equipment", value_type: "enum", values: {"swim_paddles": 3, "swim_snorkel": 5, "swim_fins": 1, "none": 0, "swim_pull_buoy": 4, "swim_kickboard": 2} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ScheduleMessage {
  /* id == 28 */
  manufacturer: u16, /* 00 Some(FitType { name: "manufacturer", value_type: "uint16", values: {"salutron": 110, "sound_of_motion": 94, "minoura": 278, "spark_hk": 10, "watteam": 261, "echowell": 12, "a_and_d": 21, "ifor_powell": 54, "stryd": 95, "alatech_technology_ltd": 58, "powerbahn": 78, "lemond_fitness": 30, "virtualtraining": 284, "dynovelo": 264, "life_time_fitness": 276, "metrigear": 17, "garmin": 1, "holux": 39, "seiko_epson": 52, "ciclosport": 77, "trainer_road": 281, "MiPulse": 97, "id_bike": 62, "topaction_technology": 104, "ace_sensor": 43, "navman": 269, "srm": 6, "timex": 16, "thita_elektronik": 24, "elite": 86, "cobi": 270, "latitude_limited": 113, "concept2": 40, "thinkrider": 116, "bsx_athletics": 98, "cardiosport": 20, "podoon": 275, "specialized": 63, "clean_mobile": 26, "bontrager": 81, "xplova": 45, "peripedal": 72, "dynastream_oem": 13, "fitcare": 106, "one_giant_leap": 42, "cycliq": 279, "cosinuss": 105, "gpulse": 25, "pioneer": 48, "idt": 5, "favero_electronics": 263, "breakaway": 57, "tigrasport": 109, "metalogics": 50, "garmin_fr405_antfs": 2, "archinoetics": 34, "actigraphcorp": 5759, "peaksware": 28, "magellan": 37, "1partcarbon": 92, "zephyr": 3, "tomtom": 71, "dk_city": 88, "sigmasport": 70, "cateye": 68, "stages_cycling": 69, "acorn_projects_aps": 79, "magtonic": 91, "dynastream": 15, "geonaute": 61, "tacx": 89, "saxonar": 29, "icg": 96, "wattbike": 73, "physical_enterprises": 65, "bkool": 67, "xelic": 18, "dayton": 4, "campagnolo_srl": 100, "wahoo_fitness": 32, "wtek": 64, "healthandlife": 257, "mio_technology_europe": 59, "lezyne": 258, "sensitivus_gauge": 274, "woodway": 85, "perception_digital": 46, "beurer": 19, "bryton": 267, "citizen_systems": 36, "igpsport": 115, "scosche": 83, "octane_fitness": 33, "luxottica": 280, "tanita": 11, "ibike": 8, "giant_manufacturing_co": 108, "praxisworks": 102, "nielsen_kellerman": 87, "spivi": 271, "osynce": 38, "seiko_epson_oem": 53, "moxy": 76, "magura": 84, "hmm": 22, "bryton_sensors": 112, "spantec": 49, "saris": 9, "north_pole_engineering": 66, "sram": 268, "magene": 107, "nautilus": 14, "technogym": 111, "zwift": 260, "development": 255, "inside_ride_technologies": 93, "fullspeedahead": 283, "bf1systems": 47, "evesports": 273, "brim_brothers": 44, "maxwell_guider": 55, "soaring_technology": 114, "lifebeam": 80, "strava": 265, "pedal_brain": 27, "rotor": 60, "direction_technology": 90, "the_hurt_box": 35, "recon": 262, "body_bike_smart": 101, "dexcom": 31, "mio_magellan": 272, "star_trac": 56, "scribe_labs": 259, "wellgo": 82, "the_sufferfest": 282, "suunto": 23, "look": 99, "falco_e_motors": 277, "limits_technology": 103, "precor": 266, "4iiiis": 51, "quarq": 7} }) */
  product: u16, /* 01 None */
  serial_number: u32, /* 02 None */
  time_created: u32, /* 03 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  completed: u8, /* 04 None */
  _type: u8, /* 05 Some(FitType { name: "schedule", value_type: "enum", values: {"course": 1, "workout": 0} }) */
  scheduled_time: u32, /* 06 Some(FitType { name: "local_date_time", value_type: "uint32", values: {"min": 268435456} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TotalsMessage {
  /* id == 33 */
  message_index: u16, /* fe Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  timer_time: u32, /* 00 None */
  distance: u32, /* 01 None */
  calories: u32, /* 02 None */
  sport: u8, /* 03 Some(FitType { name: "sport", value_type: "enum", values: {"walking": 11, "tennis": 8, "basketball": 6, "e_biking": 21, "fishing": 29, "motorcycling": 22, "rock_climbing": 31, "hunting": 28, "kitesurfing": 44, "transition": 3, "swimming": 5, "mountaineering": 16, "wakeboarding": 39, "american_football": 9, "multisport": 18, "ice_skating": 33, "golf": 25, "training": 10, "rowing": 15, "boxing": 47, "water_skiing": 40, "snowshoeing": 35, "kayaking": 41, "all": 254, "paddling": 19, "sky_diving": 34, "alpine_skiing": 13, "sailing": 32, "cross_country_skiing": 12, "snowmobiling": 36, "boating": 23, "tactical": 45, "surfing": 38, "flying": 20, "generic": 0, "cycling": 2, "windsurfing": 43, "horseback_riding": 27, "floor_climbing": 48, "fitness_equipment": 4, "jumpmaster": 46, "hang_gliding": 26, "driving": 24, "stand_up_paddleboarding": 37, "inline_skating": 30, "hiking": 17, "running": 1, "snowboarding": 14, "soccer": 7, "rafting": 42} }) */
  elapsed_time: u32, /* 04 None */
  sessions: u16, /* 05 None */
  active_time: u32, /* 06 None */
  sport_index: u8, /* 09 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WeightScaleMessage {
  /* id == 30 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  weight: u16, /* 00 Some(FitType { name: "weight", value_type: "uint16", values: {"calculating": 65534} }) */
  percent_fat: u16, /* 01 None */
  percent_hydration: u16, /* 02 None */
  visceral_fat_mass: u16, /* 03 None */
  bone_mass: u16, /* 04 None */
  muscle_mass: u16, /* 05 None */
  basal_met: u16, /* 07 None */
  physique_rating: u8, /* 08 None */
  active_met: u16, /* 09 None */
  metabolic_age: u8, /* 0a None */
  visceral_fat_rating: u8, /* 0b None */
  user_profile_index: u16, /* 0c Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BloodPressureMessage {
  /* id == 51 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  systolic_pressure: u16, /* 00 None */
  diastolic_pressure: u16, /* 01 None */
  mean_arterial_pressure: u16, /* 02 None */
  map_3_sample_mean: u16, /* 03 None */
  map_morning_values: u16, /* 04 None */
  map_evening_values: u16, /* 05 None */
  heart_rate: u8, /* 06 None */
  heart_rate_type: u8, /* 07 Some(FitType { name: "hr_type", value_type: "enum", values: {"irregular": 1, "normal": 0} }) */
  status: u8, /* 08 Some(FitType { name: "bp_status", value_type: "enum", values: {"error_data_out_of_range": 3, "error_incomplete_data": 1, "no_error": 0, "error_no_measurement": 2, "error_irregular_heart_rate": 4} }) */
  user_profile_index: u16, /* 09 Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MonitoringInfoMessage {
  /* id == 103 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  local_timestamp: u32, /* 00 Some(FitType { name: "local_date_time", value_type: "uint32", values: {"min": 268435456} }) */
  activity_type: u8, /* 01 Some(FitType { name: "activity_type", value_type: "enum", values: {"walking": 6, "all": 254, "sedentary": 8, "generic": 0, "cycling": 2, "fitness_equipment": 4, "transition": 3, "swimming": 5, "running": 1} }) */
  cycles_to_distance: u16, /* 03 None */
  cycles_to_calories: u16, /* 04 None */
  resting_metabolic_rate: u16, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MonitoringMessage {
  /* id == 55 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  device_index: u8, /* 00 Some(FitType { name: "device_index", value_type: "uint8", values: {"creator": 0} }) */
  calories: u16, /* 01 None */
  distance: u32, /* 02 None */
  cycles: u32, /* 03 None */
  active_time: u32, /* 04 None */
  activity_type: u8, /* 05 Some(FitType { name: "activity_type", value_type: "enum", values: {"walking": 6, "all": 254, "sedentary": 8, "generic": 0, "cycling": 2, "fitness_equipment": 4, "transition": 3, "swimming": 5, "running": 1} }) */
  activity_subtype: u8, /* 06 Some(FitType { name: "activity_subtype", value_type: "enum", values: {"indoor_rowing": 14, "elliptical": 15, "mountain": 8, "cyclocross": 11, "track": 4, "indoor_cycling": 6, "stair_climbing": 16, "road": 7, "all": 254, "generic": 0, "treadmill": 1, "downhill": 9, "open_water": 18, "trail": 3, "street": 2, "recumbent": 10, "track_cycling": 13, "hand_cycling": 12, "lap_swimming": 17, "spin": 5} }) */
  activity_level: u8, /* 07 Some(FitType { name: "activity_level", value_type: "enum", values: {"medium": 1, "low": 0, "high": 2} }) */
  distance_16: u16, /* 08 None */
  cycles_16: u16, /* 09 None */
  active_time_16: u16, /* 0a None */
  local_timestamp: u32, /* 0b Some(FitType { name: "local_date_time", value_type: "uint32", values: {"min": 268435456} }) */
  temperature: i16, /* 0c None */
  temperature_min: i16, /* 0e None */
  temperature_max: i16, /* 0f None */
  activity_time: u16, /* 10 None */
  active_calories: u16, /* 13 None */
  current_activity_type_intensity: Vec<u8>, /* 18 None */
  timestamp_min_8: u8, /* 19 None */
  timestamp_16: u16, /* 1a None */
  heart_rate: u8, /* 1b None */
  intensity: u8, /* 1c None */
  duration_min: u16, /* 1d None */
  duration: u32, /* 1e None */
  ascent: u32, /* 1f None */
  descent: u32, /* 20 None */
  moderate_activity_minutes: u16, /* 21 None */
  vigorous_activity_minutes: u16, /* 22 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct HrMessage {
  /* id == 132 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  fractional_timestamp: u16, /* 00 None */
  time256: u8, /* 01 None */
  filtered_bpm: u8, /* 06 None */
  event_timestamp: u32, /* 09 None */
  event_timestamp_12: Vec<u8>, /* 0a None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StressLevelMessage {
  /* id == 227 */
  stress_level_value: i16, /* 00 None */
  stress_level_time: u32, /* 01 Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MemoGlobMessage {
  /* id == 145 */
  part_index: u32, /* fa None */
  memo: Vec<u8>, /* 00 None */
  message_number: u16, /* 01 None */
  message_index: u16, /* 02 Some(FitType { name: "message_index", value_type: "uint16", values: {"reserved": 28672, "mask": 4095, "selected": 32768} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AntChannelIdMessage {
  /* id == 82 */
  channel_number: u8, /* 00 None */
  device_type: u8, /* 01 None */
  device_number: u16, /* 02 None */
  transmission_type: u8, /* 03 None */
  device_index: u8, /* 04 Some(FitType { name: "device_index", value_type: "uint8", values: {"creator": 0} }) */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AntRxMessage {
  /* id == 80 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  fractional_timestamp: u16, /* 00 None */
  mesg_id: Vec<u8>, /* 01 None */
  mesg_data: Vec<u8>, /* 02 None */
  channel_number: u8, /* 03 None */
  data: Vec<u8>, /* 04 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AntTxMessage {
  /* id == 81 */
  timestamp: u32, /* fd Some(FitType { name: "date_time", value_type: "uint32", values: {"min": 268435456} }) */
  fractional_timestamp: u16, /* 00 None */
  mesg_id: Vec<u8>, /* 01 None */
  mesg_data: Vec<u8>, /* 02 None */
  channel_number: u8, /* 03 None */
  data: Vec<u8>, /* 04 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExdScreenConfigurationMessage {
  /* id == 200 */
  screen_index: u8, /* 00 None */
  field_count: u8, /* 01 None */
  layout: u8, /* 02 Some(FitType { name: "exd_layout", value_type: "enum", values: {"full_quarter_split": 5, "full_screen": 0, "half_horizontal_bottom_split": 4, "half_vertical_left_split": 6, "half_vertical_right_split": 3, "half_vertical": 1, "half_horizontal": 2, "half_horizontal_top_split": 7} }) */
  screen_enabled: u8, /* 03 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExdDataFieldConfigurationMessage {
  /* id == 201 */
  screen_index: u8, /* 00 None */
  concept_field: Vec<u8>, /* 01 None */
  field_id: u8, /* 02 None */
  concept_count: u8, /* 03 None */
  display_type: u8, /* 04 Some(FitType { name: "exd_display_type", value_type: "enum", values: {"circle_graph": 4, "simple": 1, "bar": 3, "balance": 6, "virtual_partner": 5, "numerical": 0, "gauge": 10, "string_list": 7, "graph": 2, "string": 8, "simple_dynamic_icon": 9} }) */
  title: String, /* 05 None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExdDataConceptConfigurationMessage {
  /* id == 202 */
  screen_index: u8, /* 00 None */
  concept_field: Vec<u8>, /* 01 None */
  field_id: u8, /* 02 None */
  concept_index: u8, /* 03 None */
  data_page: u8, /* 04 None */
  concept_key: u8, /* 05 None */
  scaling: u8, /* 06 None */
  data_units: u8, /* 08 Some(FitType { name: "exd_data_units", value_type: "enum", values: {"degrees": 12, "lights": 29, "meters": 14, "feet": 16, "degrees_farenheit": 7, "enum_turn_type": 21, "meters_per_min": 47, "kilojoules": 34, "enum_bike_light_network_config_type": 28, "meters_per_hour": 5, "milliseconds": 35, "rpm": 10, "kilometers": 15, "watts_per_kilogram": 24, "inches_hg": 42, "hecto_pascals": 45, "bpm": 11, "centimeter": 38, "second_per_mile": 36, "eight_cardinal": 49, "kilometers_per_hour": 3, "degrees_celsius": 6, "minutes": 31, "hours": 32, "time": 20, "enum_bike_light_battery_status": 27, "second_per_kilometer": 37, "calories": 33, "meters_per_sec": 48, "bradians": 40, "seconds": 30, "zone": 8, "mm_hg": 43, "enum_sport": 41, "percent": 22, "feet_per_hour": 4, "enum_course_point": 39, "feet_per_min": 46, "mbars": 44, "enum_bike_light_beam_angle_mode": 26, "enum_battery_status": 25, "gear": 9, "yards": 17, "millimeters": 13, "laps": 1, "no_units": 0, "kilofeet": 18, "miles": 19, "miles_per_hour": 2, "watts": 23} }) */
  qualifier: u8, /* 09 Some(FitType { name: "exd_qualifiers", value_type: "enum", values: {"ten_second_average": 15, "last_lap": 7, "minimum_24h": 25, "stopped": 33, "shifter": 30, "to_destination": 9, "third": 29, "instantaneous": 1, "sunset": 22, "average": 2, "second": 28, "maximum_lap": 6, "to_go": 10, "moving": 32, "percent_maximum": 17, "last_sport": 31, "zone_6": 245, "zone_7": 244, "elapsed": 20, "zone_8": 243, "zone_1": 250, "thirty_second_average": 16, "three_second_average": 14, "total": 13, "to_next": 11, "maximum_average": 5, "maximum_24h": 24, "percent_maximum_average": 18, "zone_4": 247, "minimum": 26, "next_course_point": 12, "average_lap": 8, "maximum": 4, "lap_percent_maximum": 19, "no_qualifier": 0, "lap": 3, "zone_3": 248, "zone_2": 249, "zone_9": 242, "sunrise": 21, "zone_5": 246, "compared_to_virtual_partner": 23, "first": 27} }) */
  descriptor: u8, /* 0a Some(FitType { name: "exd_descriptors", value_type: "enum", values: {"right_power_phase_finish_angle": 64, "heart_rate_reserve": 27, "grade": 16, "estimated_time_of_arrival": 7, "course_distance": 47, "trainer_target_power": 12, "light_network_mode": 3, "gps_heading": 86, "off_course": 90, "time_in_power_zone": 77, "gps_elevation": 87, "time_standing": 14, "vertical_oscillation": 68, "bike_light_battery_status": 0, "course_heading": 53, "speed": 43, "navigation_estimated_time_of_arrival": 50, "ground_contact_time": 70, "time_in_heart_rate_zone": 26, "torque_effectiveness": 56, "performance_condition": 75, "course": 89, "beam_angle_status": 1, "heading": 8, "power_zone": 55, "number_lights_connected": 4, "battery_level": 10, "navigation_time": 52, "time_of_day": 32, "gear_ratio": 23, "glide_ratio": 91, "rear_gear": 22, "compass_heading": 85, "gps_signal_strength": 30, "gears": 65, "balance": 33, "gps_accuracy": 29, "vertical_speed": 19, "work": 38, "course_estimated_time_of_arrival": 49, "pedal_smoothness": 34, "intensity_factor": 37, "course_location": 79, "vertical_ratio": 69, "functional_threshold_power": 36, "time": 9, "left_power_phase_finish_angle": 63, "power_ratio": 39, "calories": 28, "normalized_power": 40, "power_weight_ratio": 58, "training_effect": 67, "time_seated": 13, "navigation_location": 80, "vertical_distance": 92, "right_ground_contact_time_balance": 72, "distance": 6, "gear_combo": 82, "training_stress_Score": 41, "navigation_turn": 78, "navigation_heading": 54, "di2_battery_level": 20, "ambient_pressure": 94, "power": 35, "vmg": 93, "icon": 84, "workout_step": 46, "pressure": 95, "left_platform_center_offset": 59, "compass": 81, "time_on_zone": 42, "left_power_phase_start_angle": 61, "front_gear": 21, "muscle_oxygen": 83, "vam": 96, "pace": 66, "course_time": 51, "right_power_phase_start_angle": 62, "descent": 18, "anaerobic_training_effect": 88, "elevation": 15, "navigation_distance": 48, "timer_time": 57, "stride_length": 73, "ascent": 17, "course_type": 76, "right_platform_center_offset": 60, "heart_rate_zone": 25, "batery_level": 2, "laps": 44, "heart_rate": 24, "trainer_resistance": 11, "left_ground_contact_time_balance": 71, "temperature": 31, "reps": 45, "cadence": 5, "running_cadence": 74} }) */
  is_signed: u8, /* 0b None */
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FieldDescriptionMessage {
  /* id == 206 */
  developer_data_index: u8, /* 00 None */
  field_definition_number: u8, /* 01 None */
  fit_base_type_id: u8, /* 02 Some(FitType { name: "fit_base_type", value_type: "uint8", values: {"sint16": 131, "byte": 13, "float32": 136, "uint32z": 140, "uint64z": 144, "sint32": 133, "uint16z": 139, "sint64": 142, "sint8": 1, "uint8": 2, "float64": 137, "uint8z": 10, "uint64": 143, "string": 7, "enum": 0, "uint32": 134, "uint16": 132} }) */
  field_name: String, /* 03 None */
  array: u8, /* 04 None */
  components: String, /* 05 None */
  scale: u8, /* 06 None */
  offset: i8, /* 07 None */
  units: String, /* 08 None */
  bits: String, /* 09 None */
  accumulate: String, /* 0a None */
  fit_base_unit_id: u16, /* 0d Some(FitType { name: "fit_base_unit", value_type: "uint16", values: {"pound": 2, "other": 0, "kilogram": 1} }) */
  native_mesg_num: u16, /* 0e Some(FitType { name: "mesg_num", value_type: "uint16", values: {"speed_zone": 53, "training_file": 72, "segment_leaderboard_entry": 149, "three_d_sensor_calibration": 167, "exd_data_concept_configuration": 202, "weather_conditions": 128, "camera_event": 161, "weather_alert": 129, "stress_level": 227, "gps_metadata": 160, "capabilities": 1, "file_id": 0, "blood_pressure": 51, "hrm_profile": 4, "cadence_zone": 131, "monitoring_info": 103, "user_profile": 3, "memo_glob": 145, "aviation_attitude": 178, "exd_screen_configuration": 200, "mfg_range_min": 65280, "activity": 34, "weight_scale": 30, "video_title": 185, "session": 18, "monitoring": 55, "hrv": 78, "developer_data_id": 207, "workout_session": 158, "bike_profile": 6, "ohr_settings": 188, "field_capabilities": 39, "totals": 33, "event": 21, "video_clip": 187, "workout_step": 27, "course_point": 32, "exd_data_field_configuration": 201, "segment_id": 148, "ant_channel_id": 82, "hr": 132, "segment_point": 150, "segment_file": 151, "sport": 12, "ant_rx": 80, "met_zone": 10, "accelerometer_data": 165, "lap": 19, "schedule": 28, "video_frame": 169, "zones_target": 7, "course": 31, "ant_tx": 81, "segment_lap": 142, "obdii_data": 174, "magnetometer_data": 208, "power_zone": 9, "device_info": 23, "mfg_range_max": 65534, "length": 101, "slave_device": 106, "one_d_sensor_calibration": 210, "mesg_capabilities": 38, "watchface_settings": 159, "field_description": 206, "barometer_data": 209, "pad": 105, "workout": 26, "sdm_profile": 5, "software": 35, "gyroscope_data": 164, "connectivity": 127, "file_capabilities": 37, "record": 20, "goal": 15, "video_description": 186, "nmea_sentence": 177, "video": 184, "timestamp_correlation": 162, "hr_zone": 8, "file_creator": 49, "device_settings": 2} }) */
  native_field_num: u8, /* 0f None */
}
pub fn get_type_name(message_id: u16, field_id: u8) -> Option<&'static str> {
  match message_id {
    0 => {
      match field_id {
        0 => Some("type"),
        1 => Some("manufacturer"),
        2 => Some("product"),
        3 => Some("serial_number"),
        4 => Some("time_created"),
        5 => Some("number"),
        8 => Some("product_name"),
        _ => None,
      }
    },
    49 => {
      match field_id {
        0 => Some("software_version"),
        1 => Some("hardware_version"),
        _ => None,
      }
    },
    162 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("fractional_timestamp"),
        1 => Some("system_timestamp"),
        2 => Some("fractional_system_timestamp"),
        3 => Some("local_timestamp"),
        4 => Some("timestamp_ms"),
        5 => Some("system_timestamp_ms"),
        _ => None,
      }
    },
    35 => {
      match field_id {
        254 => Some("message_index"),
        3 => Some("version"),
        5 => Some("part_number"),
        _ => None,
      }
    },
    106 => {
      match field_id {
        0 => Some("manufacturer"),
        1 => Some("product"),
        _ => None,
      }
    },
    1 => {
      match field_id {
        0 => Some("languages"),
        1 => Some("sports"),
        21 => Some("workouts_supported"),
        23 => Some("connectivity_supported"),
        _ => None,
      }
    },
    37 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("type"),
        1 => Some("flags"),
        2 => Some("directory"),
        3 => Some("max_count"),
        4 => Some("max_size"),
        _ => None,
      }
    },
    38 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("file"),
        1 => Some("mesg_num"),
        2 => Some("count_type"),
        3 => Some("count"),
        _ => None,
      }
    },
    39 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("file"),
        1 => Some("mesg_num"),
        2 => Some("field_num"),
        3 => Some("count"),
        _ => None,
      }
    },
    2 => {
      match field_id {
        0 => Some("active_time_zone"),
        1 => Some("utc_offset"),
        2 => Some("time_offset"),
        4 => Some("time_mode"),
        5 => Some("time_zone_offset"),
        12 => Some("backlight_mode"),
        36 => Some("activity_tracker_enabled"),
        39 => Some("clock_time"),
        40 => Some("pages_enabled"),
        46 => Some("move_alert_enabled"),
        47 => Some("date_mode"),
        55 => Some("display_orientation"),
        56 => Some("mounting_side"),
        57 => Some("default_page"),
        58 => Some("autosync_min_steps"),
        59 => Some("autosync_min_time"),
        80 => Some("lactate_threshold_autodetect_enabled"),
        86 => Some("ble_auto_upload_enabled"),
        89 => Some("auto_sync_frequency"),
        90 => Some("auto_activity_detect"),
        94 => Some("number_of_screens"),
        95 => Some("smart_notification_display_orientation"),
        _ => None,
      }
    },
    3 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("friendly_name"),
        1 => Some("gender"),
        2 => Some("age"),
        3 => Some("height"),
        4 => Some("weight"),
        5 => Some("language"),
        6 => Some("elev_setting"),
        7 => Some("weight_setting"),
        8 => Some("resting_heart_rate"),
        9 => Some("default_max_running_heart_rate"),
        10 => Some("default_max_biking_heart_rate"),
        11 => Some("default_max_heart_rate"),
        12 => Some("hr_setting"),
        13 => Some("speed_setting"),
        14 => Some("dist_setting"),
        16 => Some("power_setting"),
        17 => Some("activity_class"),
        18 => Some("position_setting"),
        21 => Some("temperature_setting"),
        22 => Some("local_id"),
        23 => Some("global_id"),
        28 => Some("wake_time"),
        29 => Some("sleep_time"),
        30 => Some("height_setting"),
        31 => Some("user_running_step_length"),
        32 => Some("user_walking_step_length"),
        _ => None,
      }
    },
    4 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("enabled"),
        1 => Some("hrm_ant_id"),
        2 => Some("log_hrv"),
        3 => Some("hrm_ant_id_trans_type"),
        _ => None,
      }
    },
    5 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("enabled"),
        1 => Some("sdm_ant_id"),
        2 => Some("sdm_cal_factor"),
        3 => Some("odometer"),
        4 => Some("speed_source"),
        5 => Some("sdm_ant_id_trans_type"),
        7 => Some("odometer_rollover"),
        _ => None,
      }
    },
    6 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("name"),
        1 => Some("sport"),
        2 => Some("sub_sport"),
        3 => Some("odometer"),
        4 => Some("bike_spd_ant_id"),
        5 => Some("bike_cad_ant_id"),
        6 => Some("bike_spdcad_ant_id"),
        7 => Some("bike_power_ant_id"),
        8 => Some("custom_wheelsize"),
        9 => Some("auto_wheelsize"),
        10 => Some("bike_weight"),
        11 => Some("power_cal_factor"),
        12 => Some("auto_wheel_cal"),
        13 => Some("auto_power_zero"),
        14 => Some("id"),
        15 => Some("spd_enabled"),
        16 => Some("cad_enabled"),
        17 => Some("spdcad_enabled"),
        18 => Some("power_enabled"),
        19 => Some("crank_length"),
        20 => Some("enabled"),
        21 => Some("bike_spd_ant_id_trans_type"),
        22 => Some("bike_cad_ant_id_trans_type"),
        23 => Some("bike_spdcad_ant_id_trans_type"),
        24 => Some("bike_power_ant_id_trans_type"),
        37 => Some("odometer_rollover"),
        38 => Some("front_gear_num"),
        39 => Some("front_gear"),
        40 => Some("rear_gear_num"),
        41 => Some("rear_gear"),
        44 => Some("shimano_di2_enabled"),
        _ => None,
      }
    },
    127 => {
      match field_id {
        0 => Some("bluetooth_enabled"),
        1 => Some("bluetooth_le_enabled"),
        2 => Some("ant_enabled"),
        3 => Some("name"),
        4 => Some("live_tracking_enabled"),
        5 => Some("weather_conditions_enabled"),
        6 => Some("weather_alerts_enabled"),
        7 => Some("auto_activity_upload_enabled"),
        8 => Some("course_download_enabled"),
        9 => Some("workout_download_enabled"),
        10 => Some("gps_ephemeris_download_enabled"),
        11 => Some("incident_detection_enabled"),
        12 => Some("grouptrack_enabled"),
        _ => None,
      }
    },
    159 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("mode"),
        1 => Some("layout"),
        _ => None,
      }
    },
    188 => {
      match field_id {
        0 => Some("enabled"),
        _ => None,
      }
    },
    7 => {
      match field_id {
        1 => Some("max_heart_rate"),
        2 => Some("threshold_heart_rate"),
        3 => Some("functional_threshold_power"),
        5 => Some("hr_calc_type"),
        7 => Some("pwr_calc_type"),
        _ => None,
      }
    },
    12 => {
      match field_id {
        0 => Some("sport"),
        1 => Some("sub_sport"),
        3 => Some("name"),
        _ => None,
      }
    },
    8 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("high_bpm"),
        2 => Some("name"),
        _ => None,
      }
    },
    53 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("high_value"),
        1 => Some("name"),
        _ => None,
      }
    },
    131 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("high_value"),
        1 => Some("name"),
        _ => None,
      }
    },
    9 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("high_value"),
        2 => Some("name"),
        _ => None,
      }
    },
    10 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("high_bpm"),
        2 => Some("calories"),
        3 => Some("fat_calories"),
        _ => None,
      }
    },
    15 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("sport"),
        1 => Some("sub_sport"),
        2 => Some("start_date"),
        3 => Some("end_date"),
        4 => Some("type"),
        5 => Some("value"),
        6 => Some("repeat"),
        7 => Some("target_value"),
        8 => Some("recurrence"),
        9 => Some("recurrence_value"),
        10 => Some("enabled"),
        11 => Some("source"),
        _ => None,
      }
    },
    34 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("total_timer_time"),
        1 => Some("num_sessions"),
        2 => Some("type"),
        3 => Some("event"),
        4 => Some("event_type"),
        5 => Some("local_timestamp"),
        6 => Some("event_group"),
        _ => None,
      }
    },
    18 => {
      match field_id {
        254 => Some("message_index"),
        253 => Some("timestamp"),
        0 => Some("event"),
        1 => Some("event_type"),
        2 => Some("start_time"),
        3 => Some("start_position_lat"),
        4 => Some("start_position_long"),
        5 => Some("sport"),
        6 => Some("sub_sport"),
        7 => Some("total_elapsed_time"),
        8 => Some("total_timer_time"),
        9 => Some("total_distance"),
        10 => Some("total_cycles"),
        11 => Some("total_calories"),
        13 => Some("total_fat_calories"),
        14 => Some("avg_speed"),
        15 => Some("max_speed"),
        16 => Some("avg_heart_rate"),
        17 => Some("max_heart_rate"),
        18 => Some("avg_cadence"),
        19 => Some("max_cadence"),
        20 => Some("avg_power"),
        21 => Some("max_power"),
        22 => Some("total_ascent"),
        23 => Some("total_descent"),
        24 => Some("total_training_effect"),
        25 => Some("first_lap_index"),
        26 => Some("num_laps"),
        27 => Some("event_group"),
        28 => Some("trigger"),
        29 => Some("nec_lat"),
        30 => Some("nec_long"),
        31 => Some("swc_lat"),
        32 => Some("swc_long"),
        34 => Some("normalized_power"),
        35 => Some("training_stress_score"),
        36 => Some("intensity_factor"),
        37 => Some("left_right_balance"),
        41 => Some("avg_stroke_count"),
        42 => Some("avg_stroke_distance"),
        43 => Some("swim_stroke"),
        44 => Some("pool_length"),
        45 => Some("threshold_power"),
        46 => Some("pool_length_unit"),
        47 => Some("num_active_lengths"),
        48 => Some("total_work"),
        49 => Some("avg_altitude"),
        50 => Some("max_altitude"),
        51 => Some("gps_accuracy"),
        52 => Some("avg_grade"),
        53 => Some("avg_pos_grade"),
        54 => Some("avg_neg_grade"),
        55 => Some("max_pos_grade"),
        56 => Some("max_neg_grade"),
        57 => Some("avg_temperature"),
        58 => Some("max_temperature"),
        59 => Some("total_moving_time"),
        60 => Some("avg_pos_vertical_speed"),
        61 => Some("avg_neg_vertical_speed"),
        62 => Some("max_pos_vertical_speed"),
        63 => Some("max_neg_vertical_speed"),
        64 => Some("min_heart_rate"),
        65 => Some("time_in_hr_zone"),
        66 => Some("time_in_speed_zone"),
        67 => Some("time_in_cadence_zone"),
        68 => Some("time_in_power_zone"),
        69 => Some("avg_lap_time"),
        70 => Some("best_lap_index"),
        71 => Some("min_altitude"),
        82 => Some("player_score"),
        83 => Some("opponent_score"),
        84 => Some("opponent_name"),
        85 => Some("stroke_count"),
        86 => Some("zone_count"),
        87 => Some("max_ball_speed"),
        88 => Some("avg_ball_speed"),
        89 => Some("avg_vertical_oscillation"),
        90 => Some("avg_stance_time_percent"),
        91 => Some("avg_stance_time"),
        92 => Some("avg_fractional_cadence"),
        93 => Some("max_fractional_cadence"),
        94 => Some("total_fractional_cycles"),
        95 => Some("avg_total_hemoglobin_conc"),
        96 => Some("min_total_hemoglobin_conc"),
        97 => Some("max_total_hemoglobin_conc"),
        98 => Some("avg_saturated_hemoglobin_percent"),
        99 => Some("min_saturated_hemoglobin_percent"),
        100 => Some("max_saturated_hemoglobin_percent"),
        101 => Some("avg_left_torque_effectiveness"),
        102 => Some("avg_right_torque_effectiveness"),
        103 => Some("avg_left_pedal_smoothness"),
        104 => Some("avg_right_pedal_smoothness"),
        105 => Some("avg_combined_pedal_smoothness"),
        111 => Some("sport_index"),
        112 => Some("time_standing"),
        113 => Some("stand_count"),
        114 => Some("avg_left_pco"),
        115 => Some("avg_right_pco"),
        116 => Some("avg_left_power_phase"),
        117 => Some("avg_left_power_phase_peak"),
        118 => Some("avg_right_power_phase"),
        119 => Some("avg_right_power_phase_peak"),
        120 => Some("avg_power_position"),
        121 => Some("max_power_position"),
        122 => Some("avg_cadence_position"),
        123 => Some("max_cadence_position"),
        124 => Some("enhanced_avg_speed"),
        125 => Some("enhanced_max_speed"),
        126 => Some("enhanced_avg_altitude"),
        127 => Some("enhanced_min_altitude"),
        128 => Some("enhanced_max_altitude"),
        129 => Some("avg_lev_motor_power"),
        130 => Some("max_lev_motor_power"),
        131 => Some("lev_battery_consumption"),
        132 => Some("avg_vertical_ratio"),
        133 => Some("avg_stance_time_balance"),
        134 => Some("avg_step_length"),
        137 => Some("total_anaerobic_training_effect"),
        139 => Some("avg_vam"),
        _ => None,
      }
    },
    19 => {
      match field_id {
        254 => Some("message_index"),
        253 => Some("timestamp"),
        0 => Some("event"),
        1 => Some("event_type"),
        2 => Some("start_time"),
        3 => Some("start_position_lat"),
        4 => Some("start_position_long"),
        5 => Some("end_position_lat"),
        6 => Some("end_position_long"),
        7 => Some("total_elapsed_time"),
        8 => Some("total_timer_time"),
        9 => Some("total_distance"),
        10 => Some("total_cycles"),
        11 => Some("total_calories"),
        12 => Some("total_fat_calories"),
        13 => Some("avg_speed"),
        14 => Some("max_speed"),
        15 => Some("avg_heart_rate"),
        16 => Some("max_heart_rate"),
        17 => Some("avg_cadence"),
        18 => Some("max_cadence"),
        19 => Some("avg_power"),
        20 => Some("max_power"),
        21 => Some("total_ascent"),
        22 => Some("total_descent"),
        23 => Some("intensity"),
        24 => Some("lap_trigger"),
        25 => Some("sport"),
        26 => Some("event_group"),
        32 => Some("num_lengths"),
        33 => Some("normalized_power"),
        34 => Some("left_right_balance"),
        35 => Some("first_length_index"),
        37 => Some("avg_stroke_distance"),
        38 => Some("swim_stroke"),
        39 => Some("sub_sport"),
        40 => Some("num_active_lengths"),
        41 => Some("total_work"),
        42 => Some("avg_altitude"),
        43 => Some("max_altitude"),
        44 => Some("gps_accuracy"),
        45 => Some("avg_grade"),
        46 => Some("avg_pos_grade"),
        47 => Some("avg_neg_grade"),
        48 => Some("max_pos_grade"),
        49 => Some("max_neg_grade"),
        50 => Some("avg_temperature"),
        51 => Some("max_temperature"),
        52 => Some("total_moving_time"),
        53 => Some("avg_pos_vertical_speed"),
        54 => Some("avg_neg_vertical_speed"),
        55 => Some("max_pos_vertical_speed"),
        56 => Some("max_neg_vertical_speed"),
        57 => Some("time_in_hr_zone"),
        58 => Some("time_in_speed_zone"),
        59 => Some("time_in_cadence_zone"),
        60 => Some("time_in_power_zone"),
        61 => Some("repetition_num"),
        62 => Some("min_altitude"),
        63 => Some("min_heart_rate"),
        71 => Some("wkt_step_index"),
        74 => Some("opponent_score"),
        75 => Some("stroke_count"),
        76 => Some("zone_count"),
        77 => Some("avg_vertical_oscillation"),
        78 => Some("avg_stance_time_percent"),
        79 => Some("avg_stance_time"),
        80 => Some("avg_fractional_cadence"),
        81 => Some("max_fractional_cadence"),
        82 => Some("total_fractional_cycles"),
        83 => Some("player_score"),
        84 => Some("avg_total_hemoglobin_conc"),
        85 => Some("min_total_hemoglobin_conc"),
        86 => Some("max_total_hemoglobin_conc"),
        87 => Some("avg_saturated_hemoglobin_percent"),
        88 => Some("min_saturated_hemoglobin_percent"),
        89 => Some("max_saturated_hemoglobin_percent"),
        91 => Some("avg_left_torque_effectiveness"),
        92 => Some("avg_right_torque_effectiveness"),
        93 => Some("avg_left_pedal_smoothness"),
        94 => Some("avg_right_pedal_smoothness"),
        95 => Some("avg_combined_pedal_smoothness"),
        98 => Some("time_standing"),
        99 => Some("stand_count"),
        100 => Some("avg_left_pco"),
        101 => Some("avg_right_pco"),
        102 => Some("avg_left_power_phase"),
        103 => Some("avg_left_power_phase_peak"),
        104 => Some("avg_right_power_phase"),
        105 => Some("avg_right_power_phase_peak"),
        106 => Some("avg_power_position"),
        107 => Some("max_power_position"),
        108 => Some("avg_cadence_position"),
        109 => Some("max_cadence_position"),
        110 => Some("enhanced_avg_speed"),
        111 => Some("enhanced_max_speed"),
        112 => Some("enhanced_avg_altitude"),
        113 => Some("enhanced_min_altitude"),
        114 => Some("enhanced_max_altitude"),
        115 => Some("avg_lev_motor_power"),
        116 => Some("max_lev_motor_power"),
        117 => Some("lev_battery_consumption"),
        118 => Some("avg_vertical_ratio"),
        119 => Some("avg_stance_time_balance"),
        120 => Some("avg_step_length"),
        121 => Some("avg_vam"),
        _ => None,
      }
    },
    101 => {
      match field_id {
        254 => Some("message_index"),
        253 => Some("timestamp"),
        0 => Some("event"),
        1 => Some("event_type"),
        2 => Some("start_time"),
        3 => Some("total_elapsed_time"),
        4 => Some("total_timer_time"),
        5 => Some("total_strokes"),
        6 => Some("avg_speed"),
        7 => Some("swim_stroke"),
        9 => Some("avg_swimming_cadence"),
        10 => Some("event_group"),
        11 => Some("total_calories"),
        12 => Some("length_type"),
        18 => Some("player_score"),
        19 => Some("opponent_score"),
        20 => Some("stroke_count"),
        21 => Some("zone_count"),
        _ => None,
      }
    },
    20 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("position_lat"),
        1 => Some("position_long"),
        2 => Some("altitude"),
        3 => Some("heart_rate"),
        4 => Some("cadence"),
        5 => Some("distance"),
        6 => Some("speed"),
        7 => Some("power"),
        8 => Some("compressed_speed_distance"),
        9 => Some("grade"),
        10 => Some("resistance"),
        11 => Some("time_from_course"),
        12 => Some("cycle_length"),
        13 => Some("temperature"),
        17 => Some("speed_1s"),
        18 => Some("cycles"),
        19 => Some("total_cycles"),
        28 => Some("compressed_accumulated_power"),
        29 => Some("accumulated_power"),
        30 => Some("left_right_balance"),
        31 => Some("gps_accuracy"),
        32 => Some("vertical_speed"),
        33 => Some("calories"),
        39 => Some("vertical_oscillation"),
        40 => Some("stance_time_percent"),
        41 => Some("stance_time"),
        42 => Some("activity_type"),
        43 => Some("left_torque_effectiveness"),
        44 => Some("right_torque_effectiveness"),
        45 => Some("left_pedal_smoothness"),
        46 => Some("right_pedal_smoothness"),
        47 => Some("combined_pedal_smoothness"),
        48 => Some("time128"),
        49 => Some("stroke_type"),
        50 => Some("zone"),
        51 => Some("ball_speed"),
        52 => Some("cadence256"),
        53 => Some("fractional_cadence"),
        54 => Some("total_hemoglobin_conc"),
        55 => Some("total_hemoglobin_conc_min"),
        56 => Some("total_hemoglobin_conc_max"),
        57 => Some("saturated_hemoglobin_percent"),
        58 => Some("saturated_hemoglobin_percent_min"),
        59 => Some("saturated_hemoglobin_percent_max"),
        62 => Some("device_index"),
        67 => Some("left_pco"),
        68 => Some("right_pco"),
        69 => Some("left_power_phase"),
        70 => Some("left_power_phase_peak"),
        71 => Some("right_power_phase"),
        72 => Some("right_power_phase_peak"),
        73 => Some("enhanced_speed"),
        78 => Some("enhanced_altitude"),
        81 => Some("battery_soc"),
        82 => Some("motor_power"),
        83 => Some("vertical_ratio"),
        84 => Some("stance_time_balance"),
        85 => Some("step_length"),
        _ => None,
      }
    },
    21 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("event"),
        1 => Some("event_type"),
        2 => Some("data16"),
        3 => Some("data"),
        4 => Some("event_group"),
        7 => Some("score"),
        8 => Some("opponent_score"),
        9 => Some("front_gear_num"),
        10 => Some("front_gear"),
        11 => Some("rear_gear_num"),
        12 => Some("rear_gear"),
        13 => Some("device_index"),
        _ => None,
      }
    },
    23 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("device_index"),
        1 => Some("device_type"),
        2 => Some("manufacturer"),
        3 => Some("serial_number"),
        4 => Some("product"),
        5 => Some("software_version"),
        6 => Some("hardware_version"),
        7 => Some("cum_operating_time"),
        10 => Some("battery_voltage"),
        11 => Some("battery_status"),
        18 => Some("sensor_position"),
        19 => Some("descriptor"),
        20 => Some("ant_transmission_type"),
        21 => Some("ant_device_number"),
        22 => Some("ant_network"),
        25 => Some("source_type"),
        27 => Some("product_name"),
        _ => None,
      }
    },
    72 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("type"),
        1 => Some("manufacturer"),
        2 => Some("product"),
        3 => Some("serial_number"),
        4 => Some("time_created"),
        _ => None,
      }
    },
    78 => {
      match field_id {
        0 => Some("time"),
        _ => None,
      }
    },
    128 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("weather_report"),
        1 => Some("temperature"),
        2 => Some("condition"),
        3 => Some("wind_direction"),
        4 => Some("wind_speed"),
        5 => Some("precipitation_probability"),
        6 => Some("temperature_feels_like"),
        7 => Some("relative_humidity"),
        8 => Some("location"),
        9 => Some("observed_at_time"),
        10 => Some("observed_location_lat"),
        11 => Some("observed_location_long"),
        12 => Some("day_of_week"),
        13 => Some("high_temperature"),
        14 => Some("low_temperature"),
        _ => None,
      }
    },
    129 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("report_id"),
        1 => Some("issue_time"),
        2 => Some("expire_time"),
        3 => Some("severity"),
        4 => Some("type"),
        _ => None,
      }
    },
    160 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("position_lat"),
        2 => Some("position_long"),
        3 => Some("enhanced_altitude"),
        4 => Some("enhanced_speed"),
        5 => Some("heading"),
        6 => Some("utc_timestamp"),
        7 => Some("velocity"),
        _ => None,
      }
    },
    161 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("camera_event_type"),
        2 => Some("camera_file_uuid"),
        3 => Some("camera_orientation"),
        _ => None,
      }
    },
    164 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("sample_time_offset"),
        2 => Some("gyro_x"),
        3 => Some("gyro_y"),
        4 => Some("gyro_z"),
        5 => Some("calibrated_gyro_x"),
        6 => Some("calibrated_gyro_y"),
        7 => Some("calibrated_gyro_z"),
        _ => None,
      }
    },
    165 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("sample_time_offset"),
        2 => Some("accel_x"),
        3 => Some("accel_y"),
        4 => Some("accel_z"),
        5 => Some("calibrated_accel_x"),
        6 => Some("calibrated_accel_y"),
        7 => Some("calibrated_accel_z"),
        8 => Some("compressed_calibrated_accel_x"),
        9 => Some("compressed_calibrated_accel_y"),
        10 => Some("compressed_calibrated_accel_z"),
        _ => None,
      }
    },
    208 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("sample_time_offset"),
        2 => Some("mag_x"),
        3 => Some("mag_y"),
        4 => Some("mag_z"),
        5 => Some("calibrated_mag_x"),
        6 => Some("calibrated_mag_y"),
        7 => Some("calibrated_mag_z"),
        _ => None,
      }
    },
    209 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("sample_time_offset"),
        2 => Some("baro_pres"),
        _ => None,
      }
    },
    167 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("sensor_type"),
        1 => Some("calibration_factor"),
        2 => Some("calibration_divisor"),
        3 => Some("level_shift"),
        4 => Some("offset_cal"),
        5 => Some("orientation_matrix"),
        _ => None,
      }
    },
    210 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("sensor_type"),
        1 => Some("calibration_factor"),
        2 => Some("calibration_divisor"),
        3 => Some("level_shift"),
        4 => Some("offset_cal"),
        _ => None,
      }
    },
    169 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("frame_number"),
        _ => None,
      }
    },
    174 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("time_offset"),
        2 => Some("pid"),
        3 => Some("raw_data"),
        4 => Some("pid_data_size"),
        5 => Some("system_time"),
        6 => Some("start_timestamp"),
        7 => Some("start_timestamp_ms"),
        _ => None,
      }
    },
    177 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("sentence"),
        _ => None,
      }
    },
    178 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("timestamp_ms"),
        1 => Some("system_time"),
        2 => Some("pitch"),
        3 => Some("roll"),
        4 => Some("accel_lateral"),
        5 => Some("accel_normal"),
        6 => Some("turn_rate"),
        7 => Some("stage"),
        8 => Some("attitude_stage_complete"),
        9 => Some("track"),
        10 => Some("validity"),
        _ => None,
      }
    },
    184 => {
      match field_id {
        0 => Some("url"),
        1 => Some("hosting_provider"),
        2 => Some("duration"),
        _ => None,
      }
    },
    185 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("message_count"),
        1 => Some("text"),
        _ => None,
      }
    },
    186 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("message_count"),
        1 => Some("text"),
        _ => None,
      }
    },
    187 => {
      match field_id {
        0 => Some("clip_number"),
        1 => Some("start_timestamp"),
        2 => Some("start_timestamp_ms"),
        3 => Some("end_timestamp"),
        4 => Some("end_timestamp_ms"),
        6 => Some("clip_start"),
        7 => Some("clip_end"),
        _ => None,
      }
    },
    31 => {
      match field_id {
        4 => Some("sport"),
        5 => Some("name"),
        6 => Some("capabilities"),
        7 => Some("sub_sport"),
        _ => None,
      }
    },
    32 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("timestamp"),
        2 => Some("position_lat"),
        3 => Some("position_long"),
        4 => Some("distance"),
        5 => Some("type"),
        6 => Some("name"),
        8 => Some("favorite"),
        _ => None,
      }
    },
    148 => {
      match field_id {
        0 => Some("name"),
        1 => Some("uuid"),
        2 => Some("sport"),
        3 => Some("enabled"),
        4 => Some("user_profile_primary_key"),
        5 => Some("device_id"),
        6 => Some("default_race_leader"),
        7 => Some("delete_status"),
        8 => Some("selection_type"),
        _ => None,
      }
    },
    149 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("name"),
        1 => Some("type"),
        2 => Some("group_primary_key"),
        3 => Some("activity_id"),
        4 => Some("segment_time"),
        5 => Some("activity_id_string"),
        _ => None,
      }
    },
    150 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("position_lat"),
        2 => Some("position_long"),
        3 => Some("distance"),
        4 => Some("altitude"),
        5 => Some("leader_time"),
        _ => None,
      }
    },
    142 => {
      match field_id {
        254 => Some("message_index"),
        253 => Some("timestamp"),
        0 => Some("event"),
        1 => Some("event_type"),
        2 => Some("start_time"),
        3 => Some("start_position_lat"),
        4 => Some("start_position_long"),
        5 => Some("end_position_lat"),
        6 => Some("end_position_long"),
        7 => Some("total_elapsed_time"),
        8 => Some("total_timer_time"),
        9 => Some("total_distance"),
        10 => Some("total_cycles"),
        11 => Some("total_calories"),
        12 => Some("total_fat_calories"),
        13 => Some("avg_speed"),
        14 => Some("max_speed"),
        15 => Some("avg_heart_rate"),
        16 => Some("max_heart_rate"),
        17 => Some("avg_cadence"),
        18 => Some("max_cadence"),
        19 => Some("avg_power"),
        20 => Some("max_power"),
        21 => Some("total_ascent"),
        22 => Some("total_descent"),
        23 => Some("sport"),
        24 => Some("event_group"),
        25 => Some("nec_lat"),
        26 => Some("nec_long"),
        27 => Some("swc_lat"),
        28 => Some("swc_long"),
        29 => Some("name"),
        30 => Some("normalized_power"),
        31 => Some("left_right_balance"),
        32 => Some("sub_sport"),
        33 => Some("total_work"),
        34 => Some("avg_altitude"),
        35 => Some("max_altitude"),
        36 => Some("gps_accuracy"),
        37 => Some("avg_grade"),
        38 => Some("avg_pos_grade"),
        39 => Some("avg_neg_grade"),
        40 => Some("max_pos_grade"),
        41 => Some("max_neg_grade"),
        42 => Some("avg_temperature"),
        43 => Some("max_temperature"),
        44 => Some("total_moving_time"),
        45 => Some("avg_pos_vertical_speed"),
        46 => Some("avg_neg_vertical_speed"),
        47 => Some("max_pos_vertical_speed"),
        48 => Some("max_neg_vertical_speed"),
        49 => Some("time_in_hr_zone"),
        50 => Some("time_in_speed_zone"),
        51 => Some("time_in_cadence_zone"),
        52 => Some("time_in_power_zone"),
        53 => Some("repetition_num"),
        54 => Some("min_altitude"),
        55 => Some("min_heart_rate"),
        56 => Some("active_time"),
        57 => Some("wkt_step_index"),
        58 => Some("sport_event"),
        59 => Some("avg_left_torque_effectiveness"),
        60 => Some("avg_right_torque_effectiveness"),
        61 => Some("avg_left_pedal_smoothness"),
        62 => Some("avg_right_pedal_smoothness"),
        63 => Some("avg_combined_pedal_smoothness"),
        64 => Some("status"),
        65 => Some("uuid"),
        66 => Some("avg_fractional_cadence"),
        67 => Some("max_fractional_cadence"),
        68 => Some("total_fractional_cycles"),
        69 => Some("front_gear_shift_count"),
        70 => Some("rear_gear_shift_count"),
        71 => Some("time_standing"),
        72 => Some("stand_count"),
        73 => Some("avg_left_pco"),
        74 => Some("avg_right_pco"),
        75 => Some("avg_left_power_phase"),
        76 => Some("avg_left_power_phase_peak"),
        77 => Some("avg_right_power_phase"),
        78 => Some("avg_right_power_phase_peak"),
        79 => Some("avg_power_position"),
        80 => Some("max_power_position"),
        81 => Some("avg_cadence_position"),
        82 => Some("max_cadence_position"),
        83 => Some("manufacturer"),
        _ => None,
      }
    },
    151 => {
      match field_id {
        254 => Some("message_index"),
        1 => Some("file_uuid"),
        3 => Some("enabled"),
        4 => Some("user_profile_primary_key"),
        7 => Some("leader_type"),
        8 => Some("leader_group_primary_key"),
        9 => Some("leader_activity_id"),
        10 => Some("leader_activity_id_string"),
        11 => Some("default_race_leader"),
        _ => None,
      }
    },
    26 => {
      match field_id {
        4 => Some("sport"),
        5 => Some("capabilities"),
        6 => Some("num_valid_steps"),
        8 => Some("wkt_name"),
        11 => Some("sub_sport"),
        14 => Some("pool_length"),
        15 => Some("pool_length_unit"),
        _ => None,
      }
    },
    158 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("sport"),
        1 => Some("sub_sport"),
        2 => Some("num_valid_steps"),
        3 => Some("first_step_index"),
        4 => Some("pool_length"),
        5 => Some("pool_length_unit"),
        _ => None,
      }
    },
    27 => {
      match field_id {
        254 => Some("message_index"),
        0 => Some("wkt_step_name"),
        1 => Some("duration_type"),
        2 => Some("duration_value"),
        3 => Some("target_type"),
        4 => Some("target_value"),
        5 => Some("custom_target_value_low"),
        6 => Some("custom_target_value_high"),
        7 => Some("intensity"),
        8 => Some("notes"),
        9 => Some("equipment"),
        _ => None,
      }
    },
    28 => {
      match field_id {
        0 => Some("manufacturer"),
        1 => Some("product"),
        2 => Some("serial_number"),
        3 => Some("time_created"),
        4 => Some("completed"),
        5 => Some("type"),
        6 => Some("scheduled_time"),
        _ => None,
      }
    },
    33 => {
      match field_id {
        254 => Some("message_index"),
        253 => Some("timestamp"),
        0 => Some("timer_time"),
        1 => Some("distance"),
        2 => Some("calories"),
        3 => Some("sport"),
        4 => Some("elapsed_time"),
        5 => Some("sessions"),
        6 => Some("active_time"),
        9 => Some("sport_index"),
        _ => None,
      }
    },
    30 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("weight"),
        1 => Some("percent_fat"),
        2 => Some("percent_hydration"),
        3 => Some("visceral_fat_mass"),
        4 => Some("bone_mass"),
        5 => Some("muscle_mass"),
        7 => Some("basal_met"),
        8 => Some("physique_rating"),
        9 => Some("active_met"),
        10 => Some("metabolic_age"),
        11 => Some("visceral_fat_rating"),
        12 => Some("user_profile_index"),
        _ => None,
      }
    },
    51 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("systolic_pressure"),
        1 => Some("diastolic_pressure"),
        2 => Some("mean_arterial_pressure"),
        3 => Some("map_3_sample_mean"),
        4 => Some("map_morning_values"),
        5 => Some("map_evening_values"),
        6 => Some("heart_rate"),
        7 => Some("heart_rate_type"),
        8 => Some("status"),
        9 => Some("user_profile_index"),
        _ => None,
      }
    },
    103 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("local_timestamp"),
        1 => Some("activity_type"),
        3 => Some("cycles_to_distance"),
        4 => Some("cycles_to_calories"),
        5 => Some("resting_metabolic_rate"),
        _ => None,
      }
    },
    55 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("device_index"),
        1 => Some("calories"),
        2 => Some("distance"),
        3 => Some("cycles"),
        4 => Some("active_time"),
        5 => Some("activity_type"),
        6 => Some("activity_subtype"),
        7 => Some("activity_level"),
        8 => Some("distance_16"),
        9 => Some("cycles_16"),
        10 => Some("active_time_16"),
        11 => Some("local_timestamp"),
        12 => Some("temperature"),
        14 => Some("temperature_min"),
        15 => Some("temperature_max"),
        16 => Some("activity_time"),
        19 => Some("active_calories"),
        24 => Some("current_activity_type_intensity"),
        25 => Some("timestamp_min_8"),
        26 => Some("timestamp_16"),
        27 => Some("heart_rate"),
        28 => Some("intensity"),
        29 => Some("duration_min"),
        30 => Some("duration"),
        31 => Some("ascent"),
        32 => Some("descent"),
        33 => Some("moderate_activity_minutes"),
        34 => Some("vigorous_activity_minutes"),
        _ => None,
      }
    },
    132 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("fractional_timestamp"),
        1 => Some("time256"),
        6 => Some("filtered_bpm"),
        9 => Some("event_timestamp"),
        10 => Some("event_timestamp_12"),
        _ => None,
      }
    },
    227 => {
      match field_id {
        0 => Some("stress_level_value"),
        1 => Some("stress_level_time"),
        _ => None,
      }
    },
    145 => {
      match field_id {
        250 => Some("part_index"),
        0 => Some("memo"),
        1 => Some("message_number"),
        2 => Some("message_index"),
        _ => None,
      }
    },
    82 => {
      match field_id {
        0 => Some("channel_number"),
        1 => Some("device_type"),
        2 => Some("device_number"),
        3 => Some("transmission_type"),
        4 => Some("device_index"),
        _ => None,
      }
    },
    80 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("fractional_timestamp"),
        1 => Some("mesg_id"),
        2 => Some("mesg_data"),
        3 => Some("channel_number"),
        4 => Some("data"),
        _ => None,
      }
    },
    81 => {
      match field_id {
        253 => Some("timestamp"),
        0 => Some("fractional_timestamp"),
        1 => Some("mesg_id"),
        2 => Some("mesg_data"),
        3 => Some("channel_number"),
        4 => Some("data"),
        _ => None,
      }
    },
    200 => {
      match field_id {
        0 => Some("screen_index"),
        1 => Some("field_count"),
        2 => Some("layout"),
        3 => Some("screen_enabled"),
        _ => None,
      }
    },
    201 => {
      match field_id {
        0 => Some("screen_index"),
        1 => Some("concept_field"),
        2 => Some("field_id"),
        3 => Some("concept_count"),
        4 => Some("display_type"),
        5 => Some("title"),
        _ => None,
      }
    },
    202 => {
      match field_id {
        0 => Some("screen_index"),
        1 => Some("concept_field"),
        2 => Some("field_id"),
        3 => Some("concept_index"),
        4 => Some("data_page"),
        5 => Some("concept_key"),
        6 => Some("scaling"),
        8 => Some("data_units"),
        9 => Some("qualifier"),
        10 => Some("descriptor"),
        11 => Some("is_signed"),
        _ => None,
      }
    },
    206 => {
      match field_id {
        0 => Some("developer_data_index"),
        1 => Some("field_definition_number"),
        2 => Some("fit_base_type_id"),
        3 => Some("field_name"),
        4 => Some("array"),
        5 => Some("components"),
        6 => Some("scale"),
        7 => Some("offset"),
        8 => Some("units"),
        9 => Some("bits"),
        10 => Some("accumulate"),
        13 => Some("fit_base_unit_id"),
        14 => Some("native_mesg_num"),
        15 => Some("native_field_num"),
        _ => None,
      }
    },
    _ => None,
  }
}

