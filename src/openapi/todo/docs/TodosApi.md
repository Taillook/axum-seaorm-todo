# \TodosApi

All URIs are relative to *http://localhost:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_todo**](TodosApi.md#get_todo) | **GET** /todo | 
[**post_todo**](TodosApi.md#post_todo) | **POST** /todo | 



## get_todo

> crate::models::TodoResponse get_todo()


todo list

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TodoResponse**](TodoResponse.md)

### Authorization

[TodoAuth](../README.md#TodoAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_todo

> crate::models::Todo post_todo(post_todo_request)


todo create

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_todo_request** | Option<[**PostTodoRequest**](PostTodoRequest.md)> |  |  |

### Return type

[**crate::models::Todo**](Todo.md)

### Authorization

[TodoAuth](../README.md#TodoAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

