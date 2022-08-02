# BillingTotal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bandwidth** | Option<**f32**> | The total amount of bandwidth used this month (See bandwidth_units for measurement). | 
**bandwidth_cost** | Option<**f32**> | The cost of the bandwidth used this month in USD. | 
**bandwidth_units** | Option<**String**> | Bandwidth measurement units based on billing plan. | 
**cost** | Option<**f32**> | The final amount to be paid. | 
**cost_before_discount** | Option<**f32**> | Total incurred cost plus extras cost. | 
**discount** | Option<**f32**> | Calculated discount rate. | 
**extras** | Option<[**Vec&lt;crate::models::BillingTotalExtras&gt;**](BillingTotalExtras.md)> | A list of any extras for this invoice. | 
**extras_cost** | Option<**f32**> | Total cost of all extras. | 
**incurred_cost** | Option<**f32**> | The total cost of bandwidth and requests used this month. | 
**overage** | Option<**f32**> | How much over the plan minimum has been incurred. | 
**plan_code** | Option<**String**> | The short code the plan this invoice was generated under. | 
**plan_minimum** | Option<**f32**> | The minimum cost of this plan. | 
**plan_name** | Option<**String**> | The name of the plan this invoice was generated under. | 
**requests** | Option<**f32**> | The total number of requests used this month. | 
**requests_cost** | Option<**f32**> | The cost of the requests used this month. | 
**terms** | Option<**String**> | Payment terms. Almost always Net15. | 

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


