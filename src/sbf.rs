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

enum SBFBlocks {
    // --- Measurement Blocks ---
    MeasEpoch = 4027,     // Measurement set of one epoch
    MeasExtra = 4000,     // Additional info such as observable variance
    Meas3Ranges = 4109,   // Code, phase and CN0 measurements
    Meas3CN0HiRes = 4110, // Extension of Meas3Ranges containing fractional C/N0 values
    Meas3Doppler = 4111,  // Extension of Meas3Ranges containing Doppler values
    Meas3PP = 4112, // Extension of Meas3Ranges containing proprietary flags for data post-processing
    Meas3MP = 4113, // Extension of Meas3Ranges containing multipath corrections applied by the receiver.
    EndOfMeas = 5922, // Measurement epoch marker

    // --- Navigation Page Blocks
    GPSRawCA = 4017,
    GPSRawL2C = 4018,
    GPSRawL5 = 4019,
    GPSRawL1C = 4221,
    GLORawCA = 4026,
    GALRawFNAV = 4022,
    GALRawINAV = 4023,
    GALRawCNAV = 4024,
    GEORawL1 = 4020,
    GEORawL5 = 4021,
    BDSRaw = 4047,
    BDSRawB1C = 4218,
    BDSRawB2a = 4219,
    BDSRawB2b = 4242,
    QZSRawL1CA = 4066,
    QZSRawL2C = 4067,
    QZSRawL5 = 4068,
    QZSRawL6 = 4069,
    QZSRawL1C = 4227,
    QZSRawL1S = 4228,
    NAVICRaw = 4093,

    // GPS Decoded Message blocks
    GPSNav = 5891,
    GPSAlm = 5892,
    GPSIon = 5893,
    GPSUtc = 5894,

    // GLONASS Decoded Message blocks
    GLONav = 4004,
    GLOAlm = 4005,
    GLOTime = 4036,

    // Galileo Decoded Message blocks
    GALNav = 4002,
    GALAlm = 4003,
    GALIon = 4030,
    GALUtc = 4031,
    GALGstGps = 4032,
    GALSARRLM = 4034,

    // BeiDou Decoded Message blocks
    BDSNav = 4081,
    BDSAlm = 4119,
    BDSIon = 4120,
    BDSUtc = 4121,

    // QZSS Decoded Message blocks
    QZSNav = 4095,
    QZSAlm = 4116,

    // SBAS L1 Decoded Message blocks
    GEOMT00 = 5925,
    GEOPRNMask = 5926,
    GEOFastCorr = 5927,
    GEOIntegrity = 5928,
    GEOFastCorrDegr = 5929,
    GEONav = 5896,
    GEODegrFactors = 5930,
    GEONetworkTime = 5918,
    GEOAlm = 5897,
    GEOIGPMask = 5931,
    GEOLongTermCorr = 5932,
    GEOIonoDelay = 5933,
    GEOServiceLevel = 5917,
    GEOClockEphcovMatrix = 5934,

    // Position, Velocity and Time Blocks
    PVICartesian = 4006,
    PVIGeodetic = 4007,
    PosCovCartesian = 5905,
    PosCovGeodetic = 5906,
    VelCovCartesian = 5907,
    VelCovGeodetic = 5908,
    DOP = 4001,
    PosCart = 4044,
    PosLocal = 4052,
    PosProjected = 4094,
    PVISatCartesian = 4008,
    PVIResiduals = 4009,
    RAIMStatistics = 4011,
    GEOCorrections = 5935,
    BaseVectorCart = 4043,
    BaseVectorGeod = 4028,
    FVISupport = 4076,
    FVISupportA = 4079,
    EndOfPVI = 5921,

    // Receiver Time Blocks
    ReceiverTime = 5914,
    xPPSOffset = 5911,

    // External Event Blocks
    ExtEvent = 5924,
    ExtEventPVICartesian = 4037,
    ExtEventPVIGeodetic = 4038,
    ExtEventBaseVectGeod = 4217,

    // Differential Correction Blocks
    DiffCorrIn = 5919,
    BaseStation = 5949,
    RICMDatum = 4049,

    // L-Band Demodulator Blocks
    LBandTrackerStatus = 4201,
    LBAS1DecoderStatus = 4202,
    LBAS1Messages = 4203,
    LBandBeams = 4204,

    // Status Blocks
    ChannelStatus = 4013,
    ReceiverStatus = 4014,
    SatVisibility = 4012,
    InputLink = 4090,
    OutputLink = 4091,
    NTRIPClientStatus = 4053,
    NTRIPServerStatus = 4122,
    IPStatus = 4058,
    WiFiAPStatus = 4054,
    WiFiClientStatus = 4096,
    DynDNSStatus = 4105,
    PowerStatus = 4101,
    QualityInd = 4082,
    DiskStatus = 4059,
    LogStatus = 4102,
    RFStatus = 4092,
    P2PPStatus = 4238,
    CosmosStatus = 4243,

    // Miscellaneous Blocks
    ReceiverSetup = 5902,
    RxComponents = 4084,
    RxMessage = 4103,
    Commands = 4015,
    Comment = 5936,
    BBSamples = 4040,
    ASCIIIn = 4075,

    // Advanced Blocks
    SystemInfo = 6000,
}

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
