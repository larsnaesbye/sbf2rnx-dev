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
    EndOfMeas = 5922
}

// --- Navigation Page Blocks
enum NavigationPageBlocks {
    GPSRawCA = 4017,
    GPSRawL2C = 4018
    // - here we fill out the rest from the PDF
}