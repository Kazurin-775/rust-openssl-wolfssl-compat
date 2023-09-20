#!/usr/bin/env python3

# Usage: nm -gD libwolfssl.so | generate-shim.py > libwolfssl-shim.c

try:
    while True:
        line = input()
        fields = line.split()
        if len(fields) != 3 or fields[1] != 'T':
            continue
        func = fields[2]
        if func == '_init' or func == '_fini':
            continue
        print(f'void {func}() {{}}')
except EOFError:
    pass
