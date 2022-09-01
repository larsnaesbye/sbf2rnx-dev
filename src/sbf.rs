//! Record holding the extracted information from an SBF file, arranged

#[derive(Clone, Debug)]
pub struct SbfBlockHeader {
    sync: [char; 2], // NOTE: always set to 0x24, 0x40 = "$@"
    crc: u16,
    id: u16,
    length: u16,
}

#[derive(Clone, Debug)]
pub struct SbfBlockTimestamp {
    tow: u32,
    wnc: u16,
}

#[derive(Clone, Debug)]
pub struct SbfBlock {
    header: SbfBlockHeader,
    timestamp: SbfBlockTimestamp,
}

// ====== SBF BLOCK DATA ======

// --- Measurement Blocks ---
const MEAS_EPOCH: i16 = 4027; // Measurement set of one epoch
const MEAS_EXTRA: i16 = 4000; // Additional info such as observable variance
const MEAS3RANGES: i16 = 4109; // Code, phase and CN0 measurements
const MEAS3CN0HI_RES: i16 = 4110; // Extension of MEAS3RANGES containing fractional C/N0 values
const MEAS3DOPPLER: i16 = 4111; // Extension of MEAS3RANGES containing Doppler values
const MEAS3PP: i16 = 4112; // Extension of MEAS3RANGES containing proprietary flags for data post-processing
const MEAS3MP: i16 = 4113; // Extension of MEAS3RANGES containing multipath corrections applied by the receiver.
const END_OF_MEAS: i16 = 5922; // Measurement epoch marker

// --- Navigation Page Blocks
const GPSRAW_CA: i16 = 4017;
const GPSRAW_L2C: i16 = 4018;
const GPSRAW_L5: i16 = 4019;
const GPSRAW_L1C: i16 = 4221;
const GLORAW_CA: i16 = 4026;
const GALRAW_FNAV: i16 = 4022;
const GALRAW_INAV: i16 = 4023;
const GALRAW_CNAV: i16 = 4024;
const GEORAW_L1: i16 = 4020;
const GEORAW_L5: i16 = 4021;
const BDSRAW: i16 = 4047;
const BDSRAW_B1C: i16 = 4218;
const BDSRAW_B2A: i16 = 4219;
const BDSRAW_B2B: i16 = 4242;
const QZSRAW_L1CA: i16 = 4066;
const QZSRAW_L2C: i16 = 4067;
const QZSRAW_L5: i16 = 4068;
const QZSRAW_L6: i16 = 4069;
const QZSRAW_L1C: i16 = 4227;
const QZSRAW_L1S: i16 = 4228;
const NAVICRAW: i16 = 4093;

// GPS Decoded Message blocks
const GPSNAV: i16 = 5891;
const GPSALM: i16 = 5892;
const GPSION: i16 = 5893;
const GPSUTC: i16 = 5894;

// GLONASS Decoded Message blocks
const GLONAV: i16 = 4004;
const GLOALM: i16 = 4005;
const GLOTIME: i16 = 4036;

// Galileo Decoded Message blocks
const GALNAV: i16 = 4002;
const GALALM: i16 = 4003;
const GALION: i16 = 4030;
const GALUTC: i16 = 4031;
const GALGST_GPS: i16 = 4032;
const GALSARRLM: i16 = 4034;

// BeiDou Decoded Message blocks
const BDSNAV: i16 = 4081;
const BDSALM: i16 = 4119;
const BDSION: i16 = 4120;
const BDSUTC: i16 = 4121;

// QZSS Decoded Message blocks
const QZSNAV: i16 = 4095;
const QZSALM: i16 = 4116;

// SBAS L1 Decoded Message blocks
const GEOMT00: i16 = 5925;
const GEOPRNMASK: i16 = 5926;
const GEOFAST_CORR: i16 = 5927;
const GEOINTEGRITY: i16 = 5928;
const GEOFAST_CORR_DEGR: i16 = 5929;
const GEONAV: i16 = 5896;
const GEODEGR_FACTORS: i16 = 5930;
const GEONETWORK_TIME: i16 = 5918;
const GEOALM: i16 = 5897;
const GEOIGPMASK: i16 = 5931;
const GEOLONG_TERM_CORR: i16 = 5932;
const GEOIONO_DELAY: i16 = 5933;
const GEOSERVICE_LEVEL: i16 = 5917;
const GEOCLOCK_EPHCOV_MATRIX: i16 = 5934;

