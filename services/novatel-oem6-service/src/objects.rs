//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use juniper::FieldResult;
use novatel_oem6_api::Component;

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    pub errors: String,
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    None,
    Noop,
    ControlPower,
    ConfigureHardware,
    TestHardware,
    IssueRawCommand,
}

/// Input structure for 'configureHardware' mutation
#[derive(GraphQLInputObject)]
pub struct ConfigStruct {
    pub option: ConfigOption,
    #[graphql(default = "false")]
    pub hold: bool,
    #[graphql(default = "0.0")]
    pub interval: f64,
    #[graphql(default = "0.0")]
    pub offset: f64,
}

/// Input field for 'configureHardware' mutation
///
/// Indicates which configuration operation should be performed
#[derive(GraphQLEnum, Debug)]
pub enum ConfigOption {
    /// Configure system to output error data when errors or events occur
    LogErrorData,
    /// Configure system to output position data at a requested interval
    LogPositionData,
    /// Stop generation of all output data from device
    UnlogAll,
    /// Stop generation of error data from device
    UnlogErrorData,
    /// Stop generation of position data from device
    UnlogPositionData,
}

/// Response fields for 'configureHardware' mutation
#[derive(GraphQLObject, Clone)]
pub struct ConfigureHardwareResponse {
    pub config: String,
    pub errors: String,
    pub success: bool,
}

/// Input field for 'testHardware' mutation
///
/// Indicates which test should be run
#[derive(GraphQLEnum)]
pub enum TestType {
    Integration,
    Hardware,
}

/// Enum for the 'testHardware' mutation response union
pub enum TestResults {
    Integration(IntegrationTestResults),
    Hardware(HardwareTestResults),
}

/// Response union for 'testHardware' mutation
graphql_union!(TestResults: () |&self| {
    instance_resolvers: |&_| {
        &IntegrationTestResults => match *self {
            TestResults::Integration(ref i) => Some(i),
            _ => None
        },
        &HardwareTestResults => match *self { TestResults::Hardware(ref h) => Some(h), _ => None},
    }
});

/// Response fields for 'testHardware(test: INTEGRATION)' mutation
#[derive(GraphQLObject)]
pub struct IntegrationTestResults {
    pub errors: String,
    pub success: bool,
    pub telemetry_debug: Option<VersionInfo>,
    /* TODO: Add telemetry_nominal */
}

/// Response fields for 'testHardware(test: HARDWARE)' mutation
#[derive(GraphQLObject)]
pub struct HardwareTestResults {
    pub errors: String,
    pub success: bool,
    pub data: String,
}

#[derive(Clone, Default)]
pub struct LockStatus {
    pub status: u32,
    pub position_type: u32,
    pub time: u32,
    pub position: bool,
    pub velocity: bool,
}

#[derive(GraphQLEnum, Debug)]
pub enum SolutionStatus {
    SolComputed,
    InsufficientObservations,
    NoConvergence,
    Singularity,
    CovarianceTraceExceeded,
    TestDistanceExceeded,
    ColdStart,
    HeightVelocityExceeded,
    VarianceExceeded,
    ResidualsTooLarge,
    IntegrityWarning,
    Pending,
    InvalidFix,
    Unauthorized,
    KubosInvalid,
}

#[derive(GraphQLEnum, Debug)]
pub enum PositionType {
    None,
    FixedPos,
    FixedHeight,
    DopplerVelocity,
    Single,
    PSRDiff,
    WAAS,
    Propagated,
    Omnistar,
    L1Float,
    IonoFreeFloat,
    NarrowFloat,
    L1Integer,
    NarrowInteger,
    OmnistarHP,
    OmnistarXP,
    PPPConverging,
    PPP,
    Operational,
    Warning,
    OutOfBounds,
    PPPBasicConverging,
    PPPBasic,
    KubosInvalid,
}

#[derive(GraphQLEnum, Debug)]
pub enum RefTimeStatus {
    Unknown,
    Approximate,
    CoarseAdjusting,
    Coarse,
    CoarseSteering,
    FreeWheeling,
    FineAdjusting,
    Fine,
    FineBackupSteering,
    FineSteering,
    SatTime,
    KubosInvalid,
}

