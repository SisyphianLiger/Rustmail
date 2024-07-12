# 3.7.3.3 Serialisation In Rust
1. Why do we need serde?
2. What does serde actually do for us?
    - A framework for serializing and deserialzing Rust data structures efficiently and generically
## 3.7.3.3.1 Generically 
1. Does not provide support for (de)serialization from/to any specific data
    a. Will need serde_json for example for json
2. A library that can support serialisation impl Serializer trait
    - each trait corresponds to the 29 types that form serde's data model
    - i.e. JSON serializaition == output an open squarebracket [ return type to serialize sequence elements
3. Serialis::serialize for a rust type == specify how to decompose it according to serdes model 
    - happens at compiler time is agnostic

## 3.7.3.3.2 Efficiently
1. Serde is not slow because of monomorphization (i love fp)
    - generic func called with concrete set of types, compiler optimizes each instance of func
    - no runtime costs for generics this way
    - serde mem usage --> no intermediate serialised struct
    - all deserialized types are available at compile-time 

## 3.7.3.3.3 Conveniently
1. the two procedural macros bundled with serde parse the definition of your type and automatically gen the right impl

## 3.7.3.3.4 Putting Everything Together
1. before calling subscribe actix-web invokes form_request for all subscribers input args
2. Form::from_request tries to deserialize the body into FormData according to rules of URL 
3. if Form::from_request fails --> 400 BAD REQUEST if succeeds 200 OK