// Position, Velocity and Time Blocks
const PVICARTESIAN: i16 = 4006;
const PVIGEODETIC: i16 = 4007;
const POS_COV_CARTESIAN: i16 = 5905;
const POS_COV_GEODETIC: i16 = 5906;
const VEL_COV_CARTESIAN: i16 = 5907;
const VEL_COV_GEODETIC: i16 = 5908;
const DOP: i16 = 4001;
const POS_CART: i16 = 4044;
const POS_LOCAL: i16 = 4052;
const POS_PROJECTED: i16 = 4094;
const PVISAT_CARTESIAN: i16 = 4008;
const PVIRESIDUALS: i16 = 4009;
const RAIMSTATISTICS: i16 = 4011;
const GEOCORRECTIONS: i16 = 5935;
const BASE_VECTOR_CART: i16 = 4043;
const BASE_VECTOR_GEOD: i16 = 4028;
const FVISUPPORT: i16 = 4076;
const FVISUPPORT_A: i16 = 4079;
const END_OF_PVI: i16 = 5921;

// Receiver Time Blocks
const RECEIVER_TIME: i16 = 5914;
const X_PPSOFFSET: i16 = 5911;

// External Event Blocks
const EXT_EVENT: i16 = 5924;
const EXT_EVENT_PVICARTESIAN: i16 = 4037;
const EXT_EVENT_PVIGEODETIC: i16 = 4038;
const EXT_EVENT_BASE_VECT_GEOD: i16 = 4217;

// Differential Correction Blocks
const DIFF_CORR_IN: i16 = 5919;
const BASE_STATION: i16 = 5949;
const RICMDATUM: i16 = 4049;

// L-Band Demodulator Blocks
const LBAND_TRACKER_STATUS: i16 = 4201;
const LBAS1DECODER_STATUS: i16 = 4202;
const LBAS1MESSAGES: i16 = 4203;
const LBAND_BEAMS: i16 = 4204;

// Status Blocks
const CHANNEL_STATUS: i16 = 4013;
const RECEIVER_STATUS: i16 = 4014;
const SAT_VISIBILITY: i16 = 4012;
const INPUT_LINK: i16 = 4090;
const OUTPUT_LINK: i16 = 4091;
const NTRIPCLIENT_STATUS: i16 = 4053;
const NTRIPSERVER_STATUS: i16 = 4122;
const IPSTATUS: i16 = 4058;
const WI_FI_APSTATUS: i16 = 4054;
const WI_FI_CLIENT_STATUS: i16 = 4096;
const DYN_DNSSTATUS: i16 = 4105;
const POWER_STATUS: i16 = 4101;
const QUALITY_IND: i16 = 4082;
const DISK_STATUS: i16 = 4059;
const LOG_STATUS: i16 = 4102;
const RFSTATUS: i16 = 4092;
const P2PPSTATUS: i16 = 4238;
const COSMOS_STATUS: i16 = 4243;

// Miscellaneous Blocks
const RECEIVER_SETUP: i16 = 5902;
const RX_COMPONENTS: i16 = 4084;
const RX_MESSAGE: i16 = 4103;
const COMMANDS: i16 = 4015;
const COMMENT: i16 = 5936;
const BBSAMPLES: i16 = 4040;
const ASCIIIN: i16 = 4075;

// Advanced Blocks
const SYSTEM_INFO: i16 = 6000;

// --- Decoding functions ---

fn decode_measepoch(measepochdata: u8) -> u8 {
    return measepochdata; // Placeholder
}

fn decode_gpsnav(gpsnavdata: u8) -> u8 {
    return gpsnavdata; // Placeholder
}

fn decode_galnav(galnavdata: u8) -> u8 {
    return galnavdata; // Placeholder
}

fn decode_glonav(glonavdata: u8) -> u8 {
    return glonavdata; // Placeholder
}

fn decode_sbasnav(sbasnavdata: u8) -> u8 {
    return sbasnavdata; // Placeholder
}

fn decode_gpsrawcanav(gpsrawcanavdata: u8) -> u8 {
    return gpsrawcanavdata; // Placeholder
}

fn decode_georaw(georawdata: u8) -> u8 {
    return georawdata; // Placeholder
}

fn decode_glorawcanav(glorawcanavdata: u8) -> u8 {
    return glorawcanavdata; // Placeholder
}

fn decode_gpsion(gpsiondata: u8) -> u8 {
    return gpsiondata; // Placeholder
}

fn decode_galion(galiondata: u8) -> u8 {
    return galiondata; // Placeholder
}

fn decode_gpsutc(gpsutcdata: u8) -> u8 {
    return gpsutcdata; // Placeholder
}

fn decode_gpsalm(gpsalmdata: u8) -> u8 {
    return gpsalmdata; // Placeholder
}

fn decode_galutc(galutcdata: u8) -> u8 {
    return galutcdata; // Placeholder
}

fn decode_galalm(galalmdata: u8) -> u8 {
    return galalmdata; // Placeholder
}

fn decode_sbf(sbfdata: u8) -> u8 {
    return sbfdata; // Placeholder
}
