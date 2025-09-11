#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MpvFormat {
  MpvFormatNone,
  MpvFormatString,
  MpvFormatOsdString,
  MpvFormatFlag,
  MpvFormatInt64,
  MpvFormatDouble,
  MpvFormatNode,
  MpvFormatNodeArray,
  MpvFormatNodeMap,
  MpvFormatByteArray
}