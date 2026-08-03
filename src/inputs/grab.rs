use crate::errors::ArgsValidationError;
use crate::inputs::{RawFormat, ValidatedFormat};
use clap::Args;

#[derive(Args, Clone, Copy, Debug)]
#[group(multiple = true, required = false)]
pub struct RawGrab {
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
    pub fn new(
        status_code: bool,
        headers: bool,
        body: bool,
        int_status_code: bool,
        full: bool,
    ) -> Result<Self, ArgsValidationError> {
        let have_individuals = status_code || headers || body;

        if have_individuals && int_status_code {
            return ArgsValidationError::new_base(
                "Cannot pass status_code, headers, or body, with int_status_code.",
            );
        }

        if have_individuals && full {
            return ArgsValidationError::new_base(
                "Cannot pass status_code, headers, or body, with full",
            );
        }

        if int_status_code && full {
            return ArgsValidationError::new_base("Cannot pass full with int_status_code.");
        }

        Ok(Self {
            status_code,
            headers,
            body,
            int_status_code,
            full,
        })
    }

    pub fn status_code(&self) -> bool {
        self.status_code
    }

    pub fn headers(&self) -> bool {
        self.headers
    }

    pub fn body(&self) -> bool {
        self.body
    }

    pub fn int_status_code(&self) -> bool {
        self.int_status_code
    }

    pub fn full(&self) -> bool {
        self.full
    }

    pub fn in_default_state(&self) -> bool {
        !self.status_code && !self.headers && !self.body && !self.int_status_code && !self.full
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ValidatedGrab {
    status_code: bool,
    headers: bool,
    body: bool,
    int_status_code: bool,
}

impl ValidatedGrab {
    // Validation occurs at RawGrab level (for response component combos) and below for interaction with formatting
    pub fn new_validated(
        grab: RawGrab,
        validated_format: ValidatedFormat,
    ) -> Result<Self, ArgsValidationError> {
        let grab = Self::init_from_raw_grab(grab);
        match validated_format.raw_format() {
            RawFormat::Http => Ok(grab),
            RawFormat::Json => Ok(grab),
            RawFormat::PrettyJson => Ok(grab),
            RawFormat::Binary => match grab.only_body() {
                true => Ok(grab),
                false => Err(ArgsValidationError::NonBodyWithBinary),
            },
        }
    }

    #[inline]
    fn only_body(&self) -> bool {
        self.body && !self.headers && !self.status_code && !self.int_status_code
    }

    pub fn status_code(&self) -> bool {
        self.status_code
    }

    pub fn headers(&self) -> bool {
        self.headers
    }

    pub fn body(&self) -> bool {
        self.body
    }

    pub fn int_status_code(&self) -> bool {
        self.int_status_code
    }

    // Validation occurs at the RawGrab level
    // Raw grab validates good combos
    // Validation either happens at CLI level or via ::new() method
    fn init_from_raw_grab(value: RawGrab) -> Self {
        // If everything is false default to headers
        if value.in_default_state() {
            return Self {
                status_code: false,
                headers: false,
                body: true,
                int_status_code: false,
            };
        }

        // If full is true then set status_code, headers, and body to true
        // and int_status_code to false
        if value.full {
            return Self {
                status_code: true,
                headers: true,
                body: true,
                int_status_code: false,
            };
        }

        // Otherwise return all values as set in RawGrab
        Self {
            status_code: value.status_code,
            headers: value.headers,
            body: value.body,
            int_status_code: value.int_status_code,
        }
    }
}
