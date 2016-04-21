netlion
=======

`netlion` is a simple graphical network debugging tool. It allows you to create sockets and streams  the data sent to that socket.

`netlion` is built using [conrod](https://github.com/PistonDevelopers/conrod), which is still a work in progress. Rust stable compatibility is a goal, however this is not guaranteed until reaching 1.0, which will happen once:

- all basic functionality is done
- conrod stabilizes (or `netlion` switches to another toolkit)
- builds on Windows and Linux
- a cool logo is designed

Current features include:

- binding to tcp sockets
- binding to udp sockets
- printing data sent to the bound address

`netlion` aims to be a more convenient, cross-platform replacement for most use cases of `netcat`. The major features outstanding are:

- bind to multiple addresses at once (tab per bound socket?)
- open a socket to a remote server and send data back and forth
- support unix domain sockets
