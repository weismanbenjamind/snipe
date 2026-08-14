use crate::errors::{FilesystemError, ResponseFormatterError, ResponseWriterError};
use crate::inputs::{RawFormat, ValidatedFormat, ValidatedGrab};
use crate::response::ResponseFormatter;
use bytes::Bytes;
use futures_util::StreamExt;
use log::{debug, info};
use reqwest::Response;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub(crate) struct ResponseWriter {
    response: Response,
}

impl ResponseWriter {
    pub(crate) fn new(response: Response) -> Self {
        Self { response }
    }

    pub(crate) async fn try_into_binary_file(
        self,
        output_file: &Path,
    ) -> Result<(), ResponseWriterError> {
        info!(
            "Trying to write response body as binary to {}.",
            output_file.display()
        );

        if !self.response.status().is_success() {
            return get_bad_response_for_binary_write_err(
                self.response.status().as_u16(),
                output_file,
            );
        }

        let mut file = open_output_file(output_file)?;

        info!("Staring response body stream.");
        let mut stream = self.response.bytes_stream();
        let mut chunk: Bytes; // Don't want to allocate memory every loop iteration

        while let Some(maybe_chunk) = stream.next().await {
            chunk = maybe_chunk.map_err(ResponseWriterError::binary_write_from_err)?;
            file.write_all(&chunk)
                .map_err(ResponseWriterError::binary_write_from_err)?;
        }

        info!(
            "Succesfully to wrote response body as binary to {}.",
            output_file.display()
        );
        Ok(())
    }

    pub(crate) async fn try_into_text_file(
        self,
        validated_grab: ValidatedGrab,
        validated_format: ValidatedFormat,
        pretty: bool,
        output_file: &Path,
    ) -> Result<(), ResponseWriterError> {
        info!(
            "Writing formatted response as string to file {}.",
            output_file.display()
        );

        let response_string = self
            .try_into_string(validated_grab, validated_format, pretty)
            .await?;
        try_create_parent_dirs(output_file)?;
        fs::write(output_file, response_string).map_err(|e| {
            ResponseWriterError::TextWrite(output_file.display().to_string(), e.to_string())
        })?;

        info!(
            "Successfully wrote formatted response as string to file {}.",
            output_file.display()
        );
        Ok(())
    }

    pub(crate) async fn try_into_console(
        self,
        validated_grab: ValidatedGrab,
        validated_format: ValidatedFormat,
        pretty: bool,
    ) -> Result<(), ResponseWriterError> {
        info!("Writing formatted response to console");
        println!(
            "{}",
            self.try_into_string(validated_grab, validated_format, pretty)
                .await?
        );
        info!("Successfully wrote formatted response to console.");
        Ok(())
    }

    async fn try_into_string(
        self,
        validated_grab: ValidatedGrab,
        validated_format: ValidatedFormat,
        pretty: bool,
    ) -> Result<String, ResponseWriterError> {
        info!("Transforming response into String.");
        let response_formatter = ResponseFormatter::try_from_response(self.response).await?;

        let result = match validated_grab.int_status_code() {
            true => response_formatter.status_code_string(),
            false => handle_string_formatted_output(
                response_formatter,
                validated_format,
                validated_grab,
                pretty,
            )?,
        };

        Ok(result)
    }
}

#[inline]
fn get_bad_response_for_binary_write_err(
    status_code: u16,
    output_file: &Path,
) -> Result<(), ResponseWriterError> {
    Err(ResponseWriterError::BadResponse(
        status_code,
        format!("Blocked writing to file {}", output_file.display()),
    ))
}

fn open_output_file<P: AsRef<Path>>(output_path: P) -> Result<fs::File, FilesystemError> {
    let as_ref = output_path.as_ref();

    // If we have a parent try to create all directoies
    // fs::create_all will pass through for any directories that already exist
    // This code should handle creating any directories leading to the output file that don't exist
    try_create_parent_dirs(as_ref)?;

    // Open the file for write only
    // Create the file if it doesn't exist
    debug!("Opening file at {}.", as_ref.display());
    fs::File::create(as_ref)
        .map_err(|e| FilesystemError::FileCreation(as_ref.display().to_string(), e.to_string()))
}

#[inline]
fn try_create_parent_dirs(path: &Path) -> Result<(), FilesystemError> {
    match path.parent() {
        Some(parent) => fs::create_dir_all(parent).map_err(|e| {
            FilesystemError::PathCreation(parent.display().to_string(), e.to_string())
        }),
        None => Ok(()),
    }
}

#[inline]
fn handle_string_formatted_output(
    response_formatter: ResponseFormatter,
    validated_format: ValidatedFormat,
    validated_grab: ValidatedGrab,
    pretty: bool,
) -> Result<String, ResponseFormatterError> {
    match validated_format.raw() {
        RawFormat::Http => {
            info!("Writing response to HTTP string");
            response_formatter.get_http_string(
                validated_grab.status_code(),
                validated_grab.headers(),
                validated_grab.body(),
            )
        }
        RawFormat::Json => {
            info!("Writing response to JSON string.");
            response_formatter.get_json_string(
                validated_grab.status_code(),
                validated_grab.headers(),
                validated_grab.body(),
                pretty,
            )
        }
        RawFormat::PrettyJson => {
            info!("Writing response to pretty JSON string.");
            response_formatter.get_json_string(
                validated_grab.status_code(),
                validated_grab.headers(),
                validated_grab.body(),
                true,
            )
        }
        RawFormat::Binary => Err(ResponseFormatterError::BinaryToString),
    }
}
