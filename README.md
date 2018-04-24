# Artax

## Installation

Artax is currently only available on Windows. It requires ZeroMQ v4.2.3. To build from source, see
https://github.com/zeromq/libzmq/releases.

Clone or download this repo.

## Usage

To use Artax, open a terminal, navigate to the root directory, and run:

`cargo build`

Once the program finishes building, open 2 more terminals and navigate to the root directory. In the first terminal, enter and run:

`cargo run --bin server`

This will start running the simulated data server.

In the second terminal, enter and run:

`cargo run --bin artax`

This will start the Artax broker.

Finally, in the third terminal, enter and run:

`cargo run --bin client <analysis_type>`

Possible analysis types available:
* reduction

More coming soon:
* resolving power
* thrash
* msms

This will start running the client, asking Artax to perform the chosen analysis of the data.
