
## Basic Steps needed 

### Build Async TOML File reader which parses it into structs
The Basic File and notify code has been written and tried out to get used to the functions of the notify-module.
It still is not very polished and needs more improvements. 

###  Build generic Warp routes which are getting constructed using the toml config files
The idea is to write a warp-Filter which adapts to the given config file and serves the correct stuff.

So far, it wasn't an easy ride. The first idea was to build a simple for-loop and `.or()` the filters together.
THIS DIDN'T WORK. Gladly, i stumbled upon a forum post in the `rust-lang.org` forums (See commit e91650b90b20a045fb857f1fef97c97d6d3796d7).
I found a solution there which uses the `and_then()` function of warp to check whether a given value is stored in our paths `Vec<String>`.

I played around a little bit and could format a costum response depending on the path accessed.

There is still much work to do. 

###  Build generic db methods
This step can be tackled as soon the config-File reader and the warp route construction from the configs works.

Right now, the Idea is following:
One File per database table. Each `.toml` files describe 1 table in the database.

###  dockerize it for simple deployment
This step is the cherry on the top, and will be done in the end when a solid foundation is laid.

## General problems so far:
- How to describe a ressource like `owner/5149/dogs` to list all dogs of the owner in the configs? 