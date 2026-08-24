# Snipe

Snipe is a lightweight, fast, precise CLI HTTP client. The idea of the tool is to configure HTTP requests in a `.toml` file then use the CLI tool to make requests and tweak output/formatting options.

## Quickstart

By default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example configuration file is shows below:

```toml
[targets.create-gist]  # Create an API request with id 'create-gist'
name = "Create Gist"  # Name field is optional
method = "POST"
url = "https://api.github.com/gists"
timeout_seconds = 10  # Timeout of 10 seconds on the request

[targets.create-gist.headers]  # Headers for 'create-gist' request
User-Agent = "snipe"

[targets.create-gist.auth]  # Auth for 'create-gist' request
scheme = "bearer"
token = "${ENV.GITHUB_PAT}"  # Use the value stored at the GITHUB_PAT env var for the API token

[targets.create-gist.payload]  # Payload for 'create-gist` request
description = "Test gist"
public = false
files = {
    "test.txt" = {
        content = "Testing Gist"
    }
}
```

Once the configuration file is created in your present working directory the following command will show the potential HTTP requests to make:

```sh
# Outputs: create-gist
snipe list
```

To make the `create-gist` request run:
```sh
# Outputs the response body
snipe shoot create-gist
```

## Walkthrough

The following sections walkthrough how to use snipe and all the functionality the tool provides.

### Config System

The idea behind `snipe` is to configure HTTP requests in config files rather than inlining them in the shell. Snipe does offer some shell inlining capabilities to override config settings. Those capabilities will be disscussed later.

An example of a `snipe` configuration file with much functionality on display is shown below.

```toml
[vars]  # Reusable STRING variables. Get interpolated via `${VARS.variable_name}` syntax.
github_api_base_url = "https://api.github.com"
gist_id = "35f936fa7e75518b3fcd5e669d223479"
payloads_dir = ".snipe/payloads"

[globals.headers]  # Reusable headers. Get interpolated by `headers = { global = global_header_name }` syntax.
user_agent = { User-Agent = "snipe" }

[globals.payload.create_gist]  # Reusable payloads. Get interpolated by `payload = { global = global_payload_name }` syntax.
description = "Test gist"
public = false
files = { "test.txt" = { content = "Testing Gist" } }

[globals.auth]  # Reusable auth. Get interpolated by `auth = { global = global_auth_name }` syntax.
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }  # Use `${ENV.ENV_VAR_NAME}` to insert environment variables into the cfg file.

[targets.create-gist]  # Define a create-gist HTTP request.
name = "Github Gist"  # Name the request "GitHub Gist". Note the name field is optional. If omitted the request id will be used (create-gist in this example).
method = "POST"  # Make a POST request.
url = "${VARS.github_api_base_url}/gists"  # The URL to post to. Use the `github_api_base_url` variable defined above.
timeout_seconds = 10  # Add 10 second timeout to the request.
headers = { global = "user_agent" }  # Use the `global.headers.user_agent` defined above.
auth = { global = "github" }  # Use the `globals.auth.github` auth field defined above.
payload = { global = "create_gist" }  # Use the `globals.payload.create_gist` payload field defined above.
output_cfg = { format = "pretty_json" }  # Print the result to the console with pretty-json formatting

[targets.get-gists]  # Define a get-gists HTTP request.
method = "GET"  # Make a GET request.
url = "${VARS.github_api_base_url}/gists"  # The URL to get from. Use the `github_api_base_url` variable defined above.
timeout_seconds = 10  # Add 10 second timeout to the request.
headers = { global = "user_agent" }  # Use the `global.headers.user_agent` defined above.
auth = { global = "github" }  # Use the `globals.auth.github` auth field defined above.
output_cfg = { format = "json", pretty = true }  # Print the result to the console with pretty-json formatting

