use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{
    convert::TryInto,
    fmt,
    io::{Error, ErrorKind},
};

use crate::core::{SMBiosData, SMBiosVersion};

/// # Raw SMBIOS Data
///
/// When Windows kernel32 [GetSystemFirmwareTable](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable) function is called for RSMB,
/// the raw SMBIOS table provider ('RSMB') it retrieves the contents of this
/// raw SMBIOS firmware table structure.
pub struct WinSMBiosData {
    windows_header: Vec<u8>,
    /// SMBios table data
    pub smbios_data: SMBiosData,
}

impl WinSMBiosData {
    /// Offset of the Used20CallingMethod field (0)
    pub const USED20_CALLING_METHOD_OFFSET: usize = 0usize;

    /// Offset of the SMBIOSMajorVersion field (1)
    pub const SMBIOS_MAJOR_VERSION_OFFSET: usize = 1usize;

    /// Offset of the SMBIOSMinorVersion field (2)
    pub const SMBIOS_MINOR_VERSION_OFFSET: usize = 2usize;

    /// Offset of the DMIRevision field (3)
    pub const DMI_REVISION_OFFSET: usize = 3usize;

    /// Offset of the Length field (4)
    pub const TABLE_DATA_LENGTH_OFFSET: usize = 4usize;

    /// Offset of the SMBIOSTableData field (8)
    pub const SMBIOS_TABLE_DATA_OFFSET: usize = 8usize;

    /// Creates an instance of [WinSMBiosData]
    ///
    /// To retrieve this structure on a windows system call load_windows_smbios_data().
    ///
    /// The new() is provided publicly to allow loading data from other sources
    /// such as a file or from memory array as is done with testing.
    pub fn new(raw_smbios_data: Vec<u8>) -> Result<WinSMBiosData, Error> {
        if !WinSMBiosData::is_valid_win_smbios_data(&raw_smbios_data) {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid WinSMBiosData structure",
            ))
        } else {
            let windows_header =
                Vec::from(&raw_smbios_data[..WinSMBiosData::SMBIOS_TABLE_DATA_OFFSET]);
            let version = WinSMBiosData::version_from_raw_header(&windows_header);
            Ok(WinSMBiosData {
                windows_header,
                smbios_data: {
                    SMBiosData::from_vec_and_version(
                        Vec::from(&raw_smbios_data[WinSMBiosData::SMBIOS_TABLE_DATA_OFFSET..]),
                        Some(version),
                    )
                },
            })
        }
    }

    /// Verify if a block of data is a valid WinSMBiosData structure
    ///
    /// This only checks if the structure itself is valid and not whether the contained
    /// [SMBiosData] structure is valid or not.
    pub fn is_valid_win_smbios_data(raw_data: &Vec<u8>) -> bool {
        let length = raw_data.len();
        if length <= WinSMBiosData::SMBIOS_TABLE_DATA_OFFSET {
            return false;
        }

        // retrieve the table data length field
        let slice = raw_data
            .get(
                WinSMBiosData::TABLE_DATA_LENGTH_OFFSET
                    ..WinSMBiosData::TABLE_DATA_LENGTH_OFFSET + 4,
            )
            .unwrap();
        let table_data_length = u32::from_le_bytes(
            slice
                .try_into()
                .expect("array length does not match type width"),
        ) as usize;

        table_data_length == length - WinSMBiosData::SMBIOS_TABLE_DATA_OFFSET
    }

    /// The raw SMBIOS data this structure is wrapping
    pub fn raw_smbios_data(&self) -> &[u8] {
        self.windows_header.as_slice()
    }

    /// Used20CallingMethod
    pub fn used20_calling_method(&self) -> u8 {
        self.windows_header[WinSMBiosData::USED20_CALLING_METHOD_OFFSET]
    }

    /// SMBIOS major version
    pub fn smbios_major_version(&self) -> u8 {
        self.windows_header[WinSMBiosData::SMBIOS_MAJOR_VERSION_OFFSET]
    }

    /// SMBIOS minor version
    pub fn smbios_minor_version(&self) -> u8 {
        self.windows_header[WinSMBiosData::SMBIOS_MINOR_VERSION_OFFSET]
    }

    /// DMI revision
    pub fn dmi_revision(&self) -> u8 {
        self.windows_header[WinSMBiosData::DMI_REVISION_OFFSET]
    }

    fn version_from_raw_header(windows_header: &Vec<u8>) -> SMBiosVersion {
        SMBiosVersion {
            major: windows_header[WinSMBiosData::SMBIOS_MAJOR_VERSION_OFFSET],
            minor: windows_header[WinSMBiosData::SMBIOS_MINOR_VERSION_OFFSET],
            revision: windows_header[WinSMBiosData::DMI_REVISION_OFFSET],
        }
    }

    /// Length of the smbios table data
    pub fn table_data_length(&self) -> u32 {
        let slice = self
            .windows_header
            .get(
                WinSMBiosData::TABLE_DATA_LENGTH_OFFSET
                    ..WinSMBiosData::TABLE_DATA_LENGTH_OFFSET + 4,
            )
            .unwrap();
        u32::from_le_bytes(
            slice
                .try_into()
                .expect("array length does not match type width"),
        )
    }
}

