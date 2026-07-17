import struct
import config
#dplay_package_size = struct.unpack("<H", data_client[0:2])[0]
#dplay_s_addr_in_ip_address = struct.unpack("<BBBB", data_client[8:12])
#print (dplay_s_addr_in_ip_address)

def isDplay(data):
    if(len(data) >= 24):
        size = struct.unpack("<H", data[0:2])[0]
        dplay_id = struct.unpack("<2c", data[2:4])
        dplay_id = "".join([dplay_id[1].hex(), dplay_id[0].hex()])
        action = struct.unpack("<cccc", data[20:24])
        action = "".join([action[0].decode("ASCII"), action[1].decode("ASCII"), action[2].decode("ASCII"), action[3].decode("ASCII")])
        if dplay_id == "fab0" and action == "play":
            return True
        if config.VERBOSE_LOGGING:
            print("size: {}, dplay_id: 0x{}, action: {}".format(size, dplay_id, action))
    return False

def parse(data, port, origin):
    if(isDplay(data)):
        return
    #data = bytearray(data)
    packet_length = len(data)
    id0 = data[0:4].hex()
    id1 = data[4:8].hex()
    action = struct.unpack("2H", data[8:12])[0]
    action = hex(action)
    # size more than 2 bytes? --- anno_length is packet_length - 8 because ids are excluded
    anno_length = struct.unpack("2H", data[12:16])[0]
    #anno_length = hex(anno_length)
    data = data[16:packet_length]
    remainder = data.hex()

# 03000000 0c000000 35000000 14121600 20020000 39ff00004a0000001000000039ff00004a0000000a000000

    if anno_length == 52 or anno_length == 28:
        i = 16
        print ("[{},{}]\t len: {} id0: {} id1: {} action: {} anno_length: {} remaining: \n{}".format(
        origin, port, packet_length, id0, id1, action, anno_length, remainder))
        amount = struct.unpack("<H",data[i:i+2])[0]
        print ("Amount: {} {} {}".format(amount, hex(amount), bin(amount)))
        return
    else:
        pass
        #return
        # remaining 2-byte blocks
        # remaining_blocks = int((packet_length - 16) / 2)

        # remainder = []

        # for i in range(remaining_blocks):
        #     j = i*2
        #     k = struct.unpack("<H", data[j:j+2])[0]
        #     remainder.append((i, "{}{}".format("0x", str(data[j:j+2].hex())), k))

    if anno_length == 124:
        pass
        #return
    print ("[{},{}]\t len: {} id0: {} id1: {} action: {} anno_length: {} remaining: \n{}".format(
        origin, port, packet_length, id0, id1, action, anno_length, remainder))


# 0x84f, 0x835 when building
# 0x84f on wood into island_inventory
# 0x848, 0x84f on try ship unload into island_inventory despite no items being transferred
# 0x849, 0x848, 0x84f on ship unload into island_inventory with items being transferred
# 0x84f on adjust tax (lowest to highest in steps)
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 44000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 48000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 4c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 50000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 54000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 58000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 5c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 60000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 64000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 68000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 6c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 70000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 74000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 78000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 7c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 80000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 84000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 88000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 8c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 90000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 94000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 98000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 9c000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 a0000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 a4000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 a8000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 ac000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 b0000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 b4000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 b8000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 bc000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 c0000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 c0000000
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining: 01000000 0c000000 35000000 31330000 c0000000

# 0x84f on money increase (amount at end?)
    # [client,2350]    len: 132 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 124 remaining: 09000000 0c000000 35000000 14120e00 be030000 35000000 31120000 12000000 35000000 31110000 80000000 35000000 31120100 00000000 35000000 31120200 00000000 35000000 31120300 00000000 35000000 31120400 00000000 35000000 32110000 80000000 39ff0000 2c000000 51240000
    # [client,2350]    len: 132 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 124 remaining: 09000000 0c000000 35000000 14120e00 bd030000 35000000 31120000 12000000 35000000 31110000 80000000 35000000 31120100 00000000 35000000 31120200 00000000 35000000 31120300 00000000 35000000 31120400 00000000 35000000 32110000 80000000 39ff0000 2c000000 52240000

# wood into island_inventory:
    # [client,2350]    len: 36 id0: 9ba49b01 id1: 9ea49b01 action: 0x84f anno_length: 28 remaining:
    # 01000000 0c000000 35000000 14121700 20000000    1
    # 01000000 0c000000 35000000 14121700 40000000    2
    # 01000000 0c000000 35000000 14121700 60000000    3
    # 01000000 0c000000 35000000 14121700 80000000    4
    # 01000000 0c000000 35000000 14121700 a0000000    5
    # 01000000 0c000000 35000000 14121700 c0000000    6
    # 01000000 0c000000 35000000 14121700 e0000000    7
    # 01000000 0c000000 35000000 14121700 00010000    8
    # 01000000 0c000000 35000000 14121700 20010000    9
    # 01000000 0c000000 35000000 14121700 40010000    10
    # 01000000 0c000000 35000000 14121700 60010000    11
    # 01000000 0c000000 35000000 14121700 80010000    12
    # 01000000 0c000000 35000000 14121700 a0010000    13
    # 01000000 0c000000 35000000 14121700 c0010000    14
    # 01000000 0c000000 35000000 14121700 e0010000    15
    # 01000000 0c000000 35000000 14121700 00020000    16
    # 01000000 0c000000 35000000 14121700 20020000    17
    # 01000000 0c000000 35000000 14121700 40020000    18
    # 01000000 0c000000 35000000 14121700 60020000    19
    # 01000000 0c000000 35000000 14121700 80020000    20
    # 01000000 0c000000 35000000 14121700 a0020000    21
    # 01000000 0c000000 35000000 14121700 c0020000    22
    # 01000000 0c000000 35000000 14121700 e0020000    23
    # 01000000 0c000000 35000000 14121700 00030000    24
    # 01000000 0c000000 35000000 14121700 20030000    25
    # 01000000 0c000000 35000000 14121700 40030000    26
    # 01000000 0c000000 35000000 14121700 60030000    27
    # 01000000 0c000000 35000000 14121700 80030000    28
    # 01000000 0c000000 35000000 14121700 a0030000    29
    # 01000000 0c000000 35000000 14121700 e0030000    31
    # 01000000 0c000000 35000000 14121700 00040000    32

#food: 010000000c0000003500000014120e000c010000