<div align="center">
  <img src="image.png" alt="rakoon" width="300">
</div>

- - - 

This is a transaction fuzzer for the Ethereum Protocol, with per-transaction custom mutators, an user interface for seeing live data from the fuzzer and a terminal to ineract with it in real time. Huge thanks to [Marius van der Wijden](https://github.com/MariusVanDerWijden) for building [tx-fuzz](https://github.com/MariusVanDerWijden/tx-fuzz), which I used as reference in many parts of this project.

## Usage

It is as simple as doing

```shell
./rakoon
```

and the user interface will pop-up. If there is no binary in the root of the project (it should be if I'm not stupid), then run

```shell
make build
```

and it will be there.

### Commands

The following commands are available in the terminal interface:

#### Set Configuration
- `set global rpc <URL>` - Set the global RPC URL
- `set global sk <private_key>` - Set the global private key
- `set global seed <number>` - Set the global seed
- `set global happy <true/false>` - Set the global happy mode

- `set <runner> rpc <URL>` - Set RPC URL for a specific runner
- `set <runner> sk <private_key>` - Set private key for a specific runner
- `set <runner> seed <number>` - Set seed for a specific runner
- `set <runner> happy <true/false>` - Set happy mode for a specific runner

Where `<runner>` can be one of `al`, `blob`, `eip1559`, `eip7702`, `legacy`, `random`

#### Reset Configuration
- `reset global all` - Reset all global configuration
- `reset global rpc` - Reset global RPC URL
- `reset global sk` - Reset global private key
- `reset global seed` - Reset global seed
- `reset global happy` - Reset global happy mode

- `reset <runner> all` - Reset all configuration for a specific runner
- `reset <runner> rpc` - Reset RPC URL for a specific runner
- `reset <runner> sk` - Reset private key for a specific runner
- `reset <runner> seed` - Reset seed for a specific runner
- `reset <runner> happy` - Reset happy mode for a specific runner

#### Runner Control
- `start` - Start all runners
- `start <runner>` - Start a specific runner
- `stop` - Stop all runners
- `stop <runner>` - Stop a specific runner

#### Other Commands
- `exit` - Exit the application


# TODO

Tienes que añadir más mutatos, como por ejemplo auths duplicadas, auths duplicadas de diferentes usuarios, signatures off the curve, access lists con duplicados...

Tambien solo fuzzea valid tx types, deberias poder mandar types random

El counter de txs sent no funciona/no implementado

Tienes que implementar happy path para todos