# BackPack
When you need a specialized language to make your REST API Backend

# CHECK OUT THE LANG SPEC AT /docs/lang_spec.md

## Example
handler for a get request on the /hello endpoint
```
GET hello {
    respond Status.OK "Hello! from BackPack";
}

#PATH /auth/login
#HEADER EXPECT Content-Type: application/json
POST (
    username: str,
    password: str
) {
    let user = authenticate(username, password);
    
    if user.is_none() {
        respond Status.UNAUTHORIZED "Invalid username or password";
    }
    
    let token = generate_token(user);
    
    respond Status.OK {
        "msg": "Login successful",
        "token": token
    };
}
```