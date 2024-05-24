/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultSettingsResponse {
    /// The type of filter to use while resizing an image.
    #[serde(rename = "resize_filter")]
    pub resize_filter: ResizeFilter,
    /// Controls whether or not to default to WebP output when the client supports it. This is equivalent to adding \"auto=webp\" to all image optimizer requests. 
    #[serde(rename = "webp")]
    pub webp: bool,
    /// The default quality to use with WebP output. This can be overridden with the second option in the \"quality\" URL parameter on specific image optimizer requests. 
    #[serde(rename = "webp_quality")]
    pub webp_quality: i32,
    /// The default type of JPEG output to use. This can be overridden with \"format=bjpeg\" and \"format=pjpeg\" on specific image optimizer requests. 
    #[serde(rename = "jpeg_type")]
    pub jpeg_type: JpegType,
    /// The default quality to use with JPEG output. This can be overridden with the \"quality\" parameter on specific image optimizer requests. 
    #[serde(rename = "jpeg_quality")]
    pub jpeg_quality: i32,
    /// Whether or not we should allow output images to render at sizes larger than input. 
    #[serde(rename = "upscale")]
    pub upscale: bool,
    /// Enables GIF to MP4 transformations on this service.
    #[serde(rename = "allow_video")]
    pub allow_video: bool,
}

impl DefaultSettingsResponse {
    pub fn new(resize_filter: ResizeFilter, webp: bool, webp_quality: i32, jpeg_type: JpegType, jpeg_quality: i32, upscale: bool, allow_video: bool) -> DefaultSettingsResponse {
        DefaultSettingsResponse {
            resize_filter,
            webp,
            webp_quality,
            jpeg_type,
            jpeg_quality,
            upscale,
            allow_video,
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

