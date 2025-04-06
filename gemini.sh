#!/bin/bash

PROMPT=$1

if [ -z "$1" ]
then
    echo "Usage: ./gemini.sh <PROMPT>"
    exit -1
fi

TMP_PROMPT_FILE=$(mktemp)

cat > "$TMP_PROMPT_FILE" << EOF
{
    "contents": [
      {
        "parts": [
          {
            "text": "$(echo $PROMPT)"
          }
        ]
      }
    ]
  }
EOF

curl --silent "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key=$GEMINI_API_KEY" \
  -H 'Content-Type: application/json' -X POST -d "@$TMP_PROMPT_FILE" | \
  jq '.candidates[].content.parts[].text' -r
