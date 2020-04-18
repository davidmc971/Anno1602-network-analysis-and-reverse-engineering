Basic event structure:
===
[id1][id2][action][size]
[data..................]
id1, id2 differ only at the integer's end and are also sligtly
different from DPLAY ids example:
id1, id2 client to host and host to client case 1:
	da b3 d5 19 df b3 d5 19
	dc b3 d5 19 df b3 d5 19
	dplay id client to host and host to client:
	ddb3d519
	deb3d519
id1, id2 client to host and host to client case 2:
	97 c3 0f 1a 92 c3 0f 1a
	91 c3 0f 1a 92 c3 0f 1a
	dplay id client to host and host to client:
	90c30f1a
	93c30f1a

FOCUS/DEFOCUS:
	0x07D2 for FOCUS
	0x07D1 for DEFOCUS
	STRUCTURE:
	[id1][id2][action][size]
	- always 4 int32 long, no additional data
	if a players game loses or gains focus, this event is sent

PAUSE/UNPAUSE:
	0x084F for both
	[id1][id2][action][size]
	[1 if pause, 0 if unpause]

MOVE / SHIP ACTION:
	0x0836
	STRUCTURE:
	[id1]:32 		[id2]:32 		[action]:32 		[size]:32
	[?]:32 			[size-16]:32 	[position?]:32 		[?]:16		[ship id]:16
	[0x3700] 		[dest?]:16 		[? + stream of moves]:variable
	NOTES:
	ship move (re-)sent on direction change
	size apparently excl. ids
	
SINK SHIP:

SELL SHIP / SHIP PRICE:
	0x084f
	STRUCTURE:
	[id1]:32 		[id2]:32 		[action]:32 	[size]:32
	[?]:32 			[?]:32 			[0x01ff] 		[ship id]:16 	[?]:32
	[?]:32 			[0x01ff] 		[ship id]:16 	[?]:32 			[price]:16 		[0x0000]
	
	price is 0x0000 - 0x0180; in dec: 0 - 384
	and in game is translated as (max ship price / 384) * this value.
	you can sell others ships by manipulating ship id