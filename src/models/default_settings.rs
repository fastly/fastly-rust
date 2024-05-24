/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultSettings {
    /// The type of filter to use while resizing an image.
    #[serde(rename = "resize_filter", skip_serializing_if = "Option::is_none")]
    pub resize_filter: Option<ResizeFilter>,
    /// Controls whether or not to default to WebP output when the client supports it. This is equivalent to adding \"auto=webp\" to all image optimizer requests. 
    #[serde(rename = "webp", skip_serializing_if = "Option::is_none")]
    pub webp: Option<bool>,
    /// The default quality to use with WebP output. This can be overridden with the second option in the \"quality\" URL parameter on specific image optimizer requests. 
    #[serde(rename = "webp_quality", skip_serializing_if = "Option::is_none")]
    pub webp_quality: Option<i32>,
    /// The default type of JPEG output to use. This can be overridden with \"format=bjpeg\" and \"format=pjpeg\" on specific image optimizer requests. 
    #[serde(rename = "jpeg_type", skip_serializing_if = "Option::is_none")]
    pub jpeg_type: Option<JpegType>,
    /// The default quality to use with JPEG output. This can be overridden with the \"quality\" parameter on specific image optimizer requests. 
    #[serde(rename = "jpeg_quality", skip_serializing_if = "Option::is_none")]
    pub jpeg_quality: Option<i32>,
    /// Whether or not we should allow output images to render at sizes larger than input. 
    #[serde(rename = "upscale", skip_serializing_if = "Option::is_none")]
    pub upscale: Option<bool>,
    /// Enables GIF to MP4 transformations on this service.
    #[serde(rename = "allow_video", skip_serializing_if = "Option::is_none")]
    pub allow_video: Option<bool>,
}

impl DefaultSettings {
    pub fn new() -> DefaultSettings {
        DefaultSettings {
            resize_filter: None,
            webp: None,
            webp_quality: None,
            jpeg_type: None,
            jpeg_quality: None,
            upscale: None,
            allow_video: None,
        }
    }
}

/// The type of filter to use while resizing an image.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResizeFilter {
    #[serde(rename = "lanczos3")]
    Lanczos3,
    #[serde(rename = "lanczos2")]
    Lanczos2,
    #[serde(rename = "bicubic")]
    Bicubic,
    #[serde(rename = "bilinear")]
    Bilinear,
    #[serde(rename = "nearest")]
    Nearest,
}

impl Default for ResizeFilter {
    fn default() -> ResizeFilter {
        Self::Lanczos3
    }
}
/// The default type of JPEG output to use. This can be overridden with \"format=bjpeg\" and \"format=pjpeg\" on specific image optimizer requests. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JpegType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "baseline")]
    Baseline,
    #[serde(rename = "progressive")]
    Progressive,
}

impl Default for JpegType {
    fn default() -> JpegType {
        Self::Auto
    }
}

