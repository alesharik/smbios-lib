use crate::*;
use std::iter::FromIterator;

/// # SMBIOS Standard Defined Structure
///
/// Represents one of the SMBIOS defined structures or, in the case
/// of an OEM defined structure, as a generically defined Unknown variant
#[derive(Debug)]
pub enum DefinedStruct<'a> {
    /// BIOS Information (Type 0)
    Information(SMBiosInformation<'a>),
    /// System Information (Type 1)
    SystemInformation(SMBiosSystemInformation<'a>),
    /// Baseboard (or Module) Information (Type 2)
    BaseBoardInformation(SMBiosBaseboardInformation<'a>),
    /// System Enclosure or Chassis (Type 3)
    SystemChassisInformation(SMBiosSystemChassisInformation<'a>),
    /// Processor Information (Type 4)
    ProcessorInformation(SMBiosProcessorInformation<'a>),
    /// Memory Controller Information (Type 5, Obsolete)
    MemoryControllerInformation(SMBiosMemoryControllerInformation<'a>),
    /// Memory Module Information (Type 6, Obsolete)
    MemoryModuleInformation(SMBiosMemoryModuleInformation<'a>),
    /// Cache Informaiton (Type 7)
    CacheInformation(SMBiosCacheInformation<'a>),
    /// Port Connector Information (Type 8)
    PortConnectorInformation(SMBiosPortConnectorInformation<'a>),
    /// System Slot Information (Type 9)
    SystemSlot(SMBiosSystemSlot<'a>),
    /// On Board Devices Information (Type 10, Obsolete)
    OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation<'a>),
    /// OEM Strings (Type 11)
    OemStrings(SMBiosOemStrings<'a>),
    /// System Configuration Options (Type 12)
    SystemConfigurationOptions(SMBiosSystemConfigurationOptions<'a>),
    /// BIOS Language Information (Type 13)
    LanguageInformation(SMBiosBiosLanguageInformation<'a>),
    /// Group Associations (Type 14)
    GroupAssociations(SMBiosGroupAssociations<'a>),
    /// System Event Log (Type 15)
    EventLog(SMBiosSystemEventLog<'a>),
    /// Physical Memory Array (Type 16)
    PhysicalMemoryArray(SMBiosPhysicalMemoryArray<'a>),
    /// Memory Device (Type 17)
    MemoryDevice(SMBiosMemoryDevice<'a>),
    /// 32-Bit Memory Error Information (Type 18)
    MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32<'a>),
    /// Memory Array Mapped Address (Type 19)
    MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress<'a>),
    /// Memory Device Mapped Address (Type 20)
    MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress<'a>),
    /// Built-in Pointing Device (Type 21)
    BuiltInPointingDevice(SMBiosBuiltInPointingDevice<'a>),
    /// Portable Battery (Type 22)
    PortableBattery(SMBiosPortableBattery<'a>),
    /// System Reset (Type 23)
    SystemReset(SMBiosSystemReset<'a>),
    /// Hardware Security (Type 24)
    HardwareSecurity(SMBiosHardwareSecurity<'a>),
    /// System Power Controls (Type 25)
    SystemPowerControls(SMBiosSystemPowerControls<'a>),
    /// Voltage Probe (Type 26)
    VoltageProbe(SMBiosVoltageProbe<'a>),
    /// Cooling Device (Type 27)
    CoolingDevice(SMBiosCoolingDevice<'a>),
    /// Temperature Probe (Type 28)
    TemperatureProbe(SMBiosTemperatureProbe<'a>),
    /// Electrical Current Probe (Type 29)
    ElectricalCurrentProbe(SMBiosElectricalCurrentProbe<'a>),
    /// Out-of-Band Remote Access (Type 30)
    OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess<'a>),
    /// Boot Integrity Services (BIS) (Type 31)
    BisEntryPoint(SMBiosBisEntryPoint<'a>),
    /// System Boot Information (Type 32)
    SystemBootInformation(SMBiosSystemBootInformation<'a>),
    /// 64-Bit Memory Error Information (Type 33)
    MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64<'a>),
    /// Management Device (Type 34)
    ManagementDevice(SMBiosManagementDevice<'a>),
    /// Management Device Component (Type 35)
    ManagementDeviceComponent(SMBiosManagementDeviceComponent<'a>),
    /// Management Device Threshold Data (Type 36)
    ManagementDeviceThresholdData(SMBiosManagementDeviceThresholdData<'a>),
    /// Memory Channel (Type 37)
    MemoryChannel(SMBiosMemoryChannel<'a>),
    /// IPMI Device Information (Type 38)
    IpmiDeviceInformation(SMBiosIpmiDeviceInformation<'a>),
    /// Power Supply (Type 39)
    SystemPowerSupply(SMBiosSystemPowerSupply<'a>),
    /// Additional Information (Type 40)
    AdditionalInformation(SMBiosAdditionalInformation<'a>),
    /// Onboard Devices Extended Information (Type 41)
    OnboardDevicesExtendedInformation(SMBiosOnboardDevicesExtendedInformation<'a>),
    /// Management Controller Host Interface (Type 42)
    ManagementControllerHostInterface(SMBiosManagementControllerHostInterface<'a>),
    /// TPM Device (Type 43)
    TpmDevice(SMBiosTpmDevice<'a>),
    /// Processor Additional Information (Type 44)
    ProcessorAdditionalInformation(SMBiosProcessorAdditionalInformation<'a>),
    /// Inactive (Type 126)
    Inactive(SMBiosInactive<'a>),
    /// End-of-Table (Type 127)
    EndOfTable(SMBiosEndOfTable<'a>),
    /// OEM-Defined or Unknown Structure
    ///
    /// - A structure with a type value not yet defined, such as by a DMTF specification
    /// that supercedes the types known by this library
    /// - An OEM type with a value > 127.
    Unknown(SMBiosUnknown<'a>),
}

impl<'a> From<&'a SMBiosStructParts<'a>> for DefinedStruct<'a> {
    fn from(parts: &'a SMBiosStructParts<'a>) -> Self {
        match parts.header.struct_type() {
            SMBiosInformation::STRUCT_TYPE => {
                DefinedStruct::Information(SMBiosInformation::new(parts))
            }
            SMBiosSystemInformation::STRUCT_TYPE => {
                DefinedStruct::SystemInformation(SMBiosSystemInformation::new(parts))
            }
            SMBiosBaseboardInformation::STRUCT_TYPE => {
                DefinedStruct::BaseBoardInformation(SMBiosBaseboardInformation::new(parts))
            }
            SMBiosSystemChassisInformation::STRUCT_TYPE => {
                DefinedStruct::SystemChassisInformation(SMBiosSystemChassisInformation::new(parts))
            }
            SMBiosProcessorInformation::STRUCT_TYPE => {
                DefinedStruct::ProcessorInformation(SMBiosProcessorInformation::new(parts))
            }
            SMBiosMemoryControllerInformation::STRUCT_TYPE => {
                DefinedStruct::MemoryControllerInformation(SMBiosMemoryControllerInformation::new(
                    parts,
                ))
            }
            SMBiosMemoryModuleInformation::STRUCT_TYPE => {
                DefinedStruct::MemoryModuleInformation(SMBiosMemoryModuleInformation::new(parts))
            }
            SMBiosCacheInformation::STRUCT_TYPE => {
                DefinedStruct::CacheInformation(SMBiosCacheInformation::new(parts))
            }
            SMBiosPortConnectorInformation::STRUCT_TYPE => {
                DefinedStruct::PortConnectorInformation(SMBiosPortConnectorInformation::new(parts))
            }
            SMBiosSystemSlot::STRUCT_TYPE => {
                DefinedStruct::SystemSlot(SMBiosSystemSlot::new(parts))
            }
            SMBiosOnBoardDeviceInformation::STRUCT_TYPE => {
                DefinedStruct::OnBoardDeviceInformation(SMBiosOnBoardDeviceInformation::new(parts))
            }
            SMBiosOemStrings::STRUCT_TYPE => {
                DefinedStruct::OemStrings(SMBiosOemStrings::new(parts))
            }
            SMBiosSystemConfigurationOptions::STRUCT_TYPE => {
                DefinedStruct::SystemConfigurationOptions(SMBiosSystemConfigurationOptions::new(
                    parts,
                ))
            }
            SMBiosBiosLanguageInformation::STRUCT_TYPE => {
                DefinedStruct::LanguageInformation(SMBiosBiosLanguageInformation::new(parts))
            }
            SMBiosGroupAssociations::STRUCT_TYPE => {
                DefinedStruct::GroupAssociations(SMBiosGroupAssociations::new(parts))
            }
            SMBiosSystemEventLog::STRUCT_TYPE => {
                DefinedStruct::EventLog(SMBiosSystemEventLog::new(parts))
            }
            SMBiosPhysicalMemoryArray::STRUCT_TYPE => {
                DefinedStruct::PhysicalMemoryArray(SMBiosPhysicalMemoryArray::new(parts))
            }
            SMBiosMemoryDevice::STRUCT_TYPE => {
                DefinedStruct::MemoryDevice(SMBiosMemoryDevice::new(parts))
            }
            SMBiosMemoryErrorInformation32::STRUCT_TYPE => {
                DefinedStruct::MemoryErrorInformation32Bit(SMBiosMemoryErrorInformation32::new(
                    parts,
                ))
            }
            SMBiosMemoryArrayMappedAddress::STRUCT_TYPE => {
                DefinedStruct::MemoryArrayMappedAddress(SMBiosMemoryArrayMappedAddress::new(parts))
            }
            SMBiosMemoryDeviceMappedAddress::STRUCT_TYPE => {
                DefinedStruct::MemoryDeviceMappedAddress(SMBiosMemoryDeviceMappedAddress::new(
                    parts,
                ))
            }
            SMBiosBuiltInPointingDevice::STRUCT_TYPE => {
                DefinedStruct::BuiltInPointingDevice(SMBiosBuiltInPointingDevice::new(parts))
            }
            SMBiosPortableBattery::STRUCT_TYPE => {
                DefinedStruct::PortableBattery(SMBiosPortableBattery::new(parts))
            }
            SMBiosSystemReset::STRUCT_TYPE => {
                DefinedStruct::SystemReset(SMBiosSystemReset::new(parts))
            }
            SMBiosHardwareSecurity::STRUCT_TYPE => {
                DefinedStruct::HardwareSecurity(SMBiosHardwareSecurity::new(parts))
            }
            SMBiosSystemPowerControls::STRUCT_TYPE => {
                DefinedStruct::SystemPowerControls(SMBiosSystemPowerControls::new(parts))
            }
            SMBiosVoltageProbe::STRUCT_TYPE => {
                DefinedStruct::VoltageProbe(SMBiosVoltageProbe::new(parts))
            }
            SMBiosCoolingDevice::STRUCT_TYPE => {
                DefinedStruct::CoolingDevice(SMBiosCoolingDevice::new(parts))
            }
            SMBiosTemperatureProbe::STRUCT_TYPE => {
                DefinedStruct::TemperatureProbe(SMBiosTemperatureProbe::new(parts))
            }
            SMBiosElectricalCurrentProbe::STRUCT_TYPE => {
                DefinedStruct::ElectricalCurrentProbe(SMBiosElectricalCurrentProbe::new(parts))
            }
            SMBiosOutOfBandRemoteAccess::STRUCT_TYPE => {
                DefinedStruct::OutOfBandRemoteAccess(SMBiosOutOfBandRemoteAccess::new(parts))
            }
            SMBiosBisEntryPoint::STRUCT_TYPE => {
                DefinedStruct::BisEntryPoint(SMBiosBisEntryPoint::new(parts))
            }
            SMBiosSystemBootInformation::STRUCT_TYPE => {
                DefinedStruct::SystemBootInformation(SMBiosSystemBootInformation::new(parts))
            }
            SMBiosMemoryErrorInformation64::STRUCT_TYPE => {
                DefinedStruct::MemoryErrorInformation64Bit(SMBiosMemoryErrorInformation64::new(
                    parts,
                ))
            }
            SMBiosManagementDevice::STRUCT_TYPE => {
                DefinedStruct::ManagementDevice(SMBiosManagementDevice::new(parts))
            }
            SMBiosManagementDeviceComponent::STRUCT_TYPE => {
                DefinedStruct::ManagementDeviceComponent(SMBiosManagementDeviceComponent::new(
                    parts,
                ))
            }
            SMBiosManagementDeviceThresholdData::STRUCT_TYPE => {
                DefinedStruct::ManagementDeviceThresholdData(
                    SMBiosManagementDeviceThresholdData::new(parts),
                )
            }
            SMBiosMemoryChannel::STRUCT_TYPE => {
                DefinedStruct::MemoryChannel(SMBiosMemoryChannel::new(parts))
            }
            SMBiosIpmiDeviceInformation::STRUCT_TYPE => {
                DefinedStruct::IpmiDeviceInformation(SMBiosIpmiDeviceInformation::new(parts))
            }
            SMBiosSystemPowerSupply::STRUCT_TYPE => {
                DefinedStruct::SystemPowerSupply(SMBiosSystemPowerSupply::new(parts))
            }
            SMBiosAdditionalInformation::STRUCT_TYPE => {
                DefinedStruct::AdditionalInformation(SMBiosAdditionalInformation::new(parts))
            }
            SMBiosOnboardDevicesExtendedInformation::STRUCT_TYPE => {
                DefinedStruct::OnboardDevicesExtendedInformation(
                    SMBiosOnboardDevicesExtendedInformation::new(parts),
                )
            }
            SMBiosManagementControllerHostInterface::STRUCT_TYPE => {
                DefinedStruct::ManagementControllerHostInterface(
                    SMBiosManagementControllerHostInterface::new(parts),
                )
            }
            SMBiosTpmDevice::STRUCT_TYPE => DefinedStruct::TpmDevice(SMBiosTpmDevice::new(parts)),
            SMBiosProcessorAdditionalInformation::STRUCT_TYPE => {
                DefinedStruct::ProcessorAdditionalInformation(
                    SMBiosProcessorAdditionalInformation::new(parts),
                )
            }
            SMBiosInactive::STRUCT_TYPE => DefinedStruct::Inactive(SMBiosInactive::new(parts)),
            SMBiosEndOfTable::STRUCT_TYPE => {
                DefinedStruct::EndOfTable(SMBiosEndOfTable::new(parts))
            }
            _ => DefinedStruct::Unknown(SMBiosUnknown::new(parts)),
        }
    }
}

#[derive(Debug)]
struct DefinedStructTable<'a>(Vec<DefinedStruct<'a>>);

impl<'a> DefinedStructTable<'a> {
    fn new() -> DefinedStructTable<'a> {
        DefinedStructTable(Vec::new())
    }

    fn add(&mut self, elem: DefinedStruct<'a>) {
        self.0.push(elem);
    }
}

impl<'a> IntoIterator for DefinedStructTable<'a> {
    type Item = DefinedStruct<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> FromIterator<&'a SMBiosStructParts<'a>> for DefinedStructTable<'a> {
    fn from_iter<I: IntoIterator<Item = &'a SMBiosStructParts<'a>>>(iter: I) -> Self {
        let mut defined_struct_table = DefinedStructTable::new();

        for struct_parts in iter {
            defined_struct_table.add(struct_parts.into());
        }

        defined_struct_table
    }
}
