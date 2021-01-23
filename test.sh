#!/bin/bash
ESC=$(printf '\033')
EXPECTED_OUTPUT_FILE=".expected_output_for_test"
ACTUAL_OUTPUT_FILE=".actual_output_for_test"

clean() {
    rm $EXPECTED_OUTPUT_FILE $ACTUAL_OUTPUT_FILE
}

for class_file in samples/*.class; do
    printf "${ESC}[32m%s${ESC}[m%s\n" 'Running: ' "${class_file}"
    class_name=$(basename -- "$class_file" | cut -d '.' -f 1)
    (cd samples && java "$class_name" > "../$EXPECTED_OUTPUT_FILE")
    if cargo run "$class_file" > $ACTUAL_OUTPUT_FILE; then
        if diff -q $EXPECTED_OUTPUT_FILE $ACTUAL_OUTPUT_FILE >/dev/null ; then
            printf "${ESC}[32m%s${ESC}[m\n\n" "Test ${class_file} successful."
        else
            printf "${ESC}[31m%s${ESC}[m\n" "Error: ${class_file}"
            clean
            exit 1
        fi
    else 
        printf "${ESC}[31m%s${ESC}[m\n" "Error: ${class_file}"
        clean
        exit 1
    fi
done
clean
