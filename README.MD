# Emoji Compression

A rust program designed to parse through text and replace certain strings with emojis.

## Goals

1. Generate a formatted list of every emoji with descriptions and unicode.

2. Submit the list to the OpenAI API and have GPT generate a map of unique strings for each emoji.

3. Create a parser that replaces strings with emojis.

4. Put it into a WASM module that can be used with JavaScript for forms/text inputs.