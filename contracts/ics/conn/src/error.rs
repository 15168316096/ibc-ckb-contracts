use core::result;

use ckb_std::error::SysError;

pub type Result<T> = result::Result<T, Error>;

#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    UnknownSysError,

    ClientEncoding,
    ConnectionEncoding,
    EnvelopeEncoding,
    MsgEncoding,

    WitnessIsIncorrect,
    WitnessTooMany,

    LoadScriptErr,
    LoadCellDataErr,

    UnexpectedMsg,
    ConnectionProofInvalid,
    ConnectionHashUnmatch,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(_) => Self::UnknownSysError,
        }
    }
}
