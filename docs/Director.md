# Director

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**backends** | Option<[**Vec&lt;crate::models::Backend&gt;**](Backend.md)> | List of backends associated to a director. | 
**capacity** | Option<**i32**> | Unused. | 
**comment** | Option<**String**> | A freeform descriptive note. | 
**name** | Option<**String**> | Name for the Director. | 
**quorum** | Option<**i32**> | The percentage of capacity that needs to be up for a director to be considered up. `0` to `100`. | [default to 75]
**shield** | Option<**String**> | Selected POP to serve as a shield for the backends. Defaults to `null` meaning no origin shielding if not set. Refer to the [POPs API endpoint](/reference/api/utils/pops/) to get a list of available POPs used for shielding. | [default to null]
**_type** | Option<**i32**> | What type of load balance group to use. | [default to Type_random]
**retries** | Option<**i32**> | How many backends to search if it fails. | [default to 5]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


