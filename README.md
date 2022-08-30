# WIP! Public "Secure" Transportation chat service

## TODO
- There is a lot of overhead in the size of the serialized struct
- The clients sends a new lookup for every new message. An obviously better solution is to cache the adresses on the client side as well, but I have no time to implement that at the moment.
- Probably some Rust spesific things as I am still learning the language
- I did not have time to test on anything other than localhost
- The client blocks while waiting for response from the server
- Error handling is pretty much non existent
