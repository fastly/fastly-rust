# WafFirewallVersionDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_http_versions** | Option<**String**> | Allowed HTTP versions. | [default to HTTP/1.0 HTTP/1.1 HTTP/2]
**allowed_methods** | Option<**String**> | A space-separated list of HTTP method names. | [default to GET HEAD POST OPTIONS PUT PATCH DELETE]
**allowed_request_content_type** | Option<**String**> | Allowed request content types. | [default to application/x-www-form-urlencoded|multipart/form-data|text/xml|application/xml|application/x-amf|application/json|text/plain]
**allowed_request_content_type_charset** | Option<**String**> | Allowed request content type charset. | [default to utf-8|iso-8859-1|iso-8859-15|windows-1252]
**arg_name_length** | Option<**i32**> | The maximum allowed argument name length. | [default to 100]
**arg_length** | Option<**i32**> | The maximum allowed length of an argument. | [default to 400]
**combined_file_sizes** | Option<**i32**> | The maximum allowed size of all files (in bytes). | [default to 10000000]
**comment** | Option<**String**> | A freeform descriptive note. | 
**critical_anomaly_score** | Option<**i32**> | Score value to add for critical anomalies. | [default to 6]
**crs_validate_utf8_encoding** | Option<**bool**> | CRS validate UTF8 encoding. | 
**error_anomaly_score** | Option<**i32**> | Score value to add for error anomalies. | [default to 5]
**high_risk_country_codes** | Option<**String**> | A space-separated list of country codes in ISO 3166-1 (two-letter) format. | 
**http_violation_score_threshold** | Option<**i32**> | HTTP violation threshold. | 
**inbound_anomaly_score_threshold** | Option<**i32**> | Inbound anomaly threshold. | 
**lfi_score_threshold** | Option<**i32**> | Local file inclusion attack threshold. | 
**locked** | Option<**bool**> | Whether a specific firewall version is locked from being modified. | [default to false]
**max_file_size** | Option<**i32**> | The maximum allowed file size, in bytes. | [default to 10000000]
**max_num_args** | Option<**i32**> | The maximum number of arguments allowed. | [default to 255]
**notice_anomaly_score** | Option<**i32**> | Score value to add for notice anomalies. | [default to 4]
**number** | Option<**i32**> |  | [readonly]
**paranoia_level** | Option<**i32**> | The configured paranoia level. | [default to 1]
**php_injection_score_threshold** | Option<**i32**> | PHP injection threshold. | 
**rce_score_threshold** | Option<**i32**> | Remote code execution threshold. | 
**restricted_extensions** | Option<**String**> | A space-separated list of allowed file extensions. | [default to .asa/ .asax/ .ascx/ .axd/ .backup/ .bak/ .bat/ .cdx/ .cer/ .cfg/ .cmd/ .com/ .config/ .conf/ .cs/ .csproj/ .csr/ .dat/ .db/ .dbf/ .dll/ .dos/ .htr/ .htw/ .ida/ .idc/ .idq/ .inc/ .ini/ .key/ .licx/ .lnk/ .log/ .mdb/ .old/ .pass/ .pdb/ .pol/ .printer/ .pwd/ .resources/ .resx/ .sql/ .sys/ .vb/ .vbs/ .vbproj/ .vsdisco/ .webinfo/ .xsd/ .xsx]
**restricted_headers** | Option<**String**> | A space-separated list of allowed header names. | [default to /proxy/ /lock-token/ /content-range/ /translate/ /if/]
**rfi_score_threshold** | Option<**i32**> | Remote file inclusion attack threshold. | 
**session_fixation_score_threshold** | Option<**i32**> | Session fixation attack threshold. | 
**sql_injection_score_threshold** | Option<**i32**> | SQL injection attack threshold. | 
**total_arg_length** | Option<**i32**> | The maximum size of argument names and values. | [default to 6400]
**warning_anomaly_score** | Option<**i32**> | Score value to add for warning anomalies. | 
**xss_score_threshold** | Option<**i32**> | XSS attack threshold. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


