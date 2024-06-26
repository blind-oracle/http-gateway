use futures::Stream;
use http::Response;
use ic_agent::AgentError;
use std::{
    fmt::{Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
};

pub type CanisterResponse = Response<HttpGatewayResponseBody>;

/// A response from the HTTP gateway.
#[derive(Debug)]
pub struct HttpGatewayResponse {
    /// The certified response, excluding uncertified headers.
    /// If response verification v1 is used, the original, uncertified headers are returned.
    pub canister_response: CanisterResponse,

    /// Additional metadata regarding the response.
    pub metadata: HttpGatewayResponseMetadata,
}

/// The body of an HTTP gateway response.
#[derive(Debug)]
pub enum HttpGatewayResponseBody {
    /// A byte array representing the response body.
    Bytes(Vec<u8>),

    /// A stream of response body chunks.
    Stream(ResponseBodyStream),
}

/// An item in a response body stream.
pub type ResponseBodyStreamItem = Result<Vec<u8>, AgentError>;

/// A stream of response body chunks.
pub struct ResponseBodyStream {
    inner: Pin<Box<dyn Stream<Item = ResponseBodyStreamItem> + Send>>,
}

// Trait bound added for cloning.
impl ResponseBodyStream {
    pub fn new(stream: impl Stream<Item = ResponseBodyStreamItem> + Send + 'static) -> Self {
        Self {
            inner: Box::pin(stream),
        }
    }
}

// Debug implementation remains the same
impl Debug for ResponseBodyStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseBodyStream").finish()
    }
}

// Stream implementation remains the same
impl Stream for ResponseBodyStream {
    type Item = ResponseBodyStreamItem;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.as_mut().poll_next(cx)
    }
}

/// Additional metadata regarding the response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpGatewayResponseMetadata {
    /// Whether the original query call was upgraded to an update call.
    pub upgraded_to_update_call: bool,

    /// The version of response verification that was used to verify the response.
    /// If the protocol fails before getting to the verification step, or the
    /// original query call is upgraded to an update call, this field will be `None`.
    pub response_verification_version: Option<u16>,
}
