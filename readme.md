# BackPack
When you need a specialized language to make your REST API Backend

## Example
handler for a get request on the /hello endpoint
```
get hello[](
    name: str
){
    
    respond Status.OK { 
        message: format("Hello, {}!", name) 
    };
}

// json payload

// {
//     "name": "Jack"
// }

// response:

// Status Code: 200
// {
//     "message": "Hello, Jack!"
// }
```