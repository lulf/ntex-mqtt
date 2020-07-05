use std::num::NonZeroU16;

use bytes::Bytes;
use bytestring::ByteString;
use ntex::router::Path;
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;

use crate::router::Route;
use crate::v5::codec;

/// Publish message
pub struct Publish {
    publish: codec::Publish,
    topic: Path<ByteString>,
}

impl Publish {
    pub(crate) fn new(publish: codec::Publish) -> Self {
        Self {
            topic: Path::new(publish.topic.clone()),
            publish,
        }
    }

    #[inline]
    /// this might be re-delivery of an earlier attempt to send the Packet.
    pub fn dup(&self) -> bool {
        self.publish.dup
    }

    #[inline]
    pub fn retain(&self) -> bool {
        self.publish.retain
    }

    #[inline]
    /// the level of assurance for delivery of an Application Message.
    pub fn qos(&self) -> codec::QoS {
        self.publish.qos
    }

    #[inline]
    /// the information channel to which payload data is published.
    pub fn publish_topic(&self) -> &str {
        &self.publish.topic
    }

    #[inline]
    /// only present in PUBLISH Packets where the QoS level is 1 or 2.
    pub fn id(&self) -> Option<NonZeroU16> {
        self.publish.packet_id
    }

    #[inline]
    pub fn topic(&self) -> &Path<ByteString> {
        &self.topic
    }

    #[inline]
    pub fn topic_mut(&mut self) -> &mut Path<ByteString> {
        &mut self.topic
    }

    #[inline]
    pub fn packet(&self) -> &codec::Publish {
        &self.publish
    }

    #[inline]
    /// the Application Message that is being published.
    pub fn payload(&self) -> &Bytes {
        &self.publish.payload
    }

    /// Extract Bytes from packet payload
    pub fn take_payload(&self) -> Bytes {
        self.publish.payload.clone()
    }

    /// Loads and parse `application/json` encoded body.
    pub fn json<T: DeserializeOwned>(&mut self) -> Result<T, JsonError> {
        serde_json::from_slice(&self.publish.payload)
    }

    /// Successfully ack publish packet
    pub fn ack(self) -> PublishAck {
        PublishAck {
            reason_code: codec::PublishAckReason::Success,
            properties: codec::UserProperties::default(),
            reason_string: None,
        }
    }
}

impl Route for Publish {
    #[inline]
    fn publish_topic(&self) -> &Path<ByteString> {
        &self.topic
    }

    #[inline]
    fn publish_topic_mut(&mut self) -> &mut Path<ByteString> {
        &mut self.topic
    }
}

impl std::fmt::Debug for Publish {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.publish.fmt(f)
    }
}

pub struct PublishAck {
    pub(crate) reason_code: codec::PublishAckReason,
    pub(crate) properties: codec::UserProperties,
    pub(crate) reason_string: Option<ByteString>,
}

impl PublishAck {
    pub fn properties<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut codec::UserProperties),
    {
        f(&mut self.properties);
        self
    }

    pub fn reason(mut self, reason: ByteString) -> Self {
        self.reason_string = Some(reason);
        self
    }
}