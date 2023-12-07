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

## Specifications
### Syntax
- lines are terminated by a semicolon ;
- Blocks are enclosed in curly braces { }

### Basic Datatypes
- `int`: an integer data type, It's size is determined on runtime
- `float`: a floating point data type, It's size is determined on runtime
- `str`: a string data type for sequences of characters.

### Variables
```
var <identifier>(: <type>)? = <value>;

var name: str = "jack";
var name = "jack" // with type inference
```

### Functions
```
// * we might expect a repeating pattern
// ? we might expect that to exists
func <name>(<arg: type>*) < -> return type ?> {
    ...
}

func say_hello(name: str){ // return nothing
    print("{}", name)
}
```

### Methods
```
// * we might expect a repeating pattern

<method type> <method name>[(/identifier | identifier)*]((key: type)*)
{
    respond <status code> <response>
}


// identifiers in [] are automatically considered as url parameters
// unless if its prefixed with / then it's a path parameter
// url parameters that have been prefixed with ? means it's optional

// same function but different ways to get input


// url: /hello_path/jack
// payload: {}
// response: "Hello, Jack"
get hello_path[/name](){
    respond Status.OK format("Hello, {}", name)
}

// url: /hello_url?name=Jack
// payload: {}
// response: "Hello, Jack"
get hello_url[name](){
    respond Status.OK format("Hello, {}", name)
}

// input: /hello_json
// payload: {"name": str}
// response: "Hello, Jack"
get hello_json[](
    name: str
){
    respond Status.OK format("Hello, {}", name)
}

```