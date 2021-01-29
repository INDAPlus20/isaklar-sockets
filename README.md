# isaklar-sockets

The networking is completely implemented in the `game_state` module.

It's only p2p on the local network. To run, either use the .exe or `cargo run` with two arguments: `$ cargo run [host/connect] [adress]`    
For example: `$ cargo run host 127.0.0.1:8787`

*Minor bug exclaimer: The clients will desync if they are not sending inputs to eachother*
