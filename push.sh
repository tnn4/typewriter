#!/bin/bash

remove_first_param() {
    first="$1"
    shift # see: https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html#index-shift-121
}

main() {
    # place stuff you want to run here
    echo "hello world!"
    head=$1
    tail="${@:2}"
    last="${@: -1}"

    echo "head: $head"
    echo "tail: $tail"
    echo "git add $tail"
    echo "git commit -m $head"
    echo "git push"
    git add "$tail"
    git commit -m "$head"
    git push
}

main "$@"