impl fmt::Debug for WinSMBiosData {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<WinSMBiosData>())
            .field("used20_calling_method", &self.used20_calling_method())
            .field("smbios_major_version", &self.smbios_major_version())
            .field("smbios_minor_version", &self.smbios_minor_version())
            .field("dmi_revision", &self.dmi_revision())
            .field("table_data_length", &self.table_data_length())
            .field("smbios_data", &self.smbios_data)
            .finish()
    }
}

impl Serialize for WinSMBiosData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("WinSMBiosData", 6)?;
            state.serialize_field("used20_calling_method", &self.used20_calling_method())?;
            state.serialize_field("smbios_major_version", &self.smbios_major_version())?;
            state.serialize_field("smbios_minor_version", &self.smbios_minor_version())?;
            state.serialize_field("dmi_revision", &self.dmi_revision())?;
            state.serialize_field("table_data_length", &self.table_data_length())?;
            state.serialize_field("smbios_data", &self.smbios_data)?;
            state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_raw_smbios_data() {
        // Good structure (lengths are correct)
        let struct_data = vec![0x00u8, 0x03, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0xAB];
        assert!(WinSMBiosData::is_valid_win_smbios_data(&struct_data));

        // Bad structure (too short)
        let struct_data = vec![0x00u8, 0x03, 0x03];
        assert!(!WinSMBiosData::is_valid_win_smbios_data(&struct_data));

        // Bad structure (bad table data length)
        let struct_data = vec![0x00u8, 0x03, 0x03, 0x00, 0xFF, 0x00, 0x00, 0x00, 0xAB];
        assert!(!WinSMBiosData::is_valid_win_smbios_data(&struct_data));
    }

    #[test]
    fn test_win_smbios_data_headers() {
        let raw_win_data = vec![0x00u8, 0x03, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];

        let win_smbios_data = WinSMBiosData::new(raw_win_data).unwrap();

        assert_eq!(win_smbios_data.used20_calling_method(), 0x00);
        assert_eq!(win_smbios_data.smbios_major_version(), 0x03);
        assert_eq!(win_smbios_data.smbios_minor_version(), 0x04);
        assert_eq!(win_smbios_data.dmi_revision(), 0x00);
        assert_eq!(win_smbios_data.table_data_length(), 0x01);
    }

    #[test]
    fn test_win_smbios_data_constructor() {
        let raw_win_data = vec![0x00u8, 0x03, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x10, 0xFF];

        let win_smbios_data = WinSMBiosData::new(raw_win_data.clone()).unwrap();

        assert_eq!(
            win_smbios_data.windows_header.as_slice(),
            &raw_win_data[..8]
        );
    }
}
