# DashboardItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Dashboard item identifier (UUID) | [readonly]
**title** | **String** | A human-readable title for the dashboard item | 
**subtitle** | **String** | A human-readable subtitle for the dashboard item. Often a description of the visualization. | 
**data_source** | [**crate::models::DashboardItemPropertyDataSource**](DashboardItemPropertyDataSource.md) |  | 
**visualization** | [**crate::models::DashboardItemPropertyVisualization**](DashboardItemPropertyVisualization.md) |  | 
**span** | Option<**i32**> | The number of columns for the dashboard item to span. Dashboards are rendered on a 12-column grid on \"desktop\" screen sizes. | [default to 4]

[[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


