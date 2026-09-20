# Quantframe Data Protection Policy

By using Quantframe, you agree to the collection and use of information in accordance with this policy.

## Information Collection and Use

We collect several different types of information for various purposes to provide and improve our Service
to you.

## Types of Data Collected

### Personal Data

While using our Service, we may ask you to provide us with certain personally identifiable information that can be used to contact or identify you ("Personal Data"). Personally identifiable information may include, but is not limited to:

- Warframe Market Username
- Warframe Market User ID
- Device ID (collected in a fully anonymous way used for tracking/securing purposes)
- Quantframe Version
- Event data (feature usage events)
- Operating System example: windows, macOS, linux, etc.
- Stock item/riven details relevant to the user's activity (e.g., adding/deleting/updating a stock item/
  riven)

### Stock Data

We may collect information on the stock items/rivens you add, delete, or update. This
includes tracking the specific details of the stock items/rivens you interact with while
using our Service.
You can turn off the collection of this data by disabling the Analytics tag in the settings.

### Metrics Data

We collect event data on how the Service is used. Each event has a name (for
example `app_start`, `stock_item_create`, or `page_view`) and a set of
properties that describe the action without revealing its contents. Events are
queued locally and sent to our servers in batches.

For example, a `page_view` event only records which section of the app was
opened:

```
{
  "event": "page_view",
  "properties": {
    "page": "warframe_market"
  }
}
```

Actions that can fail include a `success` property (`"true"` or `"false"`), and a
non-sensitive `error_type` when `success` is `"false"`. Bulk actions include a
`count`. Event properties never include item names, prices, trading partners,
usernames, or chat message contents.

You can turn off the collection of this data by disabling the Analytics tag in the settings.

### Last Updated

This document was last updated on September 20, 2026.

<!-- <ID>0.0.2</ID>. -->
