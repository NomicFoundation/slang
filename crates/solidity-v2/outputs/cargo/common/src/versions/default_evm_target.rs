use crate::evm_targets::EvmTarget;
use crate::versions::LanguageVersion;

impl LanguageVersion {
    /// The default EVM target of this language version, used when none is requested.
    pub const fn default_evm_target(self) -> EvmTarget {
        match self {
            Self::V0_8_0 | Self::V0_8_1 | Self::V0_8_2 | Self::V0_8_3 | Self::V0_8_4 => {
                EvmTarget::Istanbul
            }

            Self::V0_8_5 | Self::V0_8_6 => EvmTarget::Berlin,

            Self::V0_8_7
            | Self::V0_8_8
            | Self::V0_8_9
            | Self::V0_8_10
            | Self::V0_8_11
            | Self::V0_8_12
            | Self::V0_8_13
            | Self::V0_8_14
            | Self::V0_8_15
            | Self::V0_8_16
            | Self::V0_8_17 => EvmTarget::London,

            Self::V0_8_18 | Self::V0_8_19 => EvmTarget::Paris,

            Self::V0_8_20 | Self::V0_8_21 | Self::V0_8_22 | Self::V0_8_23 | Self::V0_8_24 => {
                EvmTarget::Shanghai
            }

            Self::V0_8_25 | Self::V0_8_26 | Self::V0_8_27 | Self::V0_8_28 | Self::V0_8_29 => {
                EvmTarget::Cancun
            }

            Self::V0_8_30 => EvmTarget::Prague,

            Self::V0_8_31
            | Self::V0_8_32
            | Self::V0_8_33
            | Self::V0_8_34
            | Self::V0_8_35
            | Self::V0_8_36
            | Self::V0_8_37 => EvmTarget::Osaka,
        }
    }
}
