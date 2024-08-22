# Wordfinder

Search for words. (currently supports only Dutch language) 


## Usage
## Commands

### `help`

Displays usage instructions for available commands.

### `find *****`
Searches for word patterns. * represents unknown characters. The total number of characters determines the length of the words in the search dictionary.
e.g. find `**a**` or find `au*io`

### `include [characters]`

Adds characters to the `include_chars` set. Prints the current value if no argument is provided.


### `exclude [characters]`

Adds characters to the `exclude_chars` set. Prints the current value if no argument is provided.

### `list [pattern]`

Lists words from the dictionary in columns. Optionally filter words by `pattern`.

### `reset [include|exclude]`

Resets `include_chars` or `exclude_chars`. Resets the dictionary and both sets if no argument is provided.

### `exit`

Exits the application.
