# tui.rs

This file implements a basic Text-based User Interface (TUI) for interacting with a language model, interacting with the Gemini API.

## Purpose

Essentially, `tui.rs` provides a simple command-line interface for:

*   Interacting with a language model.
*   Maintaining a history of the conversation.
*   Displaying the responses in a user-friendly way with colored prompts.

## Functionality

Here's a breakdown of the code's functionality:

1.  **Initialization:**
    *   Imports necessary modules, including:
        *   `secrets` (likely for API keys)
        *   `api` (for interacting with the language model API)
        *   `std::io` (for input/output)
        *   `gemini_rs::Conversation` (for managing the conversation with the language model)
        *   `colored::Colorize` (for adding colors to the output)

2.  **Main Loop:**
    *   Enters an infinite loop (`loop`) to continuously interact with the user.
    *   Prompts the user for input with a red "**>>>>**" symbol.
    *   Reads the user's query from standard input.
    *   Stores the user's query in a `history` vector.
    *   Constructs a prompt for the language model, including the conversation history.
    *   Sends the prompt to the language model using the `gemini_rs::Conversation` object.
    *   Prints the language model's response.
    *   Adds the language model's response to the `history` vector.