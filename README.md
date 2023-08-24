# sbf2rnx-dev

A fun little side project to see if I can convert SBF (Septentrio) binary files to RINEX using Rust.

The format, though proprietary, is openly described in the Septentrio reference guide, e.g. page 244 on https://www.septentrio.com/system/files/support/asterx_sb_firmware_v4.8.4_reference_guide.pdf

Others have made parsers for it, e.g. here: https://github.com/asvol/gnss.net/blob/main/src/Asv.Gnss/Parsers/SBF/SbfBinaryParser.cs or https://raw.githubusercontent.com/tomojitakasu/RTKLIB/master/src/rcv/septentrio.c 

If it works, it might be turned into a plugin for the georust/rinex crate.