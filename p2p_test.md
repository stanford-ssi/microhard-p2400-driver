# Point to Point Testing

Put radio 1 in command mode, and run the following:

```AT
AT&F6
ATS109=? # query register
ATS109=61
ATS109=? # query register
AT&W
```

Put radio 2 in command mode, and run the following:

```AT
AT&F7
# ATS109=61
AT&W
```
