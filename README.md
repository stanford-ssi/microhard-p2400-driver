# Microhard p2400 Driver

Connect over serial with a 9-pin VGA.

```bash
ls /dev/tty to find
screen /dev/tty.usbserial-1130 9600 cs8 -cstopb -parenb -ixoff
```

## Command mode

### Force Into Command Mode

0. disconnect 9-pin connector and reconnect
1. push button
2. power on
3. button off
4. send connection message
5. power off
6. push button
7. power on
8. button off

### From Data Mode

```serial
+++
```

## Data mode

### From Command Mode

```serial
ATA
```

## Print statistics

AT&V

---

## Tests

[Point to point testing](p2p_test.md)