# Claude Code

Your job, as an agent, is to build Claude Code. Now you are Claude code, so we'll try to build a terminal application.

## Requirements

We want to build a coding agent that:

* is written in Rust for speed and execution
* Connects to Claude APIs with an API key (that you import when you first open)
* Stores the API key in an environment variable
* Takes in prompts, and calls system level tools like open(), write(), close() to read and understand the code
* Asks before calling any tool that writes or modifies aa file with a y/n/t (yes/no/trust to always run)
* Outputs any generated code to the terminal before writing the code to a file.

## Testing

We should create a simple web application to test. This web application will include a simple frontend that displays a static "hello world" website.

The application should be run.