[targets.delete-gist]  # Define a delete-gist HTTP request.
method = "DELETE"  # Make a DELETE request.
url = "${VARS.github_api_base_url}/gists/${VARS.gist_id}"  # The URL for the delete operation. Use the `github_api_base_url` and `gist_id` variables defined above.
headers = { global = "user_agent" }  # Use the `global.headers.user_agent` defined above.
auth = { global = "github" }  # Use the `globals.auth.github` auth field defined above.
output_cfg = { grab = ["status_code"] }  # Only display status code to the console for the delete operation.
```

Once the above requests are defined the can simply be called in the following manner:

```sh
# Assuming the above config file is in a .snipe_targets.toml file in your present working directory...

# List all the targets defined in the file
snipe list

# Create a gist returning the response payload as pretty json printed to the console
snipe shoot create-gist

# Get all gists returning the response payload as pretty json printed to the console
snipe shoot get-gists

# Delete gist that was just created only displaying the status code to the console
# Note - this request will require the gist_id var (e.g. ${VARS.gist_id})
# in the config to match the id of the gist that was crated
snipe shoot delete-gist
```

#### The `[vars]` Field

The `[vars]` field in the config defines reusable ***string*** variables. Anything placed this field will be interpolated by snipe via the `${VARS.var_name}` syntax. An example is show below:

```toml
[vars]  # Reusable STRING variables. Get interpolated via `${VARS.variable_name}` syntax.
github_api_base_url = "https://api.github.com"
gist_id = "35f936fa7e75518b3fcd5e669d223479"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.auth]
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

[targets.delete-gist]
method = "DELETE"
url = "${VARS.github_api_base_url}/gists/${VARS.gist_id}"  # The URL for the delete operation. Use the `github_api_base_url` and `gist_id` variables defined above.
headers = { global = "user_agent" }
auth = { global = "github" }
output_cfg = { grab = ["status_code"] }
```

#### The `[globals.headers]` Field

The `[globals.headers]` field allows for reuse of headers across requests. It's simply key-value pairs denoted by a top level key that indicates the id of the headers. Note global headers do not have to be used, they can be inlined. Examples shown below:

```toml
[vars]
github_api_base_url = "https://api.github.com"

[globals.headers]  # Global headers for reuse
user_agent = { User-Agent = "snipe" }

[globals.auth]
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

# Use a global header value
[targets.get-gists-global]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }  # Use the `global.headers.user_agent` configuration defined above.
auth = { global = "github" }

# Inline the header value
[targets.get-gists-inline]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { User-Agent = "snipe" }  # Inline the headers value. Do not use the header at `global.headers.user_agent`
auth = { global = "github" }
```

#### The `[globals.payload]` Field

The `[globals.payload]` field allows for reuse of payloads across requests. It's simply a map denoted by a top level key that indicates the id of the payload. Note global payloads do not have to be used, they can be inlined. Moreover, payloads can be pointed to a `json` file as well. That file will be read and its contents used for the payload. Examples shown below:

```toml
[vars]
github_api_base_url = "https://api.github.com"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.auth]
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

[globals.payload.create_gist]  # Inline payload for reuse
description = "Test gist"
public = false
files = { "test.txt" = { content = "Testing Gist" } }

[globals.payload.create_gist_file]  # Payload housed in a json file for reuse
file = "create_gist.json"

# Use a global payload value
[targets.create-gist-global]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
payload = { global = "create_gist" }  # Use the `globals.payload.create_gist` configuration defined above.
output_cfg = { format = "pretty_json" }

# Inline the payload value
[targets.create-gist-inline]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
output_cfg = { format = "pretty_json" }
payload = {  # Inline the payload
  description = "Test gist",
  public = false,
  files = {
    "test.txt" = {
      content = "Testing Gist"
    }
  }
}

# Point the payload to a json file inline
[targets.create-gist-inline-file]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
payload = { file = "create_gist.json" }  # Use the payload defined in create_gist.json relative to the current present working directory
output_cfg = { format = "pretty_json" }

