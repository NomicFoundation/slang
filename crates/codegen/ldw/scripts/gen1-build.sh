#!/usr/bin/env zsh

CLI=src-ts/cli/ldw.ts

echo

# for grammar in ldw::grammar::parsed ldw::model::parsed ; do
#     echo "    Processing grammar $grammar"
#     ts-node $CLI process-grammar -r $REGISTRY -n $grammar
# done

    # ldw::grammar::parsed ldw::grammar::extended ldw::grammar::typed \
for model in \
    ldw::model::parsed ldw::model::resolved ldw::model::analysed \
    ; do
    echo "    Processing model $model"
    ts-node $CLI process-model --in-dir src-ldw --out-dir src-rs --language rust --name $model
done