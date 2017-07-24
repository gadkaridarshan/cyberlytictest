# cyberlyticdaemon
To use this, please download the repository and then execute "cargo run" in the base folder

To execute either the running or stopped for the application, please send the JSON to the following endpoint:
http://localhost:8000/message

Here is a structure of JSON for "running":
{
  command: ["/path/to/executable", "argument1", "argument2"],
  cwd: "/path/to/workdir",
  state: "running"
}

Here is the structure of JSON for "stopped":
{
  command: ["/path/to/executable", "argument1", "argument2"],
  cwd: "/path/to/workdir",
  state: "stopped"
}