# Use a global payload that points to a json file
[targets.create-gist-inline-file]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
# Use the `globals.payload.create_gist_file` payload which points to a json file containing the payload to use
# Again, payload is relative to the current present working directory
payload = { global = "create_gist_file" }
output_cfg = { format = "pretty_json" }
```

#### The `[globals.auth]` Field

The `[globals.auth]` field allows for re-use of auth across requests. Note global auth does not have to be used an can be inlined. `Bearer` and `Basic` auth schemes are allowed. Examples shown below:

```toml
[vars]
github_api_base_url = "https://api.github.com"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.auth]  # Global bearer auth for re-use
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }
# Note GitHub uses bearer auth. This auth will fail if actually used. Showing for example purposes.
github_basic = { scheme = "basic", username = "github_username", password = "${ENV.GITHUB_PASSWORD}" }

# Use a global auth value
[targets.get-gists-global]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }

# Inline the auth value
[targets.get-gists-inline]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }  # Inline the auth value.

# Basic auth inline example
# Note GitHub uses bearer auth
# This request will fail if actually attempted to be used
[targets.get-gists-inline-basic]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { scheme = "basic", username = "github_username", password = "${ENV.GITHUB_PASSWORD}" }  # Inline the basic auth value.

# Basic auth global example
# Note GitHub uses bearer auth
# This request will fail if actually attempted to be used
[targets.get-gists-inline-basic]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github_basic" }  # Use the [global.auth.github_basic] value
```

#### The `outpug_cfg` field

The `output_cfg` field controls how the response output will be presented/formatted. Note - the CLI allows for overriding any value in the `output_cfg` field. The idea is response formatting settings can be saved in the `output_cfg` field but can be overriden quickly via the CLI for fast iteration. Basically, long term response settings -> `output_cfg` and short term development response settings -> CLI. The values layer between the `output_cfg` and the CLI. So any fields in the config but not overriden in the CLI will be used. A fully defined `output_cfg` is shown below. Some settings invalid with one another or must be present when others are present. Those combination are called out below.

```toml
# Assume other configurations for the `output_cfg` target already present
[targets.create_gist.output_cfg]
# How the response should be formatted
# Other possible values are "http", "json", "binary"
# Defaults to "http"
format = "pretty_json"

# Boolean for pretty printing.
# Only valid with "json" format. Passthrough for "pretty_json" format
# Fails for "http" and "binary" format
# Defaults to `false`
pretty = false

# What components to grab from the response
# Options are "status_code", "headers", "body", "full" (status_code, headers, and body), and int_status_code (status code as u16 integer)
# If "full" or "int_status_code" are specified no other options can be specified
# Any combination of "status_code", "headers", "body" is valid
# If `format = "binary"` is specified only the response body can be grabbed
# Defaults to "body"
grab = [
    "status_code",
    "headers",
    "body",
    # "full",  # Must be specified by itself, Equivalent to what is specified above
    # "int_status_code",  # Must be specifed by itself
]

# An optional file to output the response to
# If omitted the response will printed to the console
# If `format = "binary"` is specified an output file must also be specified
output_file = "response.json"

