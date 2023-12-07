# BackPack
When you need a specialized language to make your API Backend

## Example
handler for a get request on the /hello endpoint
```
/*
this only expects a json paylod with only name as a key and a string as the value
   
   
---------------------------------------
    payload:
    { 
        "name": "Jack"
    }
    
    response:
    Code: 200
    payload: "Hello, Jack!"
    
---------------------------------------
    payload:
    { 
        "namesssssss": "Jack"
    }
    
    response:
    Code: 400
    response payload: 
    {
        "error type": "Invalid Data",
        "missing": ["name"],
        "unknown": ["namesssssss"]
    }
---------------------------------------
*/
get hello(
    name: str
){
    
    respond Status.OK { 
        message: format("Hello, {}!", name) 
    };
} 


```