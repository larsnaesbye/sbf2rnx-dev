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
const MEAS3CN0HI_RES: i16 = 4110; // Extension of Meas3Ranges containing fractional C/N0 values
const MEAS3DOPPLER: i16 = 4111; // Extension of Meas3Ranges containing Doppler values
const MEAS3PP: i16 = 4112; // Extension of Meas3Ranges containing proprietary flags for data post-processing
const MEAS3MP: i16 = 4113; // Extension of Meas3Ranges containing multipath corrections applied by the receiver.
const END_OF_MEAS: i16 = 5922; // Measurement epoch marker

// --- Navigation Page Blocks
const GPSRAW_CA: i16 = 4017; // GPS CA navigation subframe
const GPSRAW_L2C: i16 = 4018; // GPS L2C navigation frame
const GPSRAW_L5: i16 = 4019; // GPS L5 navigation frame
const GPSRAW_L1C: i16 = 4221; // GPS L1C navigation frame
const GLORAW_CA: i16 = 4026; // GLONASS CA navigation string
const GALRAW_FNAV: i16 = 4022; // Galileo F/NAV navigation page
const GALRAW_INAV: i16 = 4023; // Galileo I/NAV navigation page
const GALRAW_CNAV: i16 = 4024; // Galileo C/NAV navigation page
const GEORAW_L1: i16 = 4020; // SBAS L1 navigation message
const GEORAW_L5: i16 = 4021; // SBAS L5 navigation message
const BDSRAW: i16 = 4047; // BeiDou navigation page
const BDSRAW_B1C: i16 = 4218; // BeiDou B1C navigation frame
const BDSRAW_B2A: i16 = 4219; // BeiDou B2a navigation frame
const BDSRAW_B2B: i16 = 4242; // BeiDou B2b navigation frame
const QZSRAW_L1CA: i16 = 4066; // QZSS L1 CA navigation frame
const QZSRAW_L2C: i16 = 4067; // QZSS L2C navigation frame
const QZSRAW_L5: i16 = 4068; // QZSS L5 navigation frame
const QZSRAW_L6: i16 = 4069; // QZSS L6 navigation message
const QZSRAW_L1C: i16 = 4227; // QZSS L1C navigation frame
const QZSRAW_L1S: i16 = 4228; // QZSS L1S navigation message
const NAVICRAW: i16 = 4093; // NavIC/IRNSS subframe

// GPS Decoded Message blocks
const GPSNAV: i16 = 5891; // GPS ephemeris and clock
const GPSALM: i16 = 5892; // Almanac data for a GPS satellite
const GPSION: i16 = 5893; // Ionosphere data from the GPS subframe 5
const GPSUTC: i16 = 5894; // GPS-UTC data from GPS subframe 5

// GLONASS Decoded Message blocks
const GLONAV: i16 = 4004; // GLONASS ephemeris and clock
const GLOALM: i16 = 4005; // Almanac data for a GLONASS satellite
const GLOTIME: i16 = 4036; // GLO-UTC, GLO-GPS and GLO-UT1 data

// Galileo Decoded Message blocks
const GALNAV: i16 = 4002; // Galileo ephemeris, clock, health and BGD
const GALALM: i16 = 4003; // Almanac data for a Galileo satellite
const GALION: i16 = 4030; // NeQuick Ionosphere model parameters
const GALUTC: i16 = 4031; // GST-UTC data
const GALGST_GPS: i16 = 4032; // GST-GPS data
const GALSARRLM: i16 = 4034; // Search-and-rescue return link message

// BeiDou Decoded Message blocks
const BDSNAV: i16 = 4081; // BeiDou ephemeris and clock
const BDSALM: i16 = 4119; // Almanac data for a BeiDou satellite
const BDSION: i16 = 4120; // BeiDou Ionospheric delay model parameters
const BDSUTC: i16 = 4121; // BDT-UTC data

// QZSS Decoded Message blocks
const QZSNAV: i16 = 4095; // QZSS ephemeris and clock
const QZSALM: i16 = 4116; // Almanac data for a QZSS satellite

// SBAS L1 Decoded Message blocks
const GEOMT00: i16 = 5925; // MT00 : SBAS Don’t use for safety applications
const GEOPRNMASK: i16 = 5926; // MT01 : PRN Mask assignments
const GEOFAST_CORR: i16 = 5927; // MT02-05/24: Fast Corrections
const GEOINTEGRITY: i16 = 5928; // MT06 : Integrity information
const GEOFAST_CORR_DEGR: i16 = 5929; // MT07 : Fast correction degradation factors
const GEONAV: i16 = 5896; // MT09 : SBAS navigation message
const GEODEGR_FACTORS: i16 = 5930; // MT10 : Degradation factors
const GEONETWORK_TIME: i16 = 5918; // MT12 : SBAS Network Time/UTC offset parameters
const GEOALM: i16 = 5897; // MT17 : SBAS satellite almanac
const GEOIGPMASK: i16 = 5931; // MT18 : Ionospheric grid point mask
const GEOLONG_TERM_CORR: i16 = 5932; // MT24/25 : Long term satellite error corrections
const GEOIONO_DELAY: i16 = 5933; // MT26 : Ionospheric delay corrections
const GEOSERVICE_LEVEL: i16 = 5917; // MT27 : SBAS Service Message
const GEOCLOCK_EPHCOV_MATRIX: i16 = 5934; // MT28 : Clock-Ephemeris Covariance Matrix

