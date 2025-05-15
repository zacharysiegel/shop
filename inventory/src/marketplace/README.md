# Marketplace

## eBay

### Application authorization

There are a limited number of OAuth authorization scopes available to the application without requiring a delegate user token. All of them relate to purchases and are probably irrelevant.

Nevertheless, an application authorization token may be obtained using the "Client Credentials" OAuth flow via the following API request:

```http request
POST https://{domain}/identity/v1/oauth2/token
content-type: application/x-www-form-urlencoded
authorization: Basic base64(client_id:client_secret)

grant_type=client_credentials&scope=https://api.ebay.com/oauth/api_scope+https://api.ebay.com/oauth/api_scope/buy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.feed+https://api.ebay.com/oauth/api_scope/buy.marketing+https://api.ebay.com/oauth/api_scope/buy.product.feed+https://api.ebay.com/oauth/api_scope/buy.marketplace.insights+https://api.ebay.com/oauth/api_scope/buy.proxy.guest.order+https://api.ebay.com/oauth/api_scope/buy.item.bulk+https://api.ebay.com/oauth/api_scope/buy.deal
```

### User authorization

Reference: https://developer.ebay.com/api-docs/static/oauth-authorization-code-grant.html

Any useful seller management must be performed on behalf of an eBay user (e.g. create a listing). This requires the "Authorization Code Grant Flow" to receive a user authorization code, which then is exchanged for a user access token and user refresh token.

eBay provides a redirection link for our application to provide to users when a user wishes to delegate their authorization to the application (see the developer console). The user is redirected to authenticate with eBay, but we provide an `Accept-Url` HTTP header which eBay will redirect back to after successful authentication. If the `Accept-Url` is set to a URL managed by our application, we will be able to read the user's authorization code written into the query parameters of the redirection.

After a user is redirected back to our application, harvest the authorization code from the `code` query parameter in the redirect path. This authorization code can be used in the following API request to retrieve a user access token and a user refresh token:

```http request
POST https://api.sandbox.ebay.com/identity/v1/oauth2/token
content-type: application/x-www-form-urlencoded
authorization: Basic base64(client_id:client_secret)

grant_type=authorization_code&redirect_uri=<ru_name>&code=<authorization_code>
```

### Program enrollment

Reference: https://developer.ebay.com/api-docs/sell/account/resources/program/methods/optInToProgram

In order to use the Inventory API, the seller must already have opted in to the `SELLING_POLICY_MANAGEMENT` "program". The opt-in request requires a user access token with the following OAuth scope: `https://api.ebay.com/oauth/api_scope/sell.account`.

```http request
POST https://{domain}/sell/account/v1/program/opt_in
Authorization: Bearer <user_access_token>
Content-Type: application/json

{ /* Program */
	"programType" : "SELLING_POLICY_MANAGEMENT"
}
```

Program enrollment can be verified via the following request:

```http request
GET https://api.sandbox.ebay.com/sell/account/v1/program/get_opted_in_programs
Authorization: Bearer <user_access_token>
```

