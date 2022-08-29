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

// ====== SBF BLOCKS ======

// --- Measurement Blocks ---
enum MeasurementBlocks {
    MeasEpoch = 4027,
    MeasExtra = 4000,
    Meas3Ranges = 4109,
    Meas3CN0HiRes = 4110,
    Meas3Doppler = 4111,
    Meas3PP = 4112,
    Meas3MP = 4113,
    EndOfMeas = 5922,
}

// --- Navigation Page Blocks
enum NavigationPageBlocks {
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
}
enum GPSDecodedMessageBlocks {
    GPSNav = 5891,
    GPSAlm = 5892,
    GPSIon = 5893,
    GPSUtc = 5894,
}

enum GLONASSDecodedMessageBlocks {
    GLONav = 4004,
    GLOAlm = 4005,
    GLOTime = 4036,
}

enum GalileoDecodedMessageBlocks {
    GALNav = 4002,
    GALAlm = 4003,
    GALIon = 4030,
    GALUtc = 4031,
    GALGstGps = 4032,
    GALSARRLM = 4034,
}

enum BeiDouDecodedMessageBlocks {
    BDSNav = 4081,
    BDSAlm = 4119,
    BDSIon = 4120,
    BDSUtc = 4121,
}
enum QZSSDecodedMessageBlocks {
    QZSNav = 4095,
    QZSAlm = 4116,
}

enum SBASL1DecodedMessageBlocks {
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
}

enum PositionVelocityAndTimeBlocks {
    PVICartesian = 4006,
    PVIGeodetic = 4007,
    // Fill in more from the PDF
}