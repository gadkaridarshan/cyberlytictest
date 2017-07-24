# cyberlyticdaemon
To use this, please download the repository and then execute "cargo run" in the base folder

To execute either the running or stopped for the application, please send the JSON to the following endpoint:
http://localhost:8000/message

Here is a sample JSON for "running":
{
	"command":"[cp, src/main.rs, src/main5.rs]",
	"cwd": ".",
	"state": "running"
}

Here is the sample JSON for "stopped":
{
	"command":"[kill, -9, <any current running app>]",
	"cwd": ".",
	"state": "stopped"
}
