# Snipe

Snipe is a lightweight, fast, precise CLI HTTP client/collections storage tool. The idea of the tool is to configure HTTP requests and response formatting in a `toml` file and optionally `json` files then use the CLI  to make requests and tweak output/formatting options for fast development iteration.

## Quickstart

By default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example configuration file is shown below:

```toml
[targets.create-gist]  # Create an API request with id 'create-gist'
name = "Create Gist"  # Name field is optional
method = "POST"  # Make a POST request
url = "https://api.github.com/gists"  # URL to POST to
timeout_seconds = 10  # Timeout of 10 seconds on the request

[targets.create-gist.headers]  # Headers for 'create-gist' request
User-Agent = "snipe"

[targets.create-gist.auth]  # Auth for 'create-gist' request
scheme = "bearer"  # Use bearer auth
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

## Config System

The idea behind `snipe` is to configure HTTP requests in config files rather than inlining them in the shell. Snipe does offer some shell inlining capabilities to override config settings. Those capabilities will be disscussed later.

An example of a `snipe` configuration file with much functionality on display is shown below.

```toml
[vars]  # Reusable STRING variables. Get interpolated via `${VARS.variable_name}` syntax.
github_api_base_url = "https://api.github.com"
gist_id = "35f936fa7e75518b3fcd5e669d223479"
payloads_dir = ".snipe/payloads"

[globals.headers]  # Reusable headers. Get interpolated by `headers = { global = global_header_name }` syntax.
user_agent = { User-Agent = "snipe" }

[globals.payload.create-gist]  # Reusable payloads. Get interpolated by `payload = { global = global_payload_name }` syntax.
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
payload = { global = "create-gist" }  # Use the `globals.payload.create-gist` payload field defined above.
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

# List all the targets defined in the file to the console
snipe list

# Create a gist returning the response payload as pretty json printed to the console
snipe shoot create-gist

# Get all gists returning the response payload as pretty json printed to the console
snipe shoot get-gists

# Delete gist that was just created only displaying the status code to the console
# Note - this request will require the gist_id var (e.g. ${VARS.gist_id})
# in the config to match the id of the gist that was created
snipe shoot delete-gist
```

### The `[vars]` Field

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
# The URL for the delete operation
# Uses the `github_api_base_url` and `gist_id` variables defined above.
url = "${VARS.github_api_base_url}/gists/${VARS.gist_id}"
headers = { global = "user_agent" }
auth = { global = "github" }
output_cfg = { grab = ["status_code"] }
```

### Headers

The headers field simply defines the request headers. There are 2 ways headers can be added to the request. The `[globals.headers]` field allows for reuse of headers across requests. It's simply key-value pairs denoted by a top level key that indicates the id of the headers. Headers can also be inlined. Examples shown below:

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

### Payload

The Payload field defines the request payload. There are 4 ways a payload can be added to a request. The `[globals.payload]` field allows for reuse of payloads across requests. It's simply a map denoted by a top level key that indicates the id of the payload. Payloads can also be inlined. Lastly, payloads can be pointed to a `json` file as well (via a global or inline payload). That file will be read and its contents used for the payload. Examples shown below:

```toml
[vars]
github_api_base_url = "https://api.github.com"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.auth]
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

[globals.payload.create-gist]  # Inline payload for reuse
description = "Test gist"
public = false
files = { "test.txt" = { content = "Testing Gist" } }

[globals.payload.create-gist-file]  # Payload housed in a json file for reuse
file = "create-gist.json"

# Use a global payload value
[targets.create-gist-global]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
payload = { global = "create-gist" }  # Use the `globals.payload.create-gist` configuration defined above.
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
payload = { file = "create-gist.json" }  # Use the payload defined in create-gist.json relative to the current present working directory
output_cfg = { format = "pretty_json" }

# Use a global payload that points to a json file
[targets.create-gist-inline-file]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
# Use the `globals.payload.create-gist-file` payload which points to a json file containing the payload to use
# Again, payload is relative to the current present working directory
payload = { global = "create-gist-file" }
output_cfg = { format = "pretty_json" }
```

### Auth

Auth is a special headers field for request authorization. Both bearer and basic auth are supported and there are two ways to add auth to a request. The `[globals.auth]` field allows for re-use of auth across requests. Auth can also be inlined.