// Position, Velocity and Time Blocks
const PVTCARTESIAN: i16 = 4006; // Position, velocity, and time in Cartesian coordinates
const PVTGEODETIC: i16 = 4007; // Position, velocity, and time in geodetic coordinates
const POS_COV_CARTESIAN: i16 = 5905; // Position covariance matrix (X,Y, Z)
const POS_COV_GEODETIC: i16 = 5906; // Position covariance matrix (Lat, Lon, Alt)
const VEL_COV_CARTESIAN: i16 = 5907; // Velocity covariance matrix (X, Y, Z)
const VEL_COV_GEODETIC: i16 = 5908; // Velocity covariance matrix (North, East, Up)
const DOP: i16 = 4001; // Dilution of precision
const POS_CART: i16 = 4044; // Position, variance and baseline in Cartesian coordinates
const POS_LOCAL: i16 = 4052; // Position in a local datum
const POS_PROJECTED: i16 = 4094; // Plane grid coordinates
const PVTSAT_CARTESIAN: i16 = 4008; // Satellite positions
const PVTRESIDUALS: i16 = 4009; // Measurement residuals
const RAIMSTATISTICS: i16 = 4011; // Integrity statistics
const GEOCORRECTIONS: i16 = 5935; // Orbit, Clock and pseudoranges SBAS corrections
const BASE_VECTOR_CART: i16 = 4043; // XYZ relative position and velocity with respect to base(s)
const BASE_VECTOR_GEOD: i16 = 4028; // ENU relative position and velocity with respect to base(s)
const FVISUPPORT: i16 = 4076; // Internal parameters for maintenance and support
const FVISUPPORT_A: i16 = 4079; // Internal parameters for maintenance and support
const END_OF_PVT: i16 = 5921; // PVT epoch marker

// Receiver Time Blocks
const RECEIVER_TIME: i16 = 5914; // Current receiver and UTC time
const X_PPSOFFSET: i16 = 5911; // Offset of the xPPS pulse with respect to GNSS time

// External Event Blocks
const EXT_EVENT: i16 = 5924; // Time at the instant of an external event
const EXT_EVENT_PVTCARTESIAN: i16 = 4037; // Cartesian position at the instant of an event
const EXT_EVENT_PVTGEODETIC: i16 = 4038; // Geodetic position at the instant of an event
const EXT_EVENT_BASE_VECT_GEOD: i16 = 4217; // ENU relative position with respect to base(s) at the instant of an event

// Differential Correction Blocks
const DIFF_CORR_IN: i16 = 5919; // Incoming RTCM or CMR message
const BASE_STATION: i16 = 5949; // Base station coordinates
const RICMDATUM: i16 = 4049; // Datum information from the RTK service provider

// L-Band Demodulator Blocks
const LBAND_TRACKER_STATUS: i16 = 4201; // Status of the L-band signal tracking
const LBAS1DECODER_STATUS: i16 = 4202; // Status of the LBAS1 L-band service
const LBAS1MESSAGES: i16 = 4203; // LBAS1over-the-air message
const LBAND_BEAMS: i16 = 4204; // L-band satellite/beam information

// Status Blocks
const CHANNEL_STATUS: i16 = 4013; // Status of the tracking for all receiver channels
const RECEIVER_STATUS: i16 = 4014; // Overall status information of the receiver
const SAT_VISIBILITY: i16 = 4012; // Azimuth/elevation of visible satellites
const INPUT_LINK: i16 = 4090; // Statistics on input streams
const OUTPUT_LINK: i16 = 4091; // Statistics on output streams
const NTRIPCLIENT_STATUS: i16 = 4053; // NTRIP client connection status
const NTRIPSERVER_STATUS: i16 = 4122; // NTRIP server connection status
const IPSTATUS: i16 = 4058; // IP address, gateway and MAC address of Ethernet interface
const WI_FI_APSTATUS: i16 = 4054; // WiFi status in access point mode
const WI_FI_CLIENT_STATUS: i16 = 4096; // WiFi status in client mode
const DYN_DNSSTATUS: i16 = 4105; // DynDNS status
const POWER_STATUS: i16 = 4101; // Power supply source and voltage
const QUALITY_IND: i16 = 4082; // Quality indicators
const DISK_STATUS: i16 = 4059; // Internal logging status
const LOG_STATUS: i16 = 4102; // Log sessions status
const RFSTATUS: i16 = 4092; // Radio-frequency interference mitigation status
const P2PPSTATUS: i16 = 4238; // P2PP client/server status
const COSMOS_STATUS: i16 = 4243; // Cosmos receiver service status

// Miscellaneous Blocks
const RECEIVER_SETUP: i16 = 5902; // General information about the receiver installation
const RX_COMPONENTS: i16 = 4084;
const RX_MESSAGE: i16 = 4103; // Receiver message
const COMMANDS: i16 = 4015; // Commands entered by the user
const COMMENT: i16 = 5936; // Comment entered by the user
const BBSAMPLES: i16 = 4040; // Baseband samples
const ASCIIIN: i16 = 4075; // ASCII input from external sensor

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
