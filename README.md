# WIP! Jobber fortsatt på dette, men ta gjerne en titt på koden.

## TODO
- There is a lot of overhead in the size of the serialized struct
- The clients sends a new lookup for every new message. An obviously better solution is to cache the adresses on the client side as well, but I have no time to implement that at the moment.
- Probably some Rust spesific things as I am still learning the language
- I did not have time to test on anything other than localhost
- The client blocks while waiting for response from the server
- Error handling is pretty much non existent
- Lots of unwraps that should be handled instead
- Blocking while waiting for lookup response***
- Sjekker ikke bruker input (navn kan være med whitespaces ol.)