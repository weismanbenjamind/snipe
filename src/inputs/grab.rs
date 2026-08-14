use std::collections::HashSet;

use crate::containers::output::GrabCfg;
use crate::errors::ArgsValidationError;
use crate::inputs::{RawFormat, ValidatedFormat};
use clap::Args;

// * Note - do not want to fields public for RawGrab
// Depend on clap validation to ensure proper combos
// Need to encapsulate to ensure always getting proper combos
// If need to create RawGrab outside of ClI should use the RawGrabBuilder struct
#[derive(Args, Clone, Copy, Debug)]
#[group(multiple = true, required = false)]
pub(crate) struct RawGrab {
    #[arg(
        long,
        short = 'S',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the status code should be grabbed from the response"
    )]
    status_code: bool,

    #[arg(
        long,
        short = 'H',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the headers should be grabbed from the response"
    )]
    headers: bool,

    #[arg(
        long,
        short = 'B',
        conflicts_with_all = ["int_status_code", "full"],
        help = "If the body should be grabbed from the response. If nothing is specified to grabbed from the response, the body will be grabbed by default"
    )]
    body: bool,

    #[arg(short = 'I', long, conflicts_with_all = ["status_code", "headers", "body", "full"], help = "Grab only the status code as an integer")]
    int_status_code: bool,

    #[arg(short = 'F', long, conflicts_with_all = ["status_code", "headers", "body", "int_status_code"], help = "Grab status code, headers, and body")]
    full: bool,
}

impl RawGrab {
    fn empty(&self) -> bool {
        !self.status_code && !self.headers && !self.body && !self.int_status_code && !self.full
    }
}

impl TryFrom<&Vec<GrabCfg>> for RawGrab {
    type Error = ArgsValidationError;
    fn try_from(value: &Vec<GrabCfg>) -> Result<Self, ArgsValidationError> {
        let unique: HashSet<&GrabCfg> = value.iter().collect();
        RawGrabBuilder::new()
            .with_status_code(unique.contains(&GrabCfg::IntStatusCode))
            .with_headers(unique.contains(&GrabCfg::Headers))
            .with_body(unique.contains(&GrabCfg::Body))
            .with_int_status_code(unique.contains(&GrabCfg::IntStatusCode))
            .with_full(unique.contains(&GrabCfg::Full))
            .build()
    }
}

impl Default for RawGrab {
    fn default() -> Self {
        Self {
            status_code: false,
            headers: false,
            body: true, // Default to grab body
            int_status_code: false,
            full: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct RawGrabBuilder {
    status_code: Option<bool>,
    headers: Option<bool>,
    body: Option<bool>,
    int_status_code: Option<bool>,
    full: Option<bool>,
}

impl RawGrabBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn with_status_code(mut self, status_code: bool) -> Self {
        self.status_code = Some(status_code);
        self
    }
    fn with_headers(mut self, headers: bool) -> Self {
        self.headers = Some(headers);
        self
    }

    fn with_body(mut self, body: bool) -> Self {
        self.body = Some(body);
        self
    }

    fn with_int_status_code(mut self, int_status_code: bool) -> Self {
        self.int_status_code = Some(int_status_code);
        self
    }

    fn with_full(mut self, full: bool) -> Self {
        self.full = Some(full);
        self
    }

    fn build(self) -> Result<RawGrab, ArgsValidationError> {
        let status_code = self
            .status_code
            .ok_or(ArgsValidationError::GrabNotSet("status code"))?;

        let headers = self
            .headers
            .ok_or(ArgsValidationError::GrabNotSet("headers"))?;

        let body = self.body.ok_or(ArgsValidationError::GrabNotSet("body"))?;

        let int_status_code = self
            .int_status_code
            .ok_or(ArgsValidationError::GrabNotSet("int_status_code"))?;

        let full = self.full.ok_or(ArgsValidationError::GrabNotSet("full"))?;

        if full && (status_code || headers || body || int_status_code) {
            return Err(ArgsValidationError::InvalidGrab("full response"));
        }

        if int_status_code && (status_code || headers || body || full) {
            return Err(ArgsValidationError::InvalidGrab("int status code"));
        }

        Ok(RawGrab {
            status_code,
            headers,
            body,
            int_status_code,
            full,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ValidatedGrab {
    status_code: bool,
    headers: bool,
    body: bool,
    int_status_code: bool,
}

impl ValidatedGrab {
    // Validation occurs at RawGrab level (for response component combos) and below for interaction with formatting
    pub(crate) fn new(
        grab: RawGrab,
        validated_format: ValidatedFormat,
    ) -> Result<Self, ArgsValidationError> {
        let grab = Self::init_from_raw_grab(grab)?;
        match validated_format.raw() {
            RawFormat::Http => Ok(grab),
            RawFormat::Json => Ok(grab),
            RawFormat::PrettyJson => Ok(grab),
            RawFormat::Binary => match grab.only_body() {
                true => Ok(grab),
                false => Err(ArgsValidationError::NonBodyWithBinary),
            },
        }
    }

    pub(crate) fn status_code(&self) -> bool {
        self.status_code
    }

    pub(crate) fn headers(&self) -> bool {
        self.headers
    }

    pub(crate) fn body(&self) -> bool {
        self.body
    }

    pub(crate) fn int_status_code(&self) -> bool {
        self.int_status_code
    }

    #[inline]
    fn only_body(&self) -> bool {
        self.body && !self.headers && !self.status_code && !self.int_status_code
    }

    // Validation occurs at the RawGrab level
    // Raw grab validates good combos
    // Validation either happens at CLI level or via ::new() method on RawGrab
    fn init_from_raw_grab(value: RawGrab) -> Result<Self, ArgsValidationError> {
        if value.empty() {
            return Err(ArgsValidationError::MissingGrab);
        }

        // If full is true then set status_code, headers, and body to true
        // and int_status_code to false
        if value.full {
            return Ok(Self {
                status_code: true,
                headers: true,
                body: true,
                int_status_code: false,
            });
        }

        // Otherwise return all values as set in RawGrab
        Ok(Self {
            status_code: value.status_code,
            headers: value.headers,
            body: value.body,
            int_status_code: value.int_status_code,
        })
    }
}

impl Default for ValidatedGrab {
    fn default() -> Self {
        Self {
            status_code: false,
            headers: false,
            body: true, // Default to grab body
            int_status_code: false,
        }
    }
}
