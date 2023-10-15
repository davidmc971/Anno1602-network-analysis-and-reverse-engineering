This proxy interfaces between a host and clients of a multiplayer lobby in Anno 1602 (or potentially other DirectPlay (4) games).

Upon starting it will listen on:
- UDP on 47624
- TCP/UDP between 2300-2400

It will accept lobby clients and forward their requests to the host through its own connection.
Meanwhile packet data will be read and parsed.