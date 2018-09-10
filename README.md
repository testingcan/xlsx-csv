# XLSX-CSV

Basic XLSX to CSV converter.

## Usage
```
XLSX-CSV 0.2.3
Converts XLSX-files to CSV

USAGE:
    xlsx-csv.exe [FLAGS] [OPTIONS]

FLAGS:
    -c, --crlf       Set CRLF-terminator
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show verbose debug output

OPTIONS:
    -d <delimiter>        Set the delimiter for the CSV-output [default: b',']
    -i <input>            Sets the input file or directory [default: .]
    -o <output>           Output directory [default: .]
    -s <sheet>...         Specifiy the sheet(s) to convert
```
The script can either be started by providing the input (and optionally output)-files or
by providing a (set of) config-files. For use of config-files see below. By default the
script reads the current directory for XLSX-files to convert.


## Configuration
The script comes with a default template configuration. The config files must reside in the 
same folder as the script.

Each configuration file consists of four parameters. 
* `debug`: bool
* `delimiter`
* `crlf`: bool
* `source`: with the parameter `path`
* `archive`: with the parameter `path`


In order to deploy the script to an environment, an environment variable has to be set directly on the server,
since the respective configuration file is being loaded conditionally. If not present already, create an
environment variable in the desired environment, `RUN_MODE` and assign the respective value 
(e.g. `integration` for config file `config/integration.toml`).