# Boolean for a dry run
# If set to true the request will be attempted to be built from the config/CLI args and not sent
# Defaults to false
dry_run = false
```

#### Other Fields

There other fields that must be present are described below. They're pretty self-explanatory but necessary and worth calling out.
- `name`: The name of the target. Only used for logging. If omitted the `id` of the target will be used.
- `url`: The url the response should hit.
- `method`: One of `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`. Caps agnostic (e.g. `get` and `GET` and `GeT` all work).
- `timeout_seconds`: Optional timeout (seconds) for the request. Useful so request doesn't hang. By default `snipe` will wait forever for a request.

Examples of the above fields shown below:

```toml
[targets.get-gists]
name = "Get Gists"  # Name the target "Get Gists" - only used for logging
url = "https://api.github.com/gists"  # URL the request should ping
method = "GET"  # Make a GET request
timeout_seconds = 10  # 10 second timeout on the request
headers = { User-Agent = "snipe" }
auth = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }
```

## Example Usage

Below are some more detailed examples of `snipe` usage. Note - help for these setting can seen by simply using the `--help` command on the `snipe` CLI. For example:

```sh
snipe --help  # Help for snipe
snipe list --help  # Help for the `list` command
snipe shoot --help  # Help for `shoot` command - Note this is the command used for actually making HTTP requests
```

### Sending Requests and Formatting Output

By default snipe will display the response body to `stdout`. However, flags can be passed for status code, body, headers, full, and int status code to customize the output. There are some constraints with which combos can be passed together which are shown below.

#### Grabbing Specific Response Components

`snipe` allows for grabbing specific components from a response. Some response components cannot be grabbed at the same time. Examples below.

```sh
# Valid args
snipe shoot request-id-from-cfg --status-code  # Only status code (e.g. 200 OK)
snipe shoot request-id-from-cfg --headers  # Only headers
snipe shoot request-id-from-cfg --body  # Only body (default)
snipe shoot request-id-from-cfg --int-status-code  # Status code integer. (e.g just 200)
snipe shoot request-id-from-cfg --full  # Status code (e.g. 200 OK), headers, body
snipe shoot request-id-from-cfg --status-code --headers  # Status code (e.g. 200 OK) and headers
snipe shoot request-id-from-cfg --status-code --body  # Status code (e.g. 200 OK) and body
snipe shoot request-id-from-cfg --headers --body  # Headers and body

# Invalid args
snipe shoot request-id-from-cfg --status-code --full  # ERROR! => --full cannot be passed with any other flags
snipe shoot request-id-from-cfg --status-code --int-status-code  # ERROR! => --int-status-code cannot be passed with any other flags
```

If no formatting args are passed after the desired target, the response body is grabbed. For example the following will return only the response body:

```sh
# Returns only response body
snipe shoot request-id-from-cfg
```

#### Tweaking response format

The `--format` argument allows for changing response output format. Currently there are three options `http` (default), `json`, and binary. The `http` return the output as an `http` string in the format:

```sh
status_code
headers

body
```

Combos will follow a similar format. For example `snipe shoot request-id-from-cfg --status-code --body` will return:

```sh
status_code

body
```

While the combo `snipe shoot request-id-from-cfg --status-code --headers` will return:

```sh
status_code
headers
```

***Note - if the the response cannot be parsed into a string the snipe will emit an error.***

The `json` format will attempt to parse the response into a json string. If multiple response fields are requested the output is formatted something like:

```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
  "body": "<response_body>",
}
```

Similarly - combos can be used. For example `snipe shoot request-id-from-cfg --status-code --body --format json` will return:

```json
{
  "status_code": "<response_status_code>",
  "body": "<response_body>",
}
```

If a single response field is requested, the key indicating the field is omitted. For example, the combo `snipe shoot request-id-from-cfg --format json` will return (remember by default only the response body is returned):

```json
{
  "<response_body>"
}
```

***Note - if the the response cannot be parsed into a json string snipe will emit an error.***

Parsing to json also has a `--pretty` flag which can be used to make the output more readable. `--pretty` is not valid with `--format http` or `--format binary` and if one of these combinations is passed snipe will emit an error. An examples of using `--pretty` are shown below:

```sh
# Valid args
snipe shoot request-id-from-cfg --full --format json --pretty

