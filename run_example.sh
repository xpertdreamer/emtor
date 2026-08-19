#!/usr/bin/env bash

# References:
# 1. https://www.stevemar.net/use-a-dictionary-in-bash/
# 2. https://stackoverflow.com/questions/1494178/how-to-define-hash-tables-in-bash
# 3. https://www.baeldung.com/linux/use-command-line-arguments-in-bash-script#bd-bd-processing-the-input
# 4. https://stackoverflow.com/questions/18544359/how-do-i-read-user-input-into-a-variable-in-bash
# 5. https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script

declare -A examples
examples=(["add"]="examples/add.emt" ["memory"]="examples/mem.emt" ["jct"]="examples/branching.emt" ["jor"]="examples/branching2.emt" ["jmp"]="examples/branching3.emt"
         ["comments"]="examples/comment.emt")
PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
cd "$PROJECT_ROOT"
TESTNAME=$1
REBUILD=$2
if [[ "$#" -ne 2 || ! -v examples["$1"] ]]; then
    echo -e "Usage: $0 <test-name> <rebuild>"
    echo "  rebuild (y/n) - Pass 'y' if you want to rebuild entire project before example run"
    echo ""
    echo "Currently available tests:"
    for i in "${!examples[@]}"
    do
        echo "  $i=${examples[$i]}"
    done
    exit 1
fi

if [ "$REBUILD" == "y" ]; then
    echo "Rebuilding entire project"
    make -B
else
    echo "Rebuilding denied, running test ${TESTNAME}"
fi

./target/emtor "${examples[$TESTNAME]}"
