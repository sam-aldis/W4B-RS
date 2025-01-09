# W4B
---

W4B is a project working with AI. At the moment a simple command line
interface has been made but in future a way of communicating with hosts
over the internet and displaying the results with AI designed HTML5 will
be implemented.

current working idea for remote communication means that users will be
able to interact with the AI and query it instead of seeing a static webpage.

to work the an API key for Gemini needs to be added in the src folder to a file
called secrets.rs as follows:
```rust

pub fn getAPIKey() -> String {
	let API_KEY: String = String::from("<ADD KEY HERE>");
	return API_KEY;
}
```

## LibAPI
Libapi has been created to facilitate both the creation of the terminal cli, tui and the final GUI.
see more at: [libapi](./LIBAPI.md)