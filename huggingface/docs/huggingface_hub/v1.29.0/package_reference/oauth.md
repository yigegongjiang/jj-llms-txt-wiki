# OAuth and FastAPI

OAuth is an open standard for access delegation, commonly used to grant applications limited access to a user's information without exposing their credentials. When combined with FastAPI it allows you to build secure APIs that allow users to log in using external identity providers like Google or GitHub.
In a usual scenario:
- FastAPI will define the API endpoints and handles the HTTP requests.
- OAuth is integrated using libraries like fastapi.security or external tools like Authlib.
- When a user wants to log in, FastAPI redirects them to the OAuth provider’s login page.
- After successful login, the provider redirects back with a token.
- FastAPI verifies this token and uses it to authorize the user or fetch user profile data.

This approach helps avoid handling passwords directly and offloads identity management to trusted providers.

# Hugging Face OAuth Integration in FastAPI

This module provides tools to integrate Hugging Face OAuth into a FastAPI application. It enables user authentication using the Hugging Face platform including mocked behavior for local development and real OAuth flow for Spaces.

## OAuth Overview

The `attach_huggingface_oauth` function adds login, logout, and callback endpoints to your FastAPI app. When used in a Space, it connects to the Hugging Face OAuth system. When used locally it will inject a mocked user. Click here to learn more about [adding a Sign-In with HF option to your Space](https://huggingface.co/docs/hub/en/spaces-oauth)

### How to use it?

```python
from huggingface_hub import attach_huggingface_oauth, parse_huggingface_oauth
from fastapi import FastAPI, Request

app = FastAPI()
attach_huggingface_oauth(app)

@app.get("/")
def greet_json(request: Request):
    oauth_info = parse_huggingface_oauth(request)
    if oauth_info is None:
        return {"msg": "Not logged in!"}
    return {"msg": f"Hello, {oauth_info.user_info.preferred_username}!"}
```

> [!TIP]
> You might also be interested in [a practical example that demonstrates OAuth in action](https://huggingface.co/spaces/Wauplin/fastapi-oauth/blob/main/app.py).
> For a more comprehensive implementation, check out [medoidai/GiveBackGPT](https://huggingface.co/spaces/medoidai/GiveBackGPT) Space which implements HF OAuth in a full-scale application.

### attach_huggingface_oauth[[huggingface_hub.attach_huggingface_oauth]]

#### huggingface_hub.attach_huggingface_oauth[[huggingface_hub.attach_huggingface_oauth]]

```python
huggingface_hub.attach_huggingface_oauth(app: fastapi.FastAPI, route_prefix: str = '/')
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_oauth.py#L124)

Add OAuth endpoints to a FastAPI app to enable OAuth login with Hugging Face.

How to use:
- Call this method on your FastAPI app to add the OAuth endpoints.
- Inside your route handlers, call `parse_huggingface_oauth(request)` to retrieve the OAuth info.
- If user is logged in, an [OAuthInfo](/docs/huggingface_hub/v1.29.0/en/package_reference/oauth#huggingface_hub.OAuthInfo) object is returned with the user's info. If not, `None` is returned.
- In your app, make sure to add links to `/oauth/huggingface/login` and `/oauth/huggingface/logout` for the user to log in and out.

Example:
```py
from huggingface_hub import attach_huggingface_oauth, parse_huggingface_oauth

# Create a FastAPI app
app = FastAPI()

# Add OAuth endpoints to the FastAPI app
attach_huggingface_oauth(app)

# Add a route that greets the user if they are logged in
@app.get("/")
def greet_json(request: Request):
    # Retrieve the OAuth info from the request
    oauth_info = parse_huggingface_oauth(request)  # e.g. OAuthInfo dataclass
    if oauth_info is None:
        return {"msg": "Not logged in!"}
    return {"msg": f"Hello, {oauth_info.user_info.preferred_username}!"}
```

### parse_huggingface_oauth[[huggingface_hub.parse_huggingface_oauth]]

#### huggingface_hub.parse_huggingface_oauth[[huggingface_hub.parse_huggingface_oauth]]

```python
huggingface_hub.parse_huggingface_oauth(request: fastapi.Request)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_oauth.py#L191)

Returns the information from a logged-in user as a [OAuthInfo](/docs/huggingface_hub/v1.29.0/en/package_reference/oauth#huggingface_hub.OAuthInfo) object.

For flexibility and future-proofing, this method is very lax in its parsing and does not raise errors.
Missing fields are set to `None` without a warning.

Return `None`, if the user is not logged in (no info in session cookie).

See [attach_huggingface_oauth()](/docs/huggingface_hub/v1.29.0/en/package_reference/oauth#huggingface_hub.attach_huggingface_oauth) for an example on how to use this method.

### OAuthOrgInfo[[huggingface_hub.OAuthOrgInfo]]

#### huggingface_hub.OAuthOrgInfo[[huggingface_hub.OAuthOrgInfo]]

```python
huggingface_hub.OAuthOrgInfo(sub: str, name: str, preferred_username: str, picture: str, plan: str | None = None, can_pay: bool | None = None, role_in_org: str | None = None, security_restrictions: list[typing.Literal['ip', 'token-policy', 'mfa', 'sso']] | None = None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_oauth.py#L23)

**Parameters:**

sub (`str`) : Unique identifier for the org. OpenID Connect field.

name (`str`) : The org's full name. OpenID Connect field.

preferred_username (`str`) : The org's username. OpenID Connect field.

picture (`str`) : The org's profile picture URL. OpenID Connect field.

plan (`str`, *optional*) : The org's plan (e.g., "enterprise", "team"). Hugging Face field.

can_pay (`Optional[bool]`, *optional*) : Whether the org has a payment method set up. Hugging Face field.

role_in_org (`Optional[str]`, *optional*) : The user's role in the org. Hugging Face field.

security_restrictions (`Optional[list[Literal["ip", "token-policy", "mfa", "sso"]]]`, *optional*) : Array of security restrictions that the user hasn't completed for this org. Possible values: "ip", "token-policy", "mfa", "sso". Hugging Face field.

Information about an organization linked to a user logged in with OAuth.

### OAuthUserInfo[[huggingface_hub.OAuthUserInfo]]

#### huggingface_hub.OAuthUserInfo[[huggingface_hub.OAuthUserInfo]]

```python
huggingface_hub.OAuthUserInfo(sub: str, name: str, preferred_username: str, email_verified: bool | None, email: str | None, picture: str, profile: str, website: str | None, is_pro: bool, can_pay: bool | None, orgs: list[huggingface_hub._oauth.OAuthOrgInfo] | None)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_oauth.py#L57)

**Parameters:**

sub (`str`) : Unique identifier for the user, even in case of rename. OpenID Connect field.

name (`str`) : The user's full name. OpenID Connect field.

preferred_username (`str`) : The user's username. OpenID Connect field.

email_verified (`Optional[bool]`, *optional*) : Indicates if the user's email is verified. OpenID Connect field.

email (`Optional[str]`, *optional*) : The user's email address. OpenID Connect field.

picture (`str`) : The user's profile picture URL. OpenID Connect field.

profile (`str`) : The user's profile URL. OpenID Connect field.

website (`Optional[str]`, *optional*) : The user's website URL. OpenID Connect field.

is_pro (`bool`) : Whether the user is a pro user. Hugging Face field.

can_pay (`Optional[bool]`, *optional*) : Whether the user has a payment method set up. Hugging Face field.

orgs (`Optional[list[OrgInfo]]`, *optional*) : List of organizations the user is part of. Hugging Face field.

Information about a user logged in with OAuth.

### OAuthInfo[[huggingface_hub.OAuthInfo]]

#### huggingface_hub.OAuthInfo[[huggingface_hub.OAuthInfo]]

```python
huggingface_hub.OAuthInfo(access_token: str, access_token_expires_at: datetime, user_info: OAuthUserInfo, state: str | None, scope: str)
```

[Source](https://github.com/huggingface/huggingface_hub/blob/v1.29.0/src/huggingface_hub/_oauth.py#L100)

**Parameters:**

access_token (`str`) : The access token.

access_token_expires_at (`datetime.datetime`) : The expiration date of the access token.

user_info ([OAuthUserInfo](/docs/huggingface_hub/v1.29.0/en/package_reference/oauth#huggingface_hub.OAuthUserInfo)) : The user information.

state (`str`, *optional*) : State passed to the OAuth provider in the original request to the OAuth provider.

scope (`str`) : Granted scope.

Information about the OAuth login.
