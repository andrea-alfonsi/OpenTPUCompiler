Compiler for gemma project

# Warnings 
Calling `cli` may result in empty output if not followd by `| hexdump -C` or `> FILE` because termminal doesn't render non ascii chars

Call `cli -i` for interactive mode

If not present create a filecalled `logging.yml` in the current directory before launching the command

# Run code in simulator
The output result can be given to the simulator located in the [OpenTPUHardware Project](https://github.com/andrea-alfonsi/OpenTPUHardware)
or the [OpenTPUDriver Project](https://github.com/andrea-alfonsi/OpenTPUDriver)