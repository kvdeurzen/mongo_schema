# MongoSchema

This library tries to implement a library that simplifies the use of mongodb. 
The idea is that one can construct a struct and that attribute derive macro's create all implementation that handles posting, updating, and retrieval from a Mongo database.
Additionally the goal is that structs can reference each other by ID.

## Usage
```rust
#[mongo_schema(collection = "example_schema")]
struct ExampleSchema {
    pub a_field: usize,
}
```

The atribute macro `mongo_schema` is used. It accepts the following argument:
**collection**: the name of the mongodb Collection in which the documents are stored.

Associated functions that are implemented for each schema:
`new`: initializes an instance. It requires an arguments for each field that the was added to the struct.
`collection_name`: returns a slice of the name of the connection in which the schema objects are saved in mongodb.

Methods that are implemented for each schema:
`post`: Saves the instance as a new document on mongodb.
`update`: Updates the related document on mongodb based on current state of the instance.
`save`: If the object has no id; `post`. Else; `update`.

## Architectural overview

The sub crate `mongo_schema_macro` introduces the `MongoSchema` trait.

The sub crate `mongo_schema_macro_derive` introduces the derive-macro that rebuilds a given struct and build all implementations.

`OneCell` is used to initiate a single mongodb connection. This connection is used by all struct implementations.