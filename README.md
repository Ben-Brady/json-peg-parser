# JSON Peg Parser

A JSON parser written in rust making using of pest.

This was my first venture into PEG based parsing after doing some more conventional parsing for my [simple scripting language](https://github.com/Ben-Brady/simple-scripting-language) project. Once I establsihed the grammar, writing the rest of the parser was quite easy as I could use the languages built-in convesions (string and floating point) to transform into language constructs as they had already been validated by the PEG grammar.
