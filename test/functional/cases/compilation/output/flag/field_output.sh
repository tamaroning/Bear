#!/usr/bin/env sh

# REQUIRES: shell, dynamic-shell
# RUN: cd %T; %{bear} -vvvv --cdb %t.json --field-output -- %{shell} %s
# RUN: assert_compilation %t.json count -eq 2
# RUN: assert_compilation %t.json contains -file field_output_1.c -output field_output_1.o -directory %T -arguments %{c_compiler} -c -o field_output_1.o field_output_1.c
# RUN: assert_compilation %t.json contains -file field_output_2.c -output field_output_2.o -directory %T -arguments %{c_compiler} -c -o field_output_2.o field_output_2.c

touch field_output_1.c field_output_2.c

$CC -c -o field_output_1.o field_output_1.c;
$CC -c -o field_output_2.o field_output_2.c;
