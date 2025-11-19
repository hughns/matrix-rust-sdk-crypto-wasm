//! Types for QR code login

use matrix_sdk_crypto::types::qr_login::{self};
use url::Url;
use wasm_bindgen::prelude::*;

use crate::vodozemac::Curve25519PublicKey;

/// The intent of the QR code login.
///
/// The QR code login mechanism supports both, the new device, as well as the
/// existing device to display the QR code.
///
/// The different intents have an explicit one-byte identifier which gets added to
/// the QR code data.
#[wasm_bindgen]
#[derive(Debug)]
pub enum QrCodeIntent {
    /// The new device is displaying the QR code.
    Login,
    /// The existing device is displaying the QR code.
    Reciprocate,
}

impl From<qr_login::QrCodeIntent> for QrCodeIntent {
    fn from(value: qr_login::QrCodeIntent) -> Self {
        match value {
            qr_login::QrCodeIntent::Login => Self::Login,
            qr_login::QrCodeIntent::Reciprocate => Self::Reciprocate,
        }
    }
}

impl From<QrCodeIntent> for qr_login::QrCodeIntent {
    fn from(value: QrCodeIntent) -> Self {
        match value {
            QrCodeIntent::Login => Self::Login,
            QrCodeIntent::Reciprocate => Self::Reciprocate,
        }
    }
}

/// Data for the QR code login mechanism.
///
/// The {@link QrCodeData} can be serialized and encoded as a QR code or it can
/// be decoded from a QR code.
#[wasm_bindgen]
#[derive(Debug)]
pub struct QrCodeData {
    inner: qr_login::QrCodeData,
}

#[wasm_bindgen]
impl QrCodeData {
    /// Create new {@link QrCodeData} from a given public key, a rendezvous URL
    /// and, optionally, a server name for the homeserver.
    ///
    /// If a server name is given, then the {@link QrCodeData} mode will be
    /// {@link QrCodeMode.Reciprocate}, i.e. the QR code will contain data for
    /// the existing device to display the QR code.
    ///
    /// If no server name is given, the {@link QrCodeData} mode will be
    /// {@link QrCodeMode.Login}, i.e. the QR code will contain data for the
    /// new device to display the QR code.
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: Curve25519PublicKey,
        rendezvous_id: &str,
        base_url: String,
        intent: QrCodeIntent,
    ) -> Result<QrCodeData, JsError> {
        let public_key = public_key.inner;
        let base_url = Url::parse(&base_url)?;
        let rendezvous_id = rendezvous_id.to_owned();

        let intent: qr_login::QrCodeIntent = intent.into();

        let inner = qr_login::QrCodeData { public_key, rendezvous_id, base_url, intent };

        Ok(QrCodeData { inner })
    }

    /// Attempt to decode a slice of bytes into a {@link QrCodeData} object.
    ///
    /// The slice of bytes would generally be returned by a QR code decoder.
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: &[u8]) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_bytes(bytes)? })
    }

    /// Encode the {@link QrCodeData} into a list of bytes.
    ///
    /// The list of bytes can be used by a QR code generator to create an image
    /// containing a QR code.
    #[wasm_bindgen(js_name = "toBytes")]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    /// Attempt to decode a base64 encoded string into a {@link QrCodeData}
    /// object.
    #[wasm_bindgen(js_name = "fromBase64")]
    pub fn from_base64(data: &str) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_base64(data)? })
    }

    /// Encode the {@link QrCodeData} into a string using base64.
    ///
    /// This format can be used for debugging purposes and the
    /// [`QrcodeData::from_base64()`] method can be used to parse the string
    /// again.
    #[wasm_bindgen(js_name = "toBase64")]
    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }

    /// Get the Curve25519 public key embedded in the {@link QrCodeData}.
    ///
    /// This Curve25519 public key should be used to establish an
    /// [ECIES](https://en.wikipedia.org/wiki/Integrated_Encryption_Scheme)
    /// (Elliptic Curve Integrated Encryption Scheme) channel with the other
    /// device.
    #[wasm_bindgen(getter, js_name = "publicKey")]
    pub fn public_key(&self) -> Curve25519PublicKey {
        self.inner.public_key.into()
    }

    /// Get the ID of the rendezvous which will be used to exchange
    /// messages between the two devices.
    #[wasm_bindgen(getter, js_name = "rendezvousId")]
    pub fn rendezvous_id(&self) -> String {
        self.inner.rendezvous_id.as_str().to_owned()
    }

    /// Get the base URL of the homeserver hosting the rendezvous.
    #[wasm_bindgen(getter, js_name = "baseUrl")]
    pub fn base_url(&self) -> String {
        self.inner.base_url.as_str().to_owned()
    }

    /// Get the mode of this {@link QrCodeIntent} instance.
    #[wasm_bindgen(getter)]
    pub fn intent(&self) -> QrCodeIntent {
        self.inner.intent.clone().into()
    }
}