graphql_object!(LockStatus: () | &self | {
    field status() -> FieldResult<SolutionStatus> {
        Ok(match self.status {
            0 => SolutionStatus::SolComputed,
            1 => SolutionStatus::InsufficientObservations,
            2 => SolutionStatus::NoConvergence,
            3 => SolutionStatus::Singularity,
            4 => SolutionStatus::CovarianceTraceExceeded,
            5 => SolutionStatus::TestDistanceExceeded,
            6 => SolutionStatus::ColdStart,
            7 => SolutionStatus::HeightVelocityExceeded,
            8 => SolutionStatus::VarianceExceeded,
            9 => SolutionStatus::ResidualsTooLarge,
            13 => SolutionStatus::IntegrityWarning,
            18 => SolutionStatus::Pending,
            19 => SolutionStatus::InvalidFix,
            20 => SolutionStatus::Unauthorized,
        	_ => SolutionStatus::KubosInvalid
        })
    }
    
    field position_type() -> FieldResult<PositionType> {
        Ok(match self.position_type {
            0 => PositionType::None,
            1 => PositionType::FixedPos,
            2 => PositionType::FixedHeight,
            8 => PositionType::DopplerVelocity,
            16 => PositionType::Single,
            17 => PositionType::PSRDiff,
            18 => PositionType::WAAS,
            19 => PositionType::Propagated,
            20 => PositionType::Omnistar,
            32 => PositionType::L1Float,
            33 => PositionType::IonoFreeFloat,
            34 => PositionType::NarrowFloat,
            48 => PositionType::L1Integer,
            50 => PositionType::NarrowInteger,
            64 => PositionType::OmnistarHP,
            65 => PositionType::OmnistarXP,
            68 => PositionType::PPPConverging,
            69 => PositionType::PPP,
            70 => PositionType::Operational,
            71 => PositionType::Warning,
            72 => PositionType::OutOfBounds,
            77 => PositionType::PPPBasicConverging,
            78 => PositionType::PPPBasic,
            _ => PositionType::KubosInvalid
        })
    }
    
    field time() -> FieldResult<RefTimeStatus> {
        Ok(match self.time {
            20 => RefTimeStatus::Unknown,
            60 => RefTimeStatus::Approximate,
            80 => RefTimeStatus::CoarseAdjusting,
            100 => RefTimeStatus::Coarse,
            120 => RefTimeStatus::CoarseSteering,
            130 => RefTimeStatus::FreeWheeling,
            140 => RefTimeStatus::FineAdjusting,
            160 => RefTimeStatus::Fine,
            170 => RefTimeStatus::FineBackupSteering,
            180 => RefTimeStatus::FineSteering,
            200 => RefTimeStatus::SatTime,
            _ => RefTimeStatus::KubosInvalid
        })
    }
    
    field position() -> FieldResult<bool> {
        // TODO
        Ok(false)
    }
    
    field velocity() -> FieldResult<bool> {
        // TODO
        Ok(false)
    }
});

#[derive(Clone, Default)]
pub struct LockInfo {
    pub time: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
}

graphql_object!(LockInfo: () | &self | {
    field time() -> FieldResult<f64> {
        Ok(self.time)
    }
    
    field position() -> FieldResult<Vec<f64>> {
        Ok(self.position.to_vec())
    }
    
    field velocity() -> FieldResult<Vec<f64>> {
        Ok(self.velocity.to_vec())
    }
});

/// Version information about the device, returned as the
/// `telemetryDebug` response field
#[derive(GraphQLObject)]
pub struct VersionInfo {
    pub num_components: i32,
    pub components: Vec<VersionComponent>,
}

pub struct VersionComponent(pub Component);

graphql_object!(VersionComponent: () | &self | {
    field comp_type() -> FieldResult<i32> {
        Ok(self.0.comp_type as i32)
    }

    field model() -> FieldResult<String> {
        Ok(self.0.model.clone())
    }

    field serial_num() -> FieldResult<String> {
        Ok(self.0.serial_num.clone())
    }

    field hw_version() -> FieldResult<String> {
        Ok(self.0.hw_version.clone())
    }

    field sw_version() -> FieldResult<String> {
        Ok(self.0.sw_version.clone())
    }

    field boot_version() -> FieldResult<String> {
        Ok(self.0.boot_version.clone())
    }

    field compile_date() -> FieldResult<String> {
        Ok(self.0.compile_date.clone())
    }

    field compile_time() -> FieldResult<String> {
        Ok(self.0.compile_time.clone())
    }
});
