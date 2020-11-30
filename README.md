# make_opcodes

A CLI tool to generate a list of commands used in a script source.

## Usage

```
make_opcodes.exe <input> [--output <output>] [--classes <classes>] [--keywords <keywords>]
```

`<input>` - path to a text file

`--output <file path>` - path to a file where to save the result. The tool prints to stdout by default

`--keywords <file path>` - path to keywords.txt (keywords definitions)

`--classes <file path>` - path to classes.db (classes definitions)

## Licence

MIT