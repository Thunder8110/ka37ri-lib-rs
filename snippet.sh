cargo snippet --type vscode > snippet_generated.json
jq -s add snippet_manual.json snippet_generated.json > snippet.json

snippet_path="/mnt/c/Users/kaminari/AppData/Roaming/Code/User/snippets/rust.json"
if [ -n "$1" ] && [ $1 = "update" ] && [ -f $snippet_path ]; then
  cp snippet.json $snippet_path
fi
