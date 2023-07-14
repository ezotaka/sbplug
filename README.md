# sbplug

sbplug is a command line interface (CLI) tool that allows you to control your SwitchBot Plug devices. You can list, turn on/off, and check the status of your devices directly from the command line.

## Usage

Before you can use sbplug, you need to set the SwitchBot app token as an environment variable named `SWITCHBOT_TOKEN`. You can get this token from the SwitchBot app. For details on how to retrieve the token, please refer to the [SwitchBot API documentation](https://github.com/OpenWonderLabs/SwitchBotAPI/blob/main/README-v1.0.md).

After setting the environment variable, you can use the `sbplug` command followed by a subcommand. Use the `-h/--help` flag to print help information.

```shell
$ sbplug <SUBCOMMAND>
```

### Available Subcommands

- `help`: Prints help information for sbplug or its subcommands.
- `list`: Lists all the devices you have.
- `off`: Turns off a device.
- `on`: Turns on a device.
- `status`: Checks the status of a device.

## Examples

Here are some examples of how to use sbplug:

1. List all your devices using the `list` subcommand:

```shell
$ sbplug list
```

The output will show the device ID, device name, and power status:

```shell
1)	AAAAAAAAAAAAAA	Plug-Mini-AAAA	on
2)	BBBBBBBBBBBBBB	Plug-Mini-BBBB	on
```

2. Turn on a device with the `on` subcommand followed by the device ID:

```shell
$ sbplug on AAAAAAAAAAAAAA
```

or

```shell
$ sbplug on 1
```

The output will show `success` if the operation was successful.

3. Turn off a device with the `off` subcommand followed by the device ID:

```shell
$ sbplug off BBBBBBBBBBBBBB
```

Again, the output will show `success` if the operation was successful.

4. Check the status of a device with the `status` subcommand followed by the device ID:

```shell
$ sbplug status AAAAAAAAAAAAAA
```

The output will display detailed information about the device:

```shell
DeviceId: AAAAAAAAAAAAAA
DeviceType: Plug Mini (JP)
ElectricCurrent: 0
ElectricityOfDay: 50
HubDeviceId: CCCCCCCCCCCCCC
Power: on
Voltage: 100.3
Weight: 0
```

Remember, you can use either the device ID or the device's index number from the `list` subcommand when using the `on`, `off`, and `status` subcommands.
