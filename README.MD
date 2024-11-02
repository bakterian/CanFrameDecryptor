# Can Frame Decryptor

## Small rust program for translating CAN byte-stream to spec-defined signal values
The program is useful when intercepting CAN data with simple CAN-Analyzer/Sniffer.  
More advanced CAN-analyzer solutions (i.e. CANoe) consume a communication matrix and  
do translate the communication on the fly. The more basic tools like PCan analyzer  
with their software don't contain this functionality.  
Even the sophisticated CANoe tools may not be able to support all of the customer  
defined message definition catalogues and would require a translation to a well known  
communication matrix format.  
The same applies for this tool as it requires a specific CAN frame definition format  
enclosed in a json file. A simple example k_matrix.json file can be found in the src  
folder.  
 
## Prerequisites for usage
- git
- rust environment

## How to build the program
- checkout the source from the remote repository
- go to the root folder of the checked out sources
- for building a debug version type in cmd: cargo build
- for building a release version type in cmd: cargo build --release

## Usage
The user provides:  
- The message definition file (i.e src/k_matrix.json) which should be placed in 
the same directory as the build program (needs to be copied manually after build)

- A CAN frame as program input argument represented as a collection of bytes in the following format:  
```sh
<CAN FRAME ID as 4byte hex> <DLC single byte> <multiple byte payload>
```

## Example program execution
```sh
cargo run -- "0xAAAAAAAA 8 0x00 0x07 0x00 0x00 0xC8 0xE0 0x01 0x00"
decypring frame with ID: 0xAAAAAAAA
ControlSomeDisp_X_control_surround : 0
ControlSomeDisp_X_fokus_zone : Rear
ControlSomeDisp_X_contr_iClima_av : Init
ControlSomeDisp_X_control_fct : Init
ControlSomeDisp_X_control_iClima : Init
ControlSomeDisp_X_control_MapMode : Init
ControlSomeDisp_X_control_MapZoom : Init
ControlSomeDisp_X_control_sound : Audio
ControlSomeDisp_X_control_vol : 6
ControlSomeDisp_X_control_vol_max : 30
ControlSomeDisp_X_control_vol_min : 0
ControlSomeDisp_X_control_Point : Init
ControlSomeDisp_X_control_Point : Init