```toml
[vars]
github_api_base_url = "https://api.github.com"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.auth]  # Global auth for re-use
# Bearer auth - what GitHub actually uses
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

# Note GitHub uses bearer auth. This auth will fail if actually used. Showing for example purposes.
github_basic = { scheme = "basic", username = "github_username", password = "${ENV.GITHUB_PASSWORD}" }

# Use a global auth value
[targets.get-gists-global]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }  # Use [globals.auth.github] value

# Inline the auth value
[targets.get-gists-inline]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }  # Inline the bearer auth value

# Basic auth inline example
# Note GitHub uses bearer auth
# This request will fail if actually attempted to be used
[targets.get-gists-inline-basic]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { scheme = "basic", username = "github_username", password = "${ENV.GITHUB_PASSWORD}" }  # Inline the basic auth value

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

### The `outpug_cfg` field

The `output_cfg` field controls how the response output will be presented/formatted. Note - the CLI allows for overriding any value in the `output_cfg` field. The idea is response formatting settings can be saved in the `output_cfg` field but can be overriden quickly via the CLI for fast iteration. Basically, long term response settings -> `output_cfg` and short term development response settings -> CLI. The values layer between the `output_cfg` and the CLI. So any fields in the config but not overriden in the CLI will be used. A fully defined `output_cfg` is shown below. Some settings are invalid with one another or must be present when others are present. Those combinations are called out below.

```toml
# Assume other configurations for the `create-gist` target already present

[targets.create-gist.output_cfg]
# How the response should be formatted
# Possible values are "http", "json", "pretty_json", "binary"
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
# Any combination of "status_code", "headers", or "body" is valid
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

Below are examples of invalid `output_cfg` settings. ***NOTE - ALL THESE CONFIGURATIONS WILL FAIL IF USED.***

```toml
# Assume other configurations for all targets already present

[targets.create-gist.pretty_with_http]
# Cannot specify pretty output with http formatting
fomrat = "http"
pretty = true

[targets.create-gist.pretty_with_http_default]
# Cannot specify pretty output with http formatting
# `format = "http"` is the default that will be used if omitted
pretty = true

[targets.create-gist.pretty_with_binary]
# Cannot specify pretty output with http formatting
fomrat = "binary"
pretty = true

[targets.create-gist.full_with_other_response_components]
# grab = ["full"] must be specified by itself
grab = ["full", "body"]

[targets.create-gist.int_status_code_with_other_response_components]
# grab = ["int_status_code"] must be specified by itself
grab = ["int_status_code", "body"]

[targets.create-gist.binary_without_only_body]
# If specifying `format = "binary"` must specify to grab only the response body
format = "binary"
grab = ["status_code"]

[targets.create-gist.binary_without_output_file]
# If specifying `format = "binary"` must specify an output file
# Cannot print a binary response to the console
format = "binary"
```

### Other Fields

The other fields that must be present are described below. They're pretty self-explanatory but necessary and worth calling out.
- `name`: The name of the target. Only used for logging. If omitted the `id` of the target will be used.
- `url`: The url the request should hit.
- `method`: One of `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`. Caps agnostic (e.g. `get` and `GET` and `GeT` all work).
- `timeout_seconds`: Optional timeout (seconds) for the request. Useful so request doesn't hang. By default `snipe` will wait forever for a response.

Examples of the above fields shown below:

```toml
[targets.get-gists]
name = "Get Gists"  # Name the target "Get Gists" - only used for logging. If omitted `get-gists` (from [targets.get-gists]) will be used for the name
url = "https://api.github.com/gists"  # URL the request should ping
method = "GET"  # Make a GET request
timeout_seconds = 10  # 10 second timeout on the request
headers = { User-Agent = "snipe" }
auth = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }
```

## CLI

The `snipe` CLI can be used to override any of the `output_cfg` settings described above. As stated previously, the idea behine the CLI interface is for rapid development iteration.

### Grabbing Specific Response Components

Flags can be be passed to specify which response components to grab. As stated above some combinations are invalid. Examples below.

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

### Tweaking response format

