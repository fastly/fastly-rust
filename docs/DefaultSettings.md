# DefaultSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**resize_filter** | Option<**String**> | The type of filter to use while resizing an image. | [default to ResizeFilter_Lanczos3]
**webp** | Option<**bool**> | Controls whether or not to default to WebP output when the client supports it. This is equivalent to adding \"auto=webp\" to all image optimizer requests.  | [default to false]
**webp_quality** | Option<**i32**> | The default quality to use with WebP output. This can be overridden with the second option in the \"quality\" URL parameter on specific image optimizer requests.  | [default to 85]
**jpeg_type** | Option<**String**> | The default type of JPEG output to use. This can be overridden with \"format=bjpeg\" and \"format=pjpeg\" on specific image optimizer requests.  | [default to JpegType_Auto]
**jpeg_quality** | Option<**i32**> | The default quality to use with JPEG output. This can be overridden with the \"quality\" parameter on specific image optimizer requests.  | [default to 85]
**upscale** | Option<**bool**> | Whether or not we should allow output images to render at sizes larger than input.  | [default to false]
**allow_video** | Option<**bool**> | Enables GIF to MP4 transformations on this service. | [default to false]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