# Invalid args
snipe shoot request-id-from-cfg --full --format http --pretty # ERROR! => Can't pass --format http with --pretty
snipe shoot request-id-from-cfg --format binary --pretty # ERROR! => Can't pass --format binary with --pretty
snipe shoot request-id-from-cfg --full --pretty # ERROR! => By default snipe uses --format http which is invalid with --pretty
```

`snipe` can output a response to a file using the `--output-file` argument. An example of writing a full response in pretty printed json to a file is shown below:

```sh
snipe shoot request-id-from-cfg --full --format json --pretty --output-file full-response.json
```

If the response body is binary, `snipe` can handle this situation using the `--format binary --output_file <OUTPUT_FILE>` args. The `--format binary` arg must be used with the `--output-file` flag and is only valid with the `--body` flag. An example for grabbing a zip file from a response body is shown below.

```sh
# Note --body is passed by default
snipe shoot request-id-from-cfg --format binary --output-file some_file.zip
```

### Uploading a File as the Response Body

`snipe` allows for appending a file as a response body. For example, using the `create-gist` target from the quickstart, the body could be stored in a `json` file that the snipe configuration file could be updated to look for. For example the `json` file could look like:

```jsonc
// create-gist.json
{
  "description": "Test gist",
  "public": false,
  "files": {
    "test.txt": {
      "content": "Testing Gist"
    }
  }
}
```

Then the snipe configuration file could be pointed to this json (see the `[targets.create-gist.payload]` section):

```toml
[vars]  # Reusable variables
github_base_url = "https://api.github.com"

[targets.create-gist]  # Create an API request with id 'create-gist'
name = "Create Gist"
method = "POST"
url = "${VARS.github_base_url}/gists"  # Reuse the variable github_base_url from [vars]
timeout_seconds = 10  # Timeout of 10 seconds on the request

[targets.create-gist.headers]  # Headers for 'create-gist' request
User-Agent = "snipe"

[targets.create-gist.auth]  # Auth for 'create-gist' request
scheme = "bearer"
token = "${ENV.GITHUB_PAT}"  # Use the value stored at the GITHUB_PAT env var for the API token

[targets.create-gist.payload]  # Payload for 'create-gist` request
file = "create-gist.json"  # Point to the create-gist file which will be added as the response body
```

Now the `create-gist.json` file will be used for the response body. Note - the files are uploaded as bytes allowing for any kind of file to be appened to the HTTP request not just JSON or text data.

When configuring the response body, either a file can be used (seen above) or parameters can be specified in the `snipe` configuration file (like in the `Quickstart`) _but not both at the same time._ For example, the below configuration will produce an error:

```toml
#...Snipe configurations above omitted...

[targets.create-gist.payload]  # Payload for 'create-gist` request

# INVALID - CANNOT POINT TO A FILE AND CONFIGURE PARAMS IN THE TOML AT THE SAME TIME
file = "create-gist.json"  # Point to the create-gist file which will be added as the response body

# INVALID - CANNOT CONFIGURE PARAMS IN THE TOML AND POINT TO A FILE AT THE SAME TIME
description = "Test gist"
public = false
files = {
    "test.txt" = {
        content = "Testing Gist"
    }
}
```

### Changing the Path to the Configuration File

Use the the `--config` (`-c`) argument to change the path of the configuration file. As stated above by default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of using a different config looks something like the following:

```sh
snipe --config ~/.config/snipe/snipe_targets.toml shoot request-id-from-cfg
```

If the config cannot be found, snipe will fall back to attempting to read the `SNIPE_TARGETS` environment variable which houses the path to the config. The environment variable to look for can be tweaked with the `--cfg-env` (`-e`) arg. Searching for the environment variable can be skipped entirely by passing `skip` for this argument. For example:

```sh
snipe --cfg-env SNIPE_CONFIG shoot request-id-from-cfg  # If can't find the config at ./.snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --config ~/.config/snipe/snipe_targets.toml --cfg-env SNIPE_CONFIG shoot request-id-from-cfg  # If can't find the config at ~/.config/snipe/snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --cfg-env skip shoot request-id-from-cfg  # Skip an environment variable to identify the config
```

### Verbosity

Use `--verbose` (`-v`) for verbose mode. This mode will set the log level to info. Use `-vv` to set the log level to debug. Note, any number of `v` arguments can be passed. However, two or more arguments will simply set the log level to debug. For example:

```sh
snipe --verbose shoot request-id-from-cfg  # Log level at info
snipe -v shoot request-id-from-cfg  # Log level at info
snipe -vv shoot request-id-from-cfg  # Log level at debug
snipe -vvv shoot request-id-from-cfg  # Log level at debug
snipe -vvvvv shoot request-id-from-cfg  # Log level at debug
```