The `--format` argument allows for changing response output format. Currently there are four options `http` (default), `json`, `pretty-json` and `binary`. The `http` returns the output as an `http` string in the format:

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

If a single response field is requested, the key indicating the field is omitted. For example, the combo `snipe shoot request-id-from-cfg --format json` will return (note by default only the response body is returned):

```json
{
  "<response_body>"
}
```

***Note - if the the response cannot be parsed into a json string snipe will emit an error.***

Parsing to json also has a `--pretty` flag which can be used to make the output more readable. `--pretty` is not valid with `--format http` or `--format binary` and if one of these combinations is passed snipe will emit an error. If `--pretty` is passed with `--format pretty-json` no action will be taken. Examples of using `--pretty` are shown below:

```sh
# Valid args
snipe shoot request-id-from-cfg --full --format json --pretty
snipe shoot request-id-from-cfg --full --format pretty-json --pretty

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

### Dry Run

Snipe allows for dry runs. The request defined by combination of the target config (from the configuration file) and they CLI args will be built and validated but not sent. Example:

```sh
snipe shoot request-id-from-cfg --format pretty-json --dry-run
```

### Changing the Path to the Configuration File

Use the the `--config` (`-c`) argument to change the path of the configuration file. As stated above by default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of using a different config looks something like the following:

```sh
snipe --config ~/.config/snipe/snipe_targets.toml shoot request-id-from-cfg
```

If the config cannot be found, snipe will fall back to attempting to read the `SNIPE_TARGETS` environment variable which houses the path to the config. The environment variable to look for can be tweaked with the `--cfg-env` (`-e`) arg. Searching for the environment variable can be skipped entirely by passing `skip` for this argument. For example:

```sh
snipe --cfg-env SNIPE_CONFIG shoot request-id-from-cfg  # If can't find the config at ./.snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --config ~/.config/snipe/snipe_targets.toml --cfg-env CFG_VAR shoot request-id-from-cfg  # If can't find the config at ~/.config/snipe/snipe_targets.toml use the value at the environment variable CFG_VAR
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

## Example Usage

Below show some examples for using `snipe`.

First define the config file we'll use:

```toml
[vars]
github_api_base_url = "https://api.github.com"
gist_id = "35f936fa7e75518b3fcd5e669d223479"
payloads_dir = ".snipe/payloads"

[globals.headers]
user_agent = { User-Agent = "snipe" }

[globals.payload.create-gist]
description = "Test gist"
public = false
files = { "test.txt" = { content = "Testing Gist" } }

[globals.auth]
github = { scheme = "bearer", token = "${ENV.GITHUB_PAT}" }

[targets.create-gist]
name = "Github Gist"
method = "POST"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
payload = { global = "create-gist" }
output_cfg = { format = "pretty_json" }

[targets.get-gists]
method = "GET"
url = "${VARS.github_api_base_url}/gists"
timeout_seconds = 10
headers = { global = "user_agent" }
auth = { global = "github" }
output_cfg = { format = "pretty_json" }

[targets.delete-gist]
method = "DELETE"
url = "${VARS.github_api_base_url}/gists/${VARS.gist_id}"
headers = { global = "user_agent" }
auth = { global = "github" }
output_cfg = { grab = ["status_code"] }
```

Next we'll create a gist using the default settings for this target to view the result in the console as pretty json:

```sh
# Create gist - view in console as pretty json as specified in the config file
snipe shoot create-gist
```

After creating the gist we'll inspect all the gists we have in pretty json and print them to the output file `gists.json` by using a CLI arg (e.g. the output file is not specified in the `targets.create-gist.output_cfg` settings):

```sh
# View all gists as pretty json and output them to gists.json
snipe shoot get-gists --output-file gists.json
```

Lastly we'll grab the id of our newly created gist, update the `vars.gist_id` variable, and override the grab setting to grab only the int status code:

```toml
# Snipe configuration file
[vars]
gist_id = "d0e3f2a4bb68195e4644310a94b2a2e8"  # Update the gist id to what was returned from create-gist
```

```sh
# Delete the newly created gist, only returning the int status code
# Note the config declares to return the `status_code` vs. the `int_status_code`
# We are overriding the config value via the CLI
snipe shoot delete-gist --int-status-code
```