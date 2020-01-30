# chiya

![](https://github.com/Tosainu/chiya/workflows/CI/badge.svg)

Toy programming language and compiler.

## Language design

```c
// This is a comment.

ptr += 123; // add 123 to the data pointer
ptr -= 123; // subtract 123 from the data pointer

*ptr += 123; // add 123 to the value that 'ptr' points to
*ptr -= 123; // subtract 123 from the value that 'ptr' points to

putchar(); // writes *ptr to stdout
getchar(); // reads the next character from stdin and stores it to *ptr

// while loop
while *ptr {
    // ...
}
```

## Example

    $ cargo run -q <<EOS > ex.ll
    *ptr += 65;
    putchar();
    *ptr += 1;
    putchar();
    *ptr += 1;
    putchar();
    ptr += 1;
    *ptr += 10;
    putchar();
    EOS

    $ llc ex.ll
    $ clang ex.s

    $ ./a.out
    ABC

## License

[MIT](https://github.com/Tosainu/chiya/blob/master/LICENSE)
