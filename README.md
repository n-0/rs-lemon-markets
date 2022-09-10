# lemon_markets_rs - (community) Rust API client for lemon markets 🍋

Having a sunny day and the taste for sweet lemons in your crustacean mind? 
Wait no longer as here comes the (partly autogenerated) API client of the lemon
markets api for Rust. Due to some name clashes the OpenAPI generation of the specification
produces funny looking plain text (noncompilable code). 

This repository fixes this and supports ably live subscription
for real time data.
There isn't much testing right now and the docs are incomplete as well
as usability is clunky due to autogeneration. Will maybe fixed in the future.
If you come from the future with the solution plz open a Pull request!


## Rust API client for market_data 

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1da2ed0b5fefcf155fa143ba4b81c08c3c1c351c
- Package version: 1da2ed0b5fefcf155fa143ba4b81c08c3c1c351c
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to */v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*InstrumentsApi* | [**get_instruments_instruments_get**](docs/InstrumentsApi.md#get_instruments_instruments_get) | **GET** /instruments/ | Get Instruments
*OhlcApi* | [**ohlc_get_ohlc_d1_get**](docs/OhlcApi.md#ohlc_get_ohlc_d1_get) | **GET** /ohlc/d1/ | Ohlc Get
*OhlcApi* | [**ohlc_get_ohlc_h1_get**](docs/OhlcApi.md#ohlc_get_ohlc_h1_get) | **GET** /ohlc/h1/ | Ohlc Get
*OhlcApi* | [**ohlc_get_ohlc_m1_get**](docs/OhlcApi.md#ohlc_get_ohlc_m1_get) | **GET** /ohlc/m1/ | Ohlc Get
*QuotesApi* | [**get_latest_quote_quotes_latest_get**](docs/QuotesApi.md#get_latest_quote_quotes_latest_get) | **GET** /quotes/latest/ | Get Latest Quote
*QuotesApi* | [**get_quotes_quotes_get**](docs/QuotesApi.md#get_quotes_quotes_get) | **GET** /quotes/ | Get Quotes
*TradesApi* | [**get_latest_trades_trades_latest_get**](docs/TradesApi.md#get_latest_trades_trades_latest_get) | **GET** /trades/latest/ | Get Latest Trades
*TradesApi* | [**get_trades_trades_get**](docs/TradesApi.md#get_trades_trades_get) | **GET** /trades/ | Get Trades
*VenuesApi* | [**get_venues_venues_get**](docs/VenuesApi.md#get_venues_venues_get) | **GET** /venues/ | Get Venues


## Documentation For Models

 - [ApiEndpointsOhlcPageDict](docs/ApiEndpointsOhlcPageDict.md)
 - [ApiEndpointsQuotesPageDict](docs/ApiEndpointsQuotesPageDict.md)
 - [ApiEndpointsTradesPageDict](docs/ApiEndpointsTradesPageDict.md)
 - [From](docs/From.md)
 - [FromAnyOf](docs/FromAnyOf.md)
 - [Instrument](docs/Instrument.md)
 - [ModelType](docs/ModelType.md)
 - [OhlcDecimal](docs/OhlcDecimal.md)
 - [OhlcInt](docs/OhlcInt.md)
 - [OpeningHours](docs/OpeningHours.md)
 - [PageInstrument](docs/PageInstrument.md)
 - [PageUnionOhlcDecimalOhlcInt](docs/PageUnionOhlcDecimalOhlcInt.md)
 - [PageVenue](docs/PageVenue.md)
 - [ResponseError](docs/ResponseError.md)
 - [ResultsInner](docs/ResultsInner.md)
 - [SortType](docs/SortType.md)
 - [T](docs/T.md)
 - [Venue](docs/Venue.md)
 - [VenueSpecificAttrs](docs/VenueSpecificAttrs.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author
n-0 - Niklas Jona Lohmann


## Rust API client for trading

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: b996d5fa9acbe1796d08c2f8360172c63ac713ad
- Package version: b996d5fa9acbe1796d08c2f8360172c63ac713ad
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to */v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**account_bank_statements_get_account_bankstatements_get**](docs/AccountApi.md#account_bank_statements_get_account_bankstatements_get) | **GET** /account/bankstatements/ | Account Bank Statements Get
*AccountApi* | [**account_documents_get_account_documents_get**](docs/AccountApi.md#account_documents_get_account_documents_get) | **GET** /account/documents/ | Account Documents Get
*AccountApi* | [**account_documents_get_id_account_documents_document_id_get**](docs/AccountApi.md#account_documents_get_id_account_documents_document_id_get) | **GET** /account/documents/{document_id}/ | Account Documents Get Id
*AccountApi* | [**account_get_account_get**](docs/AccountApi.md#account_get_account_get) | **GET** /account/ | Account Get
*AccountApi* | [**account_put_account_put**](docs/AccountApi.md#account_put_account_put) | **PUT** /account/ | Account Put
*AccountApi* | [**account_withdraw_post_account_withdrawals_post**](docs/AccountApi.md#account_withdraw_post_account_withdrawals_post) | **POST** /account/withdrawals/ | Account Withdraw Post
*AccountApi* | [**account_withdrawals_get_account_withdrawals_get**](docs/AccountApi.md#account_withdrawals_get_account_withdrawals_get) | **GET** /account/withdrawals/ | Account Withdrawals Get
*OrdersApi* | [**order_get_orders_get**](docs/OrdersApi.md#order_get_orders_get) | **GET** /orders/ | Order Get
*OrdersApi* | [**order_id_activate_post_orders_order_id_activate_post**](docs/OrdersApi.md#order_id_activate_post_orders_order_id_activate_post) | **POST** /orders/{order_id}/activate/ | Order Id Activate Post
*OrdersApi* | [**order_id_delete_orders_order_id_delete**](docs/OrdersApi.md#order_id_delete_orders_order_id_delete) | **DELETE** /orders/{order_id}/ | Order Id Delete
*OrdersApi* | [**order_id_get_orders_order_id_get**](docs/OrdersApi.md#order_id_get_orders_order_id_get) | **GET** /orders/{order_id}/ | Order Id Get
*OrdersApi* | [**order_post_orders_post**](docs/OrdersApi.md#order_post_orders_post) | **POST** /orders/ | Order Post
*PositionsApi* | [**positions_get_positions_get**](docs/PositionsApi.md#positions_get_positions_get) | **GET** /positions/ | Positions Get
*PositionsApi* | [**positions_performance_get_positions_performance_get**](docs/PositionsApi.md#positions_performance_get_positions_performance_get) | **GET** /positions/performance/ | Positions Performance Get
*PositionsApi* | [**positions_statements_get_positions_statements_get**](docs/PositionsApi.md#positions_statements_get_positions_statements_get) | **GET** /positions/statements/ | Positions Statements Get
*UserApi* | [**user_get_user_get**](docs/UserApi.md#user_get_user_get) | **GET** /user/ | User Get


## Documentation For Models

 - [AccountResponse](docs/AccountResponse.md)
 - [ActivateOrderModel](docs/ActivateOrderModel.md)
 - [BankStatement](docs/BankStatement.md)
 - [CreateOrderRequest](docs/CreateOrderRequest.md)
 - [CreateOrderResponse](docs/CreateOrderResponse.md)
 - [DocumentResponse](docs/DocumentResponse.md)
 - [DocumentsResponseModel](docs/DocumentsResponseModel.md)
 - [GetUserResponse](docs/GetUserResponse.md)
 - [HttpValidationError](docs/HttpValidationError.md)
 - [LocationInner](docs/LocationInner.md)
 - [Mode](docs/Mode.md)
 - [PageBankStatement](docs/PageBankStatement.md)
 - [PagePortfolio](docs/PagePortfolio.md)
 - [PagePosition](docs/PagePosition.md)
 - [PagePositionProfit](docs/PagePositionProfit.md)
 - [PageReadOrderResponse](docs/PageReadOrderResponse.md)
 - [PageWithdrawal](docs/PageWithdrawal.md)
 - [PlaybookGlobalsModelsSchemaBankStatementTypes](docs/PlaybookGlobalsModelsSchemaBankStatementTypes.md)
 - [PlaybookGlobalsModelsSchemaOrderTypes](docs/PlaybookGlobalsModelsSchemaOrderTypes.md)
 - [Portfolio](docs/Portfolio.md)
 - [Position](docs/Position.md)
 - [PositionProfit](docs/PositionProfit.md)
 - [PositionType](docs/PositionType.md)
 - [ReadOrderResponse](docs/ReadOrderResponse.md)
 - [RegulatoryInformation](docs/RegulatoryInformation.md)
 - [ResponseAccountResponse](docs/ResponseAccountResponse.md)
 - [ResponseCreateOrderResponse](docs/ResponseCreateOrderResponse.md)
 - [ResponseDocumentResponse](docs/ResponseDocumentResponse.md)
 - [ResponseGetUserResponse](docs/ResponseGetUserResponse.md)
 - [ResponseListAppsApiEndpointsAccountDocumentsResponseModel](docs/ResponseListAppsApiEndpointsAccountDocumentsResponseModel.md)
 - [ResponseOk](docs/ResponseOk.md)
 - [ResponseReadOrderResponse](docs/ResponseReadOrderResponse.md)
 - [Sides](docs/Sides.md)
 - [Sorting](docs/Sorting.md)
 - [Status](docs/Status.md)
 - [UpdateAccountModel](docs/UpdateAccountModel.md)
 - [ValidationError](docs/ValidationError.md)
 - [Venue](docs/Venue.md)
 - [WithdrawMoneyRequest](docs/WithdrawMoneyRequest.md)
 - [Withdrawal](docs/Withdrawal.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author
n-0 - Niklas Jona Lohmann
