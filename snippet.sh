cargo snippet --type vscode > snippet_generated.json
jq -s add snippet_manual.json snippet_generated.json > snippet.json
