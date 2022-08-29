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
}

enum ReceiverTimeBlocks {
    ReceiverTime = 5914,
    xPPSOffset = 5911,
}

enum ExternalEventBlocks {
    ExtEvent = 5924,
    ExtEventPVICartesian = 4037,
    ExtEventPVIGeodetic = 4038,
    ExtEventBaseVectGeod = 4217,
}

enum DifferentialCorrectionBlocks {
    DiffCorrIn = 5919,
    BaseStation = 5949,
    RICMDatum = 4049,
}

enum LBandDemodulatorBlocks {
    LBandTrackerStatus = 4201,
    LBAS1DecoderStatus = 4202,
    LBAS1Messages = 4203,
    LBandBeams = 4204,
}
enum StatusBlocks {
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
}

enum MiscellaneousBlocks {
    ReceiverSetup = 5902,
    RxComponents = 4084,
    RxMessage = 4103,
    Commands = 4015,
    Comment = 5936,
    BBSamples = 4040,
    ASCIIIn = 4075,
}

enum AdvancedBlocks {
    SystemInfo = 6000,
}
