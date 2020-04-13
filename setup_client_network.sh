#!/bin/sh
ip netns add anno_c
ip link add veth_anno_c type veth peer name veth0 netns anno_c
ip netns exec anno_c ip link set dev veth0 up
ip link set dev veth_anno_c up
ip addr add 10.20.0.1/24 dev veth_anno_c
ip netns exec anno_c ip addr add 10.20.0.2/24 dev veth0

ip netns add anno_h
ip link add veth_anno_h type veth peer name veth0 netns anno_h
ip netns exec anno_h ip link set dev veth0 up
ip link set dev veth_anno_h up
ip addr add 10.30.0.1/24 dev veth_anno_h
ip netns exec anno_h ip addr add 10.30.0.2/24 dev veth0
