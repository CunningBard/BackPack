# BackPack
When you need a specialized language to make your REST API Backend

## Example
handler for a get request on the /hello endpoint
```
/*
this only expects a json paylod with only `name` as a key and a `string` as the value
   
   
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
get hello(name: str){
    respond Status.OK, { 
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
- `bool`: a boolean data type for true or false values.

### Variables
```
let <identifier>(: <type>)? = <value>;

let name: str = "jack";
let name = "jack" // with type inference
```

### Functions
```
// * we might expect a repeating pattern
// ? we might expect that to exists
func <name>(<arg: type>*) < -> return_type?> {
    ...
}

func say_hello(name: str){ // return nothing
    print("{}", name)
}
```

### Methods
```
// * we might expect a repeating pattern
// ? we might expect that to exists

#PATH   /<params>/*
#QUERY  <arg: type>*
#HEADER <arg: type>*
#HEADER EXPECT <arg: value>
<method> <name>(<arg: type>*) < -> return_type?> {
    ...
}

// methods use respond instead of return
// respond takes a status code and a payload
// example:

GET ping{
    respond Status.OK "pong";
}

// when return is used in a method, it will raise an error
// the following will error out

GET ping {
    return "pong";  // errors out
}


// url: /hello_url?name=Jack
// payload: {}
// response: "Hello, Jack"
#PATH /hello_url/{name}
GET hello_url {
    respond Status.OK format("Hello, {}", name)
}

// input: /hello_json
// payload: {"name": str}
// response: "Hello, Jack"
GET hello_json(
    name: str
){
    respond Status.OK format("Hello, {}", name)
}

// other methods are also supported
// post, put, patch, delete
```

### Loops
```
while <condition> {
    ...
}

for <identifier> in <iterable> {
    ...
}
```

### Conditionals
```
if <condition> {
    ...
} else if <condition> {
    ...
} else {
    ...
}
```

### Comments
```
// single line comment
/* 
    multi line comment
*/
```

### Imports
```
import <path>
from <path> import <identifier>
```

### Structs
```
// Comma separated list of key-value pairs
struct <name> {
    <identifier>: <type>, 
}

// example
struct Person {
    name: str,
    age: int,
}


// structs can also be used as an argument type for methods

// created a handler for a get request on the /say_hello endpoint
GET say_hello(person: Person){
    respond Status.OK, format("Hello, {}", person.name)
}
```

### Examples
```
#PATH /user/{name}/     // path parameters
#QUERY gender: str      // query parameters
#HEADER token           // header parameters
GET user(
    log_text: str       // payload parameters
){
    if !auth(token) {
        respond Status.UNAUTHORIZED, "Unauthorized"
    }
    
    Log.Info(log_text);
    
    let users = get_users(filter: {name: name, gender: gender});
    
    respond Status.OK, users;
}
``` 