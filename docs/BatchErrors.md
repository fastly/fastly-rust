# BatchErrors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | Option<**String**> | The key that the error corresponds to. This field will be empty if the object or one of its fields was unable to be parsed. | 
**index** | Option<**i32**> | The line number of the payload on which the error occurred (starting from 0 for the first line). | 
**code** | Option<**String**> | The HTTP response code for the request, or a 400 if the request was not able to be completed. | 
**reason** | Option<**String**> | A descriptor of this particular item's error. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


