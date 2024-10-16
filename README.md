<h1 align="center"> House Simulator Rust</h1>

<div align="center">
  
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) 
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)

</div>

## Description

This is a not so idiomatic Rust implementation of a simulator designed to be implemented in C++.

The simulator includes home areas, which have properties such as temperature, light, etc., which are inspected by sensors, that provide the readings to rules that are managed by processors, that determine whether to send a new command to a device or not according to those rules.

The simulator includes passage of time, which is completely independent of real-time measured by the computer.

## 🛠️ Pre-Requisites

Before you attempt to clone this project make sure you have the following pre-requisites:

1. [Git](https://git-scm.com/downloads)
2. [Rust](https://www.rust-lang.org/tools/install)

## 🚀 Build the Project

1️⃣ **Clone the Project**

First you'll need to find a directory where you'll want to store the project.

Open the command line and navigate into that directory and type the following:

```bash
git clone https://github.com/PixelP4radise/house_simulator_rust.git
```

2️⃣ **Build and Run the Project**

1. Navigate into the project directory using:

```bash
cd ./house_simulator_rust
```

2. And build the project with:

```bash
cargo build --release
```

3. After building the project you're able to run it with:

```bash
cargo run --release
```

If you didn't build the project, in 2. cargo will build it for you when you try to run it.

## ⌨️ Usage

This project is comprised of 3 screens:

- Welcome Screen
- Simulation Screen
- Exit Screen

### 📥 Welcome Screen

The Welcome Screen is the first screen shown to you when you run the executable.

While on the Welcome Screen:

- If you press Enter, you'll be sent to the Simulation Screen
- If you press Q or q you'll be sent to the Exit Screen

### 📈Simulation Screen

The Simulation Screen is the screen where you'll be presented with the information about the ongoing simulation

While on the Simulation Screen:

- If you press Enter, you'll enter editing mode where you'll be able to type down a command
- If you press Enter after typing in a command, you'll launch the command execution
- If you press either Q or q while not in editing mode, you'll be sent to the Exit Screen

### 📤Exit Screen

The Exit Screen is a screen to make sure you're wanting to leave the app

While on the Exit Screen:

- If you press Enter, you'll be sent back to the Simulation Screen
- If you press Q or q, you'll leave the app and it will be closed

## Commands

### Commands for simulated time

- `next`

  - Advances one instant on the house being simulated, triggering the effect of components that act on the passage of time

- `advance <n>`
  - Advances n instants, one at a time, in each one all components are triggered like on the `next` command

### Commands to manage home and zones

- `hnew <num_rows> <num_columns>`
  - Creates a new Home witht the indicated size. This command must be the first to be executed in a simulation. When creating a home, the existing one will be destroyed as well as everything associated with it
- `hrem`
  - Removes the home
- `znew <row> <column>`
  - Creates a new room in the home at the indicated row and column. The indicated position must be empty
- `zrem <zone_id>`
  - Deletes the zone with the indicated Id and all it's contents
- `zlist`
  - List all zones on the house. For each zone, it displays the ID and number of sensors, the number of processors, and the number of devices in it.

### Commands to manage zones and their content

- `zcomp <zone_id>`
  - Lists the components existing in the zone with the indicated ID.
- `zprops <zone_id>`
  - Lists the properties recognized by the simulator with the Indicated id. Each property is described by its name and its current value
- `pmod <zone_id> <name> <value>`
  - Modifies the value of the property whose name is given in the zone with the given ID to the specified value.
  - the property name is specified with first letter in uppercase
- `cnew <zone_id> <s | p | d> <type | command>`
  - Adds a sensors/processor/device of the indicated type to the zone that has the indicated id. "type" refers to the type of sensor or device. In the case of processor since there's only one type of processor, the last argument specifies the command that must be sent to the associated devices when it's rules associate to true.
  - types of sensors or devices are specified in lower case
- `crem <zone_id> <s | p | d> <id>`
  - Removes the component with the given id from the specified zone. It can't be removed as long as it's being referenced by another component

### Commands for rule processors

- `rnew <zone_id> <rules_processor_id> <rule> <sensor_id> [param1] [param2] [...]`
  - Create a new rule on the processor with the specified id. The rule is associated to the value received from the sensor identified by the sensor_id
- `pchange <zone_id> <rules_processor_id> <new_command>`
  - Changes the command to be sent in the indicated processor, at the specified room, command is one word.
- `rlist <zone_id> <rules_processor_ID>`
  - List the rules of the rules processor, one per line, indicating their name, their ID and the name and ID of the associated sensor
- `rrem <zone_id> <rules_processor_id>`
  - Removes indicated rule from the specified rule processor
- `asoc <zone_id> <rules_processor_id> <device_id>`
  - Estabilishes the association between the output of the indicated processor and the specified device
- `disa <zone_id> <rules_processor_id> <device_id>`
  - Removes the association between the output of the indicated processor and the specified device

### Commands to interact with devices

- `dcom <zone_id> <device_id> <command>`
  - Sends the indicated command to the device whose ID was specified

### Commands for copying/retrieving rule processors

- `psave <zone_id> <rule_processor_id> <name>`
  - Saves the rule processor and everything associated to it to memory. The name must be unique
- `prestore <name>`
  - Restores the rule processor saved in memory to it's previous room, if it still exists. If there's a processor with the same id still in the room, the memory one will overwrite it's information.
- `prem <name>`
  - Removes the processor from memory
- `plist`
  - Lists the processors in memory. This list includes the name, processor Id and the Id from where it came from

### Additional general Commands

- `exit`
  - Cleanly exits the program

## Properties

| Properties  |      Unit      | Minimum | Maximum |
| :---------: | :------------: | :-----: | :-----: |
| Temperature |    Celsius     |  -273   |    -    |
|    Light    |     Lumens     |    0    |    -    |
|  Radiation  |   Becquerel    |    0    |    -    |
|  Vibration  |     Hertz      |    0    |    -    |
|  Humidity   |       %        |    0    |   100   |
|    Smoke    | Obscuration(%) |    0    |   100   |
|    Sound    |    Decibels    |    0    |    -    |

## Sensors

|   Sensor    | Character(for viewing) | Observed property |
| :---------: | :--------------------: | :---------------: |
| temperature |           t            |    Temperature    |
|  movement   |           v            |     Vibration     |
| luminosity  |           m            |       Light       |
|  radiation  |           d            |     Radiation     |
|  humidity   |           h            |     Humidity      |
|    sound    |           o            |       Sound       |
|    smoke    |           s            |       Smoke       |
