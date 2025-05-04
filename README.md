# rakoon

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

and it will be there. The possible commands are the following ones:

- TODO

La idea es que puedas lanzar y apagar fuzzers con seeds arbitrarias sin apagar el programa y se añada una linea con la info de cada fuzzer cuando se active uno u otro. Si stop, todos mueren. Deberías poner seeds per runner y globales.

Tienes que añadir más mutatos, como por ejemplo auths duplicadas, auths duplicadas de diferentes usuarios, signatures off the curve, access lists con duplicados...