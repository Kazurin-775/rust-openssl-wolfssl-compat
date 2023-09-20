#!/usr/bin/env python3

import sys, re

def is_protocol_method(name):
    return name.endswith('_method') and (
            name.startswith('SSL') or
            name.startswith('TLS') or
            name.startswith('DTLS')
    )


def mangle(name):
    name = name.removesuffix('__fixed_rust')
    if name == 'SSL_get_rbio':
        return 'wolfSSL_SSL_get_rbio'
    if name.startswith('SSL_') or is_protocol_method(name):
        return 'wolf' + name
    else:
        return 'wolfSSL_' + name


if len(sys.argv) != 2:
    print('Usage: rename-symbols.py <file>.rs')
    sys.exit(1)

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()

output = ''
fn_name_re = re.compile('pub fn (\w+)')
indent_re = re.compile(r'^\s*')

for line in lines:
    if line.strip().startswith('#[link_name'):
        print('Overriding:', line.strip())
        continue
    if line.strip().startswith('pub fn '):
        fn_name = fn_name_re.search(line)[1]
        indent = indent_re.match(line)[0]
        #print('#[link_name = "' + mangle(fn_name) + '"]')
        output += indent + '#[link_name = "' + mangle(fn_name) + '"]\n'
    output += line

#print(output)

with open(sys.argv[1], 'w') as f:
    f.write(output)
