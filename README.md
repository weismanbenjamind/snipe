# Snipe

Snipe is a lightweight, fast, precise CLI HTTP client. The idea of the tool is to configure HTTP requests in a `.toml` file then use the CLI tool to make requests and tweak output/formatting options.

## Quickstart

By default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example configuration file is shows below:

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
description = "Test gist"
public = false
files = {"test.txt" = {content = "Testing Gist"}}
```

Once the configuration file is created in your present working directory the following command will show the potential HTTP requests to make:

```sh
# Outputs: create-gist
snipe list-targets
```

To make the `create-gist` request run:
```sh
# Outputs the response body
snipe shoot --target create-gist
```

## Example Usage

Below are some more detailed examples of `snipe` usage. Note - help for these setting can seen by simply using the `--help` command on the `snipe` CLI. For example:

```sh
snipe --help  # Help for snipe
snipe list-targets --help  # Help for the `list-targets` command
snipe shoot --help  # Help for `shoot` command - Note this is the command used for actually making HTTP requests
```

### Sending Requests and Formatting Output

By default snipe will display the response body to `stdout`. However, flags can be passed for status code, body, headers, full, and int status code to customize the output. There are some constraints with which combos can be passed together which are shown below.

#### Grabbing Specific Response Components

`snipe` allows for grabbing specific components from a response. Some response components cannot be grabbed at the same time. Examples below.

```sh
# Valid args
snipe shoot --target request-id-from-cfg --status-code  # Only status code (e.g. 200 OK)
snipe shoot --target request-id-from-cfg --headers  # Only headers
snipe shoot --target request-id-from-cfg --body  # Only body (default)
snipe shoot --target request-id-from-cfg --int-status-code  # Status code integer. (e.g just 200)
snipe shoot --target request-id-from-cfg --full  # Status code (e.g. 200 OK), headers, body
snipe shoot --target request-id-from-cfg --status-code --headers  # Status code (e.g. 200 OK) and headers
snipe shoot --target request-id-from-cfg --status-code --body  # Status code (e.g. 200 OK) and body
snipe shoot --target request-id-from-cfg --headers --body  # Headers and body

# Invalid args
snipe shoot --target request-id-from-cfg --status-code --full  # ERROR! => --full cannot be passed with any other flags
snipe shoot --target request-id-from-cfg --status-code --int-status-code  # ERROR! => --int-status-code cannot be passed with any other flags
```

If no formatting args are passed after the desired target, the response body is grabbed. For example the following will return only the response body:

```sh
snipe shoot --target request-id-from-cfg
```

#### Tweaking response format

The `--format` argument allows for changing response output format. Currently there are two options `http` (default) and `json`. The `http` return the output as an `http` string in the format:

```sh
status_code
headers

body
```

Combos will follow a similar format. For example `snipe shoot --target request-id-from-cfg --status-code --body` will return:

```sh
status_code

body
```

While the combo `snipe shoot --target request-id-from-cfg --status-code --headers` will return:

```sh
status_code
headers
```

***Note - if the the response cannot be parsed into a string the snipe will emit an error.***

The `json` format will attempt to parse the response into a json string. The output is formatted something like:

```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
  "body": "<response_body>",
}
```

Similarly - combos can be used. For example `snipe shoot --target request-id-from-cfg --status-code --body --format json` will return:

```json
{
  "status_code": "<response_status_code>",
  "body": "<response_body>",
}
```

While the combo `snipe shoot --target request-id-from-cfg --status-code --headers --format json` will return:

```json
{
  "status_code": "<response_status_code>",
  "headers": "<response_headers>",
}
```

***Note - if the the response cannot be parsed into a json string snipe will emit an error.***

Parsing to json also has a `--pretty` flag which can be used to make the output more readable. `--pretty` is not valid with `--format http` and if this combination is passed snipe will emit an error. An examples of using `--pretty` are shown below:

```sh
# Valid args
snipe shoot --target request-id-from-cfg --full --format json --pretty

# Invalid args
snipe shoot --target request-id-from-cfg --full --format http --pretty # ERROR! => Can't pass --format http with --pretty
snipe shoot --target request-id-from-cfg --full --pretty # ERROR! => By default snipe uses --format http which is invalid with --pretty
```

`snipe` can output a response to a file using the `--outputfile` argument. An example of writing a full response in pretty printed json to a file is shown below:

```sh
snipe shoot --target request-id-from-cfg --full --format json --pretty --output-file full-response.json
```

If the response body is binary, `snipe` can handle this situation using the `--format binary --output_file <OUTPUT_FILE>` args. The `--format binary` arg must be used with the `--output_file` flag and is only valid with the `--body` flag. An example for grabbing a zip file from a response body is shown below.

```sh
# Note --body is passed by default
snipe shoot --target request-id-from-cfg --format binary --output-file some_file.zip
```

### Uploading a File as the Response Body

`snipe` allows for appending a file as a response body. For example, using the `create-gist` target from the quickstart, the body could be stored in a `json` file that the snipe configuration file could be updated to look for. For example the `json` file could look like:

```json
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
files = {"test.txt" = {content = "Testing Gist"}}
```

### Changing the Path to the Configuration File

Use the the `--config` (`-c`) argument to change the path of the configuration file. As stated above by default `snipe` will look for a `.snipe_targets.toml` file in your present working directory. An example of using a different config looks something like the following:

```sh
snipe --config ~/.config/snipe/snipe_targets.toml shoot --target request-id-from-cfg
```

If the config cannot be found, snipe will fall back to attempting to read the `SNIPE_TARGETS` environment variable which houses the path to the config. The environment variable to look for can be tweaked with the `--cfg-env` (`-e`) arg. Searching for the environment variable can be skipped entirely by passing `skip` for this argument. For example:

```sh
snipe --cfg-env SNIPE_CONFIG shoot --target request-id-from-cfg  # If can't find the config at ./.snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --config ~/.config/snipe/snipe_targets.toml --cfg-env SNIPE_CONFIG shoot --target request-id-from-cfg  # If can't find the config at ~/.config/snipe/snipe_targets.toml use the value at the environment variable SNIPE_CONFIG
snipe --cfg-env skip shoot --target request-id-from-cfg  # Skip an environment variable to identify the config
```

### Verbosity

Use `--verbose` (`-v`) for verbose mode. This mode will set the log level to info. Use `-vv` to set the log level to debug. Note, any number of `v` arguments can be passed. However, two or more arguments will simply set the log level to debug. For example:

```sh
snipe --verbose shoot --target request-id-from-cfg  # Log level at info
snipe -v shoot --target request-id-from-cfg  # Log level at info
snipe -vv shoot --target request-id-from-cfg  # Log level at debug
snipe -vvv shoot --target request-id-from-cfg  # Log level at debug
snipe -vvvvv shoot --target request-id-from-cfg  # Log level at debug
```
