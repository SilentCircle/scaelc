Silent Circle Audit Event Log Collector
=======================================

SCAELC is a tool that will download your enterprise's logs from the
Silent Circle servers and store them locally, so you can import them
into your favorite log aggregator.


Installation
------------

There is no installation for SCAELC, just download the binary, place
it into some convenient place, preferably on your PATH.


Running
-------

To run SCAELC, simply supply it with your API key. If you don't have an
API key, please contact your Silent Circle liaison to obtain one.

```
scaelc someapikey
```

It will download and display your log items on the command line. To
write the items to a file, use the `-o` flag:

```
scaelc -o production.log someapikey
```

SCAELC supports outputting data as one big JSON list or as individual
lines with one JSON event per line. The default is one JSON object per
line, but you can get the JSON list format by passing the `-e` flag.


Requesting items by date
------------------------

SCAELC supports querying for logs before or after a specific date. This
is meant to be used for fetching log items earlier or later than a
previously-fetched log. You can simply specify that you only want to
fetch items after the last log item you fetched previously, and SCAELC
will oblige.

There are four such flags:

| Flag          | Operator |
|---------------|----------|
| `-s --since`  |       >= |
| `-a --after`  |        > |
| `-u --until`  |       <= |
| `-b --before` |        < |

Passing each flag with a given `DATE` will fetch the events that
correspond to that operator. For example, to fetch all events after
(and not including) any events exactly on midnight of 2018-01-01:

```
scaelc -a 2018-01-01T00:00:00Z
```

All dates are in UTC.
