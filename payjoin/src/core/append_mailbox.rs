//! Client helpers for append-only Payjoin Directory mailboxes.
//!
//! A mailbox is addressed by a [`ShortId`] and holds a sequence of fixed-size
//! frames. Each frame is an HPKE ciphertext padded to [`PADDED_MESSAGE_BYTES`];
//! these helpers move encrypted frames to and from the directory and never see
//! plaintext.
//! response bytes into results. The caller supplies the [`ShortId`] and sends
//! each returned [`Request`] with its own HTTP client.

use crate::directory::{ShortId, PADDED_MESSAGE_BYTES};
use crate::ohttp::{
    ohttp_encapsulate, process_get_res, process_post_res, DirectoryResponseError,
    OhttpEncapsulationError,
};
use crate::{IntoUrl, IntoUrlError, OhttpKeys, Request, Url, UrlParseError};

/// Pairs a mailbox request with the state needed to read its response.
///
/// Hold it between sending a [`Request`] and processing the response; it is
/// consumed when the response is processed.
pub struct MailboxCtx(ohttp::ClientResponse);

/// Build the request that appends one `frame` to `mailbox`.
///
/// Returns the [`Request`] to send and the [`MailboxCtx`] to process the
/// response with. `frame` must be one HPKE ciphertext of [`PADDED_MESSAGE_BYTES`];
/// the caller encrypts the message before appending it.
pub fn append_request(
    ohttp_keys: &OhttpKeys,
    directory: &Url,
    ohttp_relay: impl IntoUrl,
    mailbox: &ShortId,
    frame: &[u8],
) -> Result<(Request, MailboxCtx), MailboxError> {
    let target = mailbox_endpoint(directory, mailbox);
    let (body, ctx) = ohttp_encapsulate(&ohttp_keys.0, "POST", target.as_str(), Some(frame))?;
    let request = Request::new_v2(&relay_url(ohttp_relay, directory)?, &body);
    Ok((request, MailboxCtx(ctx)))
}

/// Process the response to an [`append_request`].
pub fn process_append_response(res: &[u8], ctx: MailboxCtx) -> Result<(), MailboxError> {
    process_post_res(res, ctx.0).map_err(MailboxError::from)
}

/// Build the request that reads the entire `mailbox`.
pub fn read_request(
    ohttp_keys: &OhttpKeys,
    directory: &Url,
    ohttp_relay: impl IntoUrl,
    mailbox: &ShortId,
) -> Result<(Request, MailboxCtx), MailboxError> {
    let target = mailbox_endpoint(directory, mailbox);
    let (body, ctx) = ohttp_encapsulate(&ohttp_keys.0, "GET", target.as_str(), None)?;
    let request = Request::new_v2(&relay_url(ohttp_relay, directory)?, &body);
    Ok((request, MailboxCtx(ctx)))
}

/// Process the response to a [`read_request`] into the mailbox's frames.
pub fn process_read_response(res: &[u8], ctx: MailboxCtx) -> Result<Vec<Vec<u8>>, MailboxError> {
    match process_get_res(res, ctx.0)? {
        Some(blob) => split_frames(&blob),
        None => Ok(Vec::new()),
    }
}

/// Split a concatenated mailbox payload into its fixed-size frames.
///
/// Every frame is [`PADDED_MESSAGE_BYTES`]; a payload that isn't a whole number
/// of frames is rejected as truncated rather than yielding a partial frame.
pub fn split_frames(blob: &[u8]) -> Result<Vec<Vec<u8>>, MailboxError> {
    if blob.len() % PADDED_MESSAGE_BYTES != 0 {
        return Err(MailboxError::PartialFrame { len: blob.len() });
    }
    Ok(blob.chunks(PADDED_MESSAGE_BYTES).map(<[u8]>::to_vec).collect())
}

fn mailbox_endpoint(directory: &Url, mailbox: &ShortId) -> Url {
    let mut url = directory.clone();
    url.path_segments_mut()
        .expect("Payjoin Directory URL cannot be a base")
        .push(&mailbox.to_string());
    url
}

/// Relay URL that reveals only the directory's scheme and authority to the relay.
fn relay_url(ohttp_relay: impl IntoUrl, directory: &Url) -> Result<Url, MailboxError> {
    let relay_base = ohttp_relay.into_url()?;
    let directory_base = directory.join("/")?;
    Ok(relay_base.join(&format!("/{directory_base}"))?)
}

/// Error from building or processing a mailbox request.
#[derive(Debug)]
pub enum MailboxError {
    /// Failed to OHTTP-encapsulate the request.
    Encapsulation(OhttpEncapsulationError),
    /// The directory returned an unexpected or undecodable response.
    Response(DirectoryResponseError),
    /// Failed to parse the directory or relay URL.
    ParseUrl(UrlParseError),
    /// Failed to interpret the OHTTP relay argument as a URL.
    IntoUrl(IntoUrlError),
    /// The mailbox payload was not a whole number of frames.
    PartialFrame { len: usize },
}

impl From<OhttpEncapsulationError> for MailboxError {
    fn from(e: OhttpEncapsulationError) -> Self { Self::Encapsulation(e) }
}
impl From<DirectoryResponseError> for MailboxError {
    fn from(e: DirectoryResponseError) -> Self { Self::Response(e) }
}
impl From<UrlParseError> for MailboxError {
    fn from(e: UrlParseError) -> Self { Self::ParseUrl(e) }
}
impl From<IntoUrlError> for MailboxError {
    fn from(e: IntoUrlError) -> Self { Self::IntoUrl(e) }
}

impl std::fmt::Display for MailboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use MailboxError::*;
        match self {
            Encapsulation(e) => write!(f, "OHTTP encapsulation error: {e}"),
            Response(e) => write!(f, "directory response error: {e}"),
            ParseUrl(e) => write!(f, "URL parse error: {e}"),
            IntoUrl(e) => write!(f, "invalid relay URL: {e}"),
            PartialFrame { len } =>
                write!(f, "mailbox payload of {len} bytes is not a whole number of frames"),
        }
    }
}

impl std::error::Error for MailboxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use MailboxError::*;
        match self {
            Encapsulation(e) => Some(e),
            Response(e) => Some(e),
            ParseUrl(e) => Some(e),
            IntoUrl(e) => Some(e),
            PartialFrame { .. } => None,
        }
    }
}

