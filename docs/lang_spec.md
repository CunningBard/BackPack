# BackPack
When you need a specialized language to make your REST API Backend

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
Variables in BackPack are declared using the `let` keyword.

Types are optional and can be inferred from the value.

However, it is recommended to specify the type for better readability and to avoid bugs.

```
let <identifier> (: <type>)? = <value>;

let name: str = "jack";
let name = "jack"; // with type inference
```

### Functions
Functions in BackPack are declared using the `func` keyword.



```
// * we might expect a repeating pattern
// ? we might expect that to exists, basically optional

func <name>(<arg: type>*) -> return_type? {
    ...
}

func say_hello(name: str){ // return nothing
    print("{}", name)
}
```

### Methods
Since BackPack is designed for building REST APIs, it has a special syntax for defining methods.

Methods are defined using the REST method names (GET, POST, PUT, PATCH, DELETE, etc.) followed by the name of the method.

Methods have specifiers for path, query, and header parameters.

The syntax for defining a method is as follows:
```
// * we might expect a repeating pattern
// ? we might expect that to exists

#PATH   /<params>/*
#QUERY  <arg: type>*
#HEADER <arg: type>*
#HEADER EXPECT <arg: value>
<method> <name>(<arg: type>*) {
    ...
}
```
`respond` is used to send a response to the client.

When `return` is declared within a method, it will raise an error.
```
GET ping {
    return "pong";  // errors out
}
```
### Specifiers
- `#PATH`: specifies the path of the endpoint
- `#QUERY`: specifies the query parameters
- `#HEADER`: specifies the header parameters
- `#HEADER EXPECT`: specifies the expected value of a header parameter

---


When using the `#PATH` specifier, the path parameters are declared like an url path.

Dynamic path parameters are declared using curly braces `{}`.

```
#PATH /hello_url/{name}
GET hello_url {
    respond Status.OK format("Hello, {}", name);
}
```

When `#PATH` is not declared, the path is assumed to be the name of the method.

```
GET hello_url {
    respond Status.OK "Hello! from BackPack";
}
```

When `#PATH` is declared more than once, the compiler will raise an error.

```
// This will raise an error

#PATH /hello_url/{name}
#PATH /hello/{name}
GET hello_url {
    respond Status.OK format("Hello, {}", name)
}
```
---
When using the `#QUERY` specifier, the query parameters are declared by a key-value pair basis.

Which will match the query parameters of the request.

The request `GET /users?name=jack&gender=male` will match the query parameters of the method below.

```
#QUERY gender: str
#QUERY name: str
GET users {
    let users = get_users(filter: {
        gender: gender,
        name: name,
    });
    
    respond Status.OK users;
}
```

Query parameters can be optional, and can be declared using the `?` operator.

```
#QUERY width: int?
#QUERY height: int?
#QUERY file_path: str

GET image {
    let img = ImageReader.Path(file_path);
    
    if !img.exists() {
        respond Status.NOT_FOUND "File not found";
    }
    
    width = width.unwrap_or(file.width);
    height = height.unwrap_or(file.height);
    
    img.resize(width, height);
    
    respond Status.OK img;
}
```

When the supplied Query Parameters is missing, the compiler will automatically respond with a **BadRequest**.

When the supplied Query Parameters is the wrong type, the compiler will automatically respond with a **BadRequest**.

---

When using the `#HEADER` specifier, the header parameters already have a known type, 
So when using the `#HEADER` specifier, the type is not required rather the name of the 
header parameter is only required.
```
#HEADER Authorization
GET users {
    if !auth(Authorization) {
        respond Status.UNAUTHORIZED, "Unauthorized"
    }
    
    let users = get_users();
    
    respond Status.OK users;
}
```

---

When using the `#HEADER EXPECT` specifier, the expected value of the header parameter is declared.

`#HEADER EXPECT` is a special kind of specifier, since it's used like a match/switch-case statement, 
When `#HEADER EXPECT` specifier has multiple values, it will act like as if it's a `or` operator.
```
#HEADER EXPECT Content-Type: application/json
GET ping {
    respond Status.OK "pong json";
}

#HEADER EXPECT Content-Type: application/json
#HEADER EXPECT Content-Type: application/xml
GET pinged {
    respond Status.OK "pong json or xml";
}
```

But there is also a case where you can implement an if statement using the `#HEADER EXPECT` specifier.

The code below will basically dispatch the request to the appropriate method based on the value of the `Content-Type` header.

```
#HEADER EXPECT Content-Type: application/json
GET ping {
    respond Status.OK "pong json";
}

#HEADER EXPECT Content-Type: application/xml
GET ping {
    respond Status.OK "pong xml";
}
```

---

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