#!/bin/bash

# $1: template file name
# $2: sub for file name
# $3: field type
# $4: sub for message name
# $5: sub for "//comment"
generate () {
    FILE_SUB=s/__file__/$2/g
    MESSAGE_SUB=s/__message__/$4/g
    FIELD_SUB=s/__field__/$3/g
    COMMENT_SUB="s/__comment__/$5/g"
    NEWNAME="$(echo $1 | sed \
        -e $FILE_SUB \
        -e 's/.protmpl/.proto/g'\
        -e 's/param_template/parameters/g'\
    )"
    echo $NEWNAME
     mkdir -p -- "$(dirname -- "$NEWNAME")"
    touch "$NEWNAME"
    
    cat $1 | \
      sed \
        -e $FILE_SUB\
        -e $FIELD_SUB\
        -e $MESSAGE_SUB\
        -e "$COMMENT_SUB"\
      > "$NEWNAME"
}

move() {
    NEWNAME="$(echo $1 | sed \
        -e 's/param_template/parameters/g'\
    )"
    cp $1 $NEWNAME
}
# import "entity/parameters/expr.proto";
find . -name *.protmpl | \
    while IFS= read -r line ; do
        generate $line double double Double "";
        generate $line expr Expr Expr "import \"entity\/expression.proto\";";
    done
