# ZTransfer
A program that makes use of websockets to provide a real-time interface for simple LAN file transfer. Not intended for public facing servers, as this assumes that all users on your network are trusted individuals. *Currently very limited.*

## Use Cases
- Transfer files between different computers on a home network without the need of a flash drive or some other hardware.
- Provide a transfer mechanism between a host machine and a guest machine in case you're too lazy to setup shared folders with a VM.

# "Documentation"

## Server-To-Client Messages
*Files with the same name will replace each other.*
```json
{
	"action": "ADD_FILE",
	"filename": "test.txt",
	"size": 86000
}
{
	"action": "REMOVE_FILE",
	"filename": "test.txt"
}
{
	"action": "UPLOAD_PROGRESS",
	"filename": "test.txt",
	"progress": 5,
	"total": 10
}
```

## Client-To-Server Messages
```json
{
	"action": "SEND_DELETE_REQUEST",
	"filename": "test.txt"
}
```